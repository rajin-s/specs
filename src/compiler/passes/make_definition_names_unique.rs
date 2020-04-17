use super::utilities::TempNameGenerator;
use crate::language::nodes::*;
use std::collections::HashMap;

pub fn apply(root_node: &mut Node)
{
    let mut environment = Environment::new();
    root_node.recur_transformation(make_function_names_unique, &mut environment);
}

fn make_function_names_unique(node: &mut Node, environment: &mut Environment)
{
    match node
    {
        Node::Variable(data) =>
        {
            if let Some(new_name) = environment.get_override(data.get_name())
            {
                data.set_name(new_name.clone());
            }
        }
        Node::Function(data) =>
        {
            let new_name = environment.get_scoped_name(data.get_name());
            
            environment.push_scope(data.get_name().clone(), Context::FunctionBody);
            {
                data.recur_transformation(make_function_names_unique, environment);
            }
            environment.pop_scope();
            
            data.set_name(new_name);
        }
        Node::Binding(data) =>
        {
            data.recur_transformation(make_function_names_unique, environment);

            // Remove overrides so they don't affect nodes after a binding with the same name
            if let Some(_new_name) = environment.get_override(data.get_name())
            {
                environment.remove_override(data.get_name());
            }
        }
        Node::Sequence(data) =>
        {
            let mut overrides: Vec<String> = Vec::new();

            // Push an anonymous scope if this sequence isn't a function body and contains some definition
            let added_anonymous_scope = if environment.get_current_context() == Context::Generic
            {
                let mut has_definition = false;
                for node in data.get_nodes().iter()
                {
                    if let Node::Function(_) = node
                    {
                        has_definition = true;
                        break;
                    }
                }

                if has_definition
                {
                    environment.push_anonymous_scope();
                    true
                }
                else
                {
                    false
                }
            }
            else
            {
                false
            };

            // Collect definitions in the sequence to preserve order-independence
            for node in data.get_nodes().iter()
            {
                match node
                {
                    Node::Function(function_data) =>
                    {
                        environment.add_override(function_data.get_name());
                        overrides.push(function_data.get_name().clone());
                    }
                    _ =>
                    {}
                }
            }

            data.recur_transformation(make_function_names_unique, environment);

            if added_anonymous_scope
            {
                environment.pop_scope();
            }

            // Restore overrides to what they were before this sequence
            for name in overrides.iter()
            {
                environment.remove_override(name);
            }
        }
        node =>
        {
            node.recur_transformation(make_function_names_unique, environment);
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Context
{
    FunctionBody,
    Generic,
}

struct Environment
{
    scopes:          Vec<(String, Context)>,
    overrides:       HashMap<String, String>,
    anonymous_names: TempNameGenerator,
}
impl Environment
{
    pub fn new() -> Self
    {
        return Self {
            scopes:          Vec::new(),
            overrides:       HashMap::new(),
            anonymous_names: TempNameGenerator::new("scope"),
        };
    }

    pub fn push_scope(&mut self, name: String, context: Context)
    {
        self.scopes.push((name, context));
    }
    pub fn push_anonymous_scope(&mut self)
    {
        let name = self.anonymous_names.next();
        self.push_scope(name, Context::Generic);
    }
    pub fn pop_scope(&mut self)
    {
        self.scopes.pop();
    }

    pub fn get_scoped_name(&self, name: &String) -> String
    {
        let mut new_name = String::new();
        for (scope, _) in self.scopes.iter()
        {
            new_name = format!("{}{}/", new_name, scope);
        }
        new_name = format!("{}{}", new_name, name);
        if self.scopes.is_empty()
        {
            return new_name;
        }
        else
        {
            return format!("_{}", new_name);
        }
    }

    pub fn add_override(&mut self, name: &String)
    {
        let scoped_name = self.get_scoped_name(name);
        self.overrides.insert(name.clone(), scoped_name);
    }
    pub fn remove_override(&mut self, name: &String)
    {
        self.overrides.remove(name);
    }
    pub fn get_override(&self, name: &String) -> Option<&String>
    {
        return self.overrides.get(name);
    }

    pub fn get_current_context(&self) -> Context
    {
        match self.scopes.last()
        {
            Some((_, context)) => *context,
            None => Context::Generic,
        }
    }
}
