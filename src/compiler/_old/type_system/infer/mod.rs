mod primitive_call;

use crate::compiler::binding_state::BindingState;
use crate::compiler::internal::*;
use crate::language::types::all::*;

use std::collections::HashMap;

/* Pass: type_system::infer
    - Writes types for nodes based on
        - Definitions
        - Atomic data
        - Bindings
        - Type inferrence for primitive operators
*/

pub struct Pass {}

type TypeMap = HashMap<String, Indirect<Type>>;

type State = BindingState<Indirect<Type>, ()>;

/* -------------------------------------------------------------------------- */
/*                                    Pass                                    */
/* -------------------------------------------------------------------------- */

impl CompilerPass<State> for Pass
{
    // Infer the type of a node
    //  note: child node types have already been inferred at this point
    fn transform(
        &mut self,
        node: &mut Node,
        state: Indirect<State>,
        messages: &mut PassMessageContext,
    )
    {
        match node
        {
            // Look up variable type in state
            Node::Variable(variable) => match state.borrow().get(variable.get_name())
            {
                Some(t) =>
                {
                    variable.set_type(t.clone());
                }
                None =>
                {
                    messages.add_error(
                        "UnknownSymbol",
                        format!("Failed to get type for symbol {}", variable.get_name()),
                        node,
                    );
                }
            },

            // Add binding types to current state
            Node::Binding(binding) =>
            {
                state
                    .borrow_mut()
                    .add_binding(binding.get_name().clone(), binding.get_binding().get_type());
            }

            // Get type for primitive operators that don't depend on operands
            //  note: other primitive operator types are inferred at the call site
            Node::PrimitiveOperator(operator) =>
            {
                use crate::language::node::primitive::Operator;
                let operator_type = match operator.get_value()
                {
                    Operator::And | Operator::Or | Operator::ExclusiveOr => FunctionType::from(
                        vec![
                            basic_types::indirect::boolean(),
                            basic_types::indirect::boolean(),
                        ],
                        basic_types::indirect::boolean(),
                    ),

                    _ =>
                    {
                        // The operator type depends on its operands, and must be inferred at the call site
                        return;
                    }
                };

                operator.set_type(Indirect::new(operator_type.to_type()));
            }

            // Get return type from calls and infer operator type if needed
            Node::Call(call) =>
            {
                let (operator, operands) = call.get_all_mut();
                let operator_type = operator.get_type();

                // Try to infer operator type from operands if it is not already known
                if operator_type.borrow().is_unknown()
                {
                    match operator
                    {
                        Node::PrimitiveOperator(primitive) =>
                        {
                            match primitive_call::infer_from_operands(
                                primitive.get_value(),
                                operands,
                            )
                            {
                                Some(t) =>
                                {
                                    primitive.set_type(t);
                                }
                                None =>
                                {}
                            }
                        }
                        _ =>
                        {}
                    }
                }

                let operator_type = operator.get_type();
                let operator_type = operator_type.borrow();

                match &*operator_type
                {
                    Type::Function(function) =>
                    {
                        // The type of the Call node is the return type of the operator
                        call.set_type(function.get_return_type());
                    }
                    _ =>
                    {}
                }
            }

            // Create reference type from target
            Node::Reference(reference) =>
            {
                let target_type = reference.get_target().get_type();
                let reference_type =
                    ReferenceType::from(reference.get_mode(), target_type).to_type();

                reference.set_type(Indirect::new(reference_type));
            }
            // Create dereference type from target
            Node::Dereference(dereference) =>
            {
                let target_type = dereference.get_target().get_type();
                let dereference_type = target_type.borrow().dereference();

                if let Some(t) = dereference_type
                {
                    dereference.set_type(t);
                }
            }

            // Conditional type is always the same as the else branch
            Node::Conditional(conditional) =>
            {
                let else_type = conditional.get_else().get_type();
                conditional.set_type(else_type);
            }
            // Sequence type is the same as the last non-definition node
            Node::Sequence(sequence) =>
            {
                let t = {
                    let mut final_node = None;

                    let nodes = sequence.get_nodes();
                    for node in nodes.iter().rev()
                    {
                        if !node.is_definition()
                        {
                            final_node = Some(node);
                            break;
                        }
                    }

                    match final_node
                    {
                        Some(node) => Some(node.get_type()),
                        None => None,
                    }
                };
                match t
                {
                    Some(t) =>
                    {
                        sequence.set_type(t);
                    }
                    None =>
                    {
                        sequence.set_type(basic_types::indirect::void());
                    }
                }
            }

            _ =>
            {}
        }
    }
    // Get the type mapping that will be used for a node's children
    fn get_child_states(
        &mut self,
        node: &Node,
        parent: Indirect<State>,
        _messages: &mut PassMessageContext,
    ) -> Vec<Indirect<State>>
    {
        let new_state = match node
        {
            // Sequence nodes potentially create a new local scope, and immediately
            //  introduce definitions (regardless of node ordering)
            Node::Sequence(sequence) => match sequence.get_mode()
            {
                // Nodes in transparent sequences affect the parent scope
                control::SequenceMode::Transparent =>
                {
                    // Get definitions in this sequence and add them to parent scope
                    let definitions = get_definitions(sequence.get_nodes());
                    parent.borrow_mut().add_definitions(definitions);

                    // This node shares the parent's state
                    parent.clone()
                }

                // Nodes in normal sequences only affect the local scope
                control::SequenceMode::Scope =>
                {
                    // Create a new scope with local definitions, carrying over all
                    //  definitions and bindings from the parent scope
                    let local_definitions = get_definitions(sequence.get_nodes());
                    let new_state =
                        State::extend_with_definitions(parent, true, local_definitions, ());
                    Indirect::new(new_state)
                }
            },

            // Function nodes introduce a new local scope that only inherits definitions,
            //  immediately adding bindings for function arguments
            Node::Function(function) =>
            {
                let argument_bindings = function
                    .get_arguments()
                    .iter()
                    .map(|arg| (arg.get_name().clone(), arg.get_type()))
                    .collect();

                let new_state = State::extend_with_bindings(parent, false, argument_bindings, ());
                Indirect::new(new_state)
            }

            // Most nodes inherit the parent scope
            _ => parent.clone(),
        };

        // Share the new child state with all children
        vec![new_state]
    }

    // The name of the pass :)
    fn get_name(&self) -> String
    {
        "InferType".to_owned()
    }
}

// Helper to collect definition types from a sequence of nodes
fn get_definitions(nodes: &Vec<Node>) -> TypeMap
{
    let mut map = TypeMap::new();
    for node in nodes
    {
        match node
        {
            Node::Function(function) =>
            {
                map.insert(function.get_name().clone(), function.get_type());
            }
            _ =>
            {}
        }
    }
    return map;
}

impl Pass
{
    pub fn new() -> Pass
    {
        return Pass {};
    }
}
