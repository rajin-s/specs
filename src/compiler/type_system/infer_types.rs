use crate::language::nodes::*;
use crate::language::symbols;
use std::collections::HashMap;

pub fn apply(root: &mut Node)
{
    build_definition_types(root, &mut ());

    let mut root_type_environment = TypeEnvironment::new();
    infer_type(root, &mut root_type_environment);
}

fn infer_type(node: &mut Node, environment: &mut TypeEnvironment)
{
    match node
    {
        Node::Nothing | Node::Integer(_) | Node::Boolean(_) =>
        {}
        Node::Variable(data) =>
        {
            match environment.get_type_of(data.get_name())
            {
                Some(symbol_type) =>
                {
                    data.set_type(symbol_type.clone());
                }
                None =>
                {
                    // Failed to find the type of this symbol
                }
            }
        }

        Node::Call(data) =>
        {
            // Try to infer the types of operator and operands first
            data.recur_transformation(infer_type, environment);

            // Handle primitive operators whose types depend on operand types
            let (operator, operands, _) = data.get_all_mut();
            if let Node::PrimitiveOperator(operator_data) = operator
            {
                match operator_data.get_operator()
                {
                    // compare : (T T) -> bool
                    PrimitiveOperator::Equal
                    | PrimitiveOperator::NotEqual
                    | PrimitiveOperator::Less
                    | PrimitiveOperator::Greater
                    | PrimitiveOperator::LessEqual
                    | PrimitiveOperator::GreaterEqual =>
                    {
                        if operands.len() >= 2
                        {
                            let a_type = operands[0].get_type();

                            let operator_type = Type::from(FunctionTypeData::new(
                                // Arguments
                                vec![a_type.clone(), a_type.clone()],
                                // Return
                                basic_types::boolean().clone(),
                                // Metadata
                                FunctionMetadata::new_basic(),
                            ));

                            operator_data.set_type(operator_type);
                        }
                    }
                    // create : T -> (instance T)
                    PrimitiveOperator::Create =>
                    {
                        if operands.len() == 1
                        {
                            let a_type = operands[0].get_type();

                            if let DataType::Type(data) = a_type.get_data_type()
                            {
                                let operator_type = Type::from(FunctionTypeData::new(
                                    // Arguments
                                    vec![a_type.clone()],
                                    // Return
                                    data.get_instance_type(),
                                    // Metadata
                                    FunctionMetadata::new_basic(),
                                ));

                                operator_data.set_type(operator_type);
                            }
                        }
                    }

                    // Other operators don't depend on operands
                    // note: operator type should have already been found
                    _ =>
                    {}
                }
            }

            // Then infer the type of the result from the operator
            let operator_type = data.get_operator().get_type();
            if let DataType::Function(operator_type_data) = operator_type.get_data_type()
            {
                let return_type = operator_type_data.get_return_type().clone();
                data.set_type(return_type);
            }
        }
        Node::PrimitiveOperator(data) => match data.get_operator()
        {
            // + : (int int) -> int
            PrimitiveOperator::Add =>
            {
                let operator_type = Type::from(FunctionTypeData::new(
                    // Arguments
                    vec![
                        basic_types::integer().clone(),
                        basic_types::integer().clone(),
                    ],
                    // Return
                    basic_types::integer().clone(),
                    // Metadata
                    FunctionMetadata::new_basic(),
                ));

                data.set_type(operator_type);
            }
            // ~ : (bool bool) -> bool
            PrimitiveOperator::And | PrimitiveOperator::Or | PrimitiveOperator::ExclusiveOr =>
            {
                let operator_type = Type::from(FunctionTypeData::new(
                    // Arguments
                    vec![
                        basic_types::boolean().clone(),
                        basic_types::boolean().clone(),
                    ],
                    // Return
                    basic_types::boolean().clone(),
                    // Metadata
                    FunctionMetadata::new_basic(),
                ));

                data.set_type(operator_type);
            }
            _ =>
            {
                // Any other operators depend on the operands, so their types need to be inferred at the call node
            }
        },

        Node::Reference(data) =>
        {
            // (ref v:T) : (ref T)

            // Try to infer the type of the target first
            data.recur_transformation(infer_type, environment);

            // The result is a reference to the original type
            let target_type = data.get_target().get_type().clone();
            let result_type = target_type.make_reference(data.get_reference_type());

            data.set_type(result_type);
        }
        Node::Dereference(data) =>
        {
            // (deref v:&T) : T

            // Try to infer the type of the target first
            data.recur_transformation(infer_type, environment);

            // The result is dereferences the original type if possible
            let target_type = data.get_target().get_type().clone();
            if let Some(result_type) = target_type.make_dereference()
            {
                data.set_type(result_type);
            }
        }

        Node::Binding(data) =>
        {
            // Infer the type of the binding
            data.recur_transformation(infer_type, environment);

            let binding_type = data.get_binding().get_type().clone();

            // Track the symbol type
            environment.add_binding(data.get_name().clone(), binding_type.clone());

            // Keep track of the binding type
            // note: the binding node might change during compilation, and we don't want to lose type information
            data.set_binding_type(binding_type);
        }
        Node::Assignment(_) =>
        {
            node.recur_transformation(infer_type, environment);
        }

        Node::Sequence(data) =>
        {
            if data.is_transparent()
            {
                // Transparent sequences don't introduce a new scope
                // note: transparent sequences are only used internally,
                //       so this doesn't matter for type-checking user programs

                // First, collect definitions from within the body of the sequence
                collect_definition_types(data.get_nodes(), environment);

                // Type check the body of the sequence
                data.recur_transformation(infer_type, environment);
            }
            else
            {
                // Create a new local scope from the original environment
                let mut new_environment = environment.fork();

                // Collect definitions from within the body of the sequence
                collect_definition_types(data.get_nodes(), &mut new_environment);

                // Infer the type of each child node in the new environment
                data.recur_transformation(infer_type, &mut new_environment);
            }

            // The type of a sequence...
            //  - with only definitions is the type of the last definition
            //  - with any non-definition nodes is the type of the last non-definition node
            let mut last_non_definition_node: Option<&Node> = None;
            for node in data.get_nodes().iter()
            {
                if !node.is_definition()
                {
                    last_non_definition_node = Some(node);
                }
            }

            if let Some(node) = last_non_definition_node
            {
                let node_type = node.get_type().clone();
                data.set_type(node_type);
            }
            else
            {
                if let Some(node) = data.get_nodes().last()
                {
                    let node_type = node.get_type().clone();
                    data.set_type(node_type);
                }
                else
                {
                    data.set_type(basic_types::void().clone());
                }
            }
        }
        Node::Conditional(data) =>
        {
            // Infer types in the condition and branches
            data.recur_transformation(infer_type, environment);
        }

        Node::Function(data) =>
        {
            // Create a new local scope from the original environment
            // note: we ignore any bindings, since they can't be used inside the body of the new function
            let mut new_environment = environment.fork_definitions_only();

            // Add bindings for each of the function arguments
            for argument in data.get_arguments().iter()
            {
                new_environment
                    .add_binding(argument.get_name().clone(), argument.get_type().clone());
            }

            // Infer the type of the body using the new environment
            data.recur_transformation(infer_type, &mut new_environment);
        }

        Node::Type(data) =>
        {
            let instance_type = data.get_instance_type().make_reference(Reference::Mutable);

            for method in data.get_methods_mut().iter_mut()
            {
                // Create a new local scope from the original environment
                // note: we ignore any bindings, since they can't be used inside the body of the new function
                let mut new_environment = environment.fork_definitions_only();

                // Add bindings for each of the function arguments
                for argument in method.get_function_data().get_arguments().iter()
                {
                    new_environment
                        .add_binding(argument.get_name().clone(), argument.get_type().clone());
                }

                // Add binding for implicit self argument for instance methods
                if method.get_scope() == MemberScope::Instance
                {
                    new_environment
                        .add_binding(String::from(symbols::keywords::SELF), instance_type.clone());
                }

                // Infer the type of the body using the new environment
                method
                    .get_function_data_mut()
                    .recur_transformation(infer_type, &mut new_environment);
            }
        }
        Node::Access(data) =>
        {
            data.recur_transformation(infer_type, environment);

            let target_type = data.get_target().get_type();

            if target_type.is_value() || target_type.is_single_reference_layer()
            {
                // The target is a value or a reference to a value

                let (type_data, scope) = match data.get_target().get_type().get_data_type()
                {
                    DataType::Instance(instance_data) =>
                    {
                        if let Some(symbol_type) = environment.get_type_of(instance_data.get_name())
                        {
                            if let DataType::Type(type_data) = symbol_type.get_data_type()
                            {
                                // This is an instance of a bound type
                                (Some(type_data), MemberScope::Instance)
                            }
                            else
                            {
                                // This is an instance of something other than a type (invalid)
                                (None, MemberScope::Instance)
                            }
                        }
                        else
                        {
                            // This is an instance of an unbound type
                            (None, MemberScope::Instance)
                        }
                    }
                    DataType::Type(type_data) =>
                    {
                        // This is a type (can access static members)
                        (Some(type_data), MemberScope::Static)
                    }
                    _ =>
                    {
                        // This is something else entirely
                        (None, MemberScope::Static)
                    }
                };
                match type_data
                {
                    Some(type_data) =>
                    {
                        let property_name = data.get_property();
                        if let Some(member_type) = type_data.get_member_type(property_name)
                        {
                            // The property is a member of the type
                            let is_valid_scope = match scope
                            {
                                MemberScope::Instance =>
                                {
                                    type_data.has_instance_member(property_name)
                                }
                                MemberScope::Static => type_data.has_static_member(property_name),
                            };
                            if is_valid_scope
                            {
                                // The property is a member of the type in the correct scope
                                let node_type = member_type.clone();
                                data.set_type(node_type);
                            }
                        }
                    }
                    None =>
                    {
                        // The property isn't a valid member name
                    }
                }
            }
        }
    }
}

