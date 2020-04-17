use crate::language::nodes::*;
use std::collections::HashMap;

pub fn apply(root: &mut Node)
{
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

                            let operator_type = Type::from(CallableTypeData::new(
                                vec![a_type.clone(), a_type.clone()],
                                Type::new(DataType::Boolean),
                            ));
                            operator_data.set_type(operator_type);
                        }
                    }
                    _ =>
                    {
                        // Other operators don't depend on operands
                        // note: operator type should have already been found
                    }
                }
            }

            // Then infer the type of the result from the operator
            let operator_type = data.get_operator().get_type();
            if let DataType::Callable(operator_type_data) = operator_type.get_data_type()
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
                let integer_type = Type::new(DataType::Integer);
                let operator_type = Type::from(CallableTypeData::new(
                    vec![integer_type.clone(), integer_type.clone()],
                    integer_type.clone(),
                ));

                data.set_type(operator_type);
            }
            // ~ : (bool bool) -> bool
            PrimitiveOperator::And | PrimitiveOperator::Or | PrimitiveOperator::ExclusiveOr =>
            {
                let boolean_type = Type::new(DataType::Boolean);
                let operator_type = Type::from(CallableTypeData::new(
                    vec![boolean_type.clone(), boolean_type.clone()],
                    boolean_type.clone(),
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
            environment.add_binding(data.get_name(), &binding_type);

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
                    data.set_type(Type::new(DataType::Void));
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
                new_environment.add_binding(argument.get_name(), argument.get_type());
            }

            // Infer the type of the body using the new environment
            data.recur_transformation(infer_type, &mut new_environment);
        }
    }
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
                environment.add_definition(data.get_name(), data.get_type());
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
    pub fn add_binding(&mut self, name: &String, binding_type: &Type)
    {
        let _original_entry = self.bindings.insert(name.clone(), binding_type.clone());
    }
    pub fn add_definition(&mut self, name: &String, definition_type: &Type)
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