// Build types for type definitions
// note: must be done before inferring all other types
fn build_definition_types(node: &mut Node, params: &mut ())
{
    match node
    {
        Node::Type(data) =>
        {
            let mut type_data = TypeTypeData::new_empty(data.get_name().clone());

            // Add all data members
            for member in data.get_members().iter()
            {
                type_data.add_member(
                    member.get_name().clone(),
                    member.get_type().clone(),
                    member.get_scope(),
                    member.get_read_visibility() == Visibility::Public,
                    member.get_write_visibility() == Visibility::Public,
                );
            }

            // Add all function members
            for method in data.get_methods().iter()
            {
                type_data.add_member(
                    method.get_function_data().get_name().clone(),
                    method.get_function_data().get_type().clone(),
                    method.get_scope(),
                    method.get_visibility() == Visibility::Public,
                    false,
                );
            }

            // Add all traits
            for trait_data in data.get_traits().iter()
            {
                // type_data.add_trait(trait_data.get_name().clone());
            }

            data.set_type(Type::from(type_data));
        }
        _ =>
        {}
    }

    node.recur_transformation(build_definition_types, params);
}

// Collect definitions from a sequence of nodes
// note: so definitions can be order-independent
fn collect_definition_types(nodes: &Vec<Node>, environment: &mut TypeEnvironment)
{
    for node in nodes.iter()
    {
        match node
        {
            Node::Function(data) =>
            {
                environment.add_definition(data.get_name().clone(), data.get_type().clone());
            }
            Node::Type(data) =>
            {
                environment.add_definition(data.get_name().clone(), data.get_type().clone());
            }
            _ =>
            {}
        }
    }
}

type SymbolTable = HashMap<String, Type>;
struct TypeEnvironment
{
    bindings:    SymbolTable,
    definitions: SymbolTable,
}
impl TypeEnvironment
{
    pub fn add_binding(&mut self, name: String, binding_type: Type)
    {
        let _original_entry = self.bindings.insert(name.clone(), binding_type.clone());
    }
    pub fn add_definition(&mut self, name: String, definition_type: Type)
    {
        let _original_entry = self
            .definitions
            .insert(name.clone(), definition_type.clone());
    }

    pub fn get_type_of(&self, name: &String) -> Option<&Type>
    {
        // Check bindings first
        match self.bindings.get(name)
        {
            Some(value) =>
            {
                return Some(value);
            }
            None =>
            {}
        }

        // Then check definitions
        return self.definitions.get(name);
    }

    pub fn new() -> Self
    {
        return Self {
            bindings:    SymbolTable::new(),
            definitions: SymbolTable::new(),
        };
    }

    // Duplicate this environment so inner-scopes don't leak bindings/definitions
    pub fn fork(&self) -> TypeEnvironment
    {
        return TypeEnvironment {
            bindings:    self.bindings.clone(),
            definitions: self.definitions.clone(),
        };
    }

    // Duplicate this environment, but clear binding types (for function bodies)
    pub fn fork_definitions_only(&self) -> TypeEnvironment
    {
        return TypeEnvironment {
            bindings:    SymbolTable::new(),
            definitions: self.definitions.clone(),
        };
    }
}
