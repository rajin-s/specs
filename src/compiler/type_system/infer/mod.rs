mod primitive_operator;

use crate::compiler::internal::*;
use crate::language::types::all::*;

use std::collections::HashMap;

type TypeMap = HashMap<String, Indirect<Type>>;

pub struct Pass {}
pub struct State
{
    definitions: Indirect<TypeMap>,
    bindings:    Indirect<TypeMap>,

    inherit_bindings: bool,

    parent: Option<Indirect<State>>,
}

/* -------------------------------------------------------------------------- */
/*                                    Pass                                    */
/* -------------------------------------------------------------------------- */

impl CompilerPass<State> for Pass
{
    // Infer the type of a node
    //  note: child node types have already been inferred at this point
    fn transform(&self, node: &mut Node, state: Indirect<State>, messages: &mut PassMessageContext)
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
                state.borrow_mut().add_binding(
                    binding.get_name().clone(),
                    binding.borrow_binding().get_type(),
                );
            }

            // Get type for primitive operators that don't depend on operands
            //  note: other primitive operator types are inferred at the call site
            Node::PrimitiveOperator(operator) =>
            {
                use crate::language::node::primitive::Operator;
                let operator_type = match operator.get_operator()
                {
                    // Operator type is known regardless of operands
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
                let operator = call.get_operator();
                let mut operator = operator.borrow_mut();

                let operator_is_unknown = {
                    let operator_type = operator.get_type();
                    let operator_type = operator_type.borrow();
                    operator_type.is_unknown()
                };

                // Try to infer operator type from operands if it is not already known
                if operator_is_unknown
                {
                    match &mut *operator
                    {
                        Node::PrimitiveOperator(primitive) =>
                        {
                            match primitive_operator::infer_from_operands(
                                primitive.get_operator(),
                                call.get_operands(),
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
                let t = {
                    let target = reference.borrow_target();
                    let target_type = target.get_type();
                    ReferenceType::from(reference.get_mode(), target_type).to_type()
                };

                reference.set_type(Indirect::new(t));
            }
            // Create dereference type from target
            Node::Dereference(dereference) =>
            {
                let t = {
                    let target = dereference.borrow_target();
                    let target_type = target.borrow_type();
                    target_type.dereference()
                };

                if let Some(t) = t
                {
                    dereference.set_type(t);
                }
            }

            // Conditional type is always the same as the else branch
            Node::Conditional(conditional) =>
            {
                let t = {
                    let else_branch = conditional.borrow_else();
                    else_branch.get_type()
                };
                conditional.set_type(t);
            }
            // Sequence type is the same as the last non-definition node
            Node::Sequence(sequence) =>
            {
                let t = {
                    let mut final_node = None;

                    let nodes = sequence.get_nodes();
                    for node in nodes.iter().rev()
                    {
                        if !node.borrow().is_definition()
                        {
                            final_node = Some(node);
                            break;
                        }
                    }

                    match final_node
                    {
                        Some(node) => Some(node.borrow().get_type()),
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
    fn get_state(
        &self,
        node: &Node,
        parent: Indirect<State>,
        messages: &mut PassMessageContext,
    ) -> Indirect<State>
    {
        match node
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
                    let new_state = State::extend_with_definitions(parent, true, local_definitions);
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

                let new_state = State::extend_with_bindings(parent, false, argument_bindings);
                Indirect::new(new_state)
            }

            // Most nodes inherit the parent scope
            _ => parent.clone(),
        }
    }

    // The name of the pass :)
    fn get_name(&self) -> String
    {
        "InferType".to_owned()
    }
}

// Helper to collect definition types from a sequence of nodes
fn get_definitions(nodes: &Vec<OtherNode>) -> TypeMap
{
    let mut map = TypeMap::new();
    for node in nodes
    {
        match &*node.borrow()
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

/* -------------------------------------------------------------------------- */
/*                                    State                                   */
/* -------------------------------------------------------------------------- */

impl State
{
    // Create a new state as a child of the current state, potentially including/excluding bindings
    //  - Immediately introduce local definitions
    pub fn extend_with_definitions(
        parent: Indirect<State>,
        inherit_bindings: bool,
        new_definitions: TypeMap,
    ) -> State
    {
        return State {
            definitions: Indirect::new(new_definitions),
            bindings: Indirect::new(TypeMap::new()),
            parent: Some(parent),
            inherit_bindings,
        };
    }
    // Create a new state as a child of the current state, potentially including/excluding bindings
    //  - Immediately introduce local bindings
    pub fn extend_with_bindings(
        parent: Indirect<State>,
        inherit_bindings: bool,
        new_bindings: TypeMap,
    ) -> State
    {
        return State {
            definitions: Indirect::new(TypeMap::new()),
            bindings: Indirect::new(new_bindings),
            parent: Some(parent),
            inherit_bindings,
        };
    }

    // Add a new bindings
    pub fn add_binding(&mut self, name: String, t: Indirect<Type>) -> bool
    {
        match self.bindings.borrow_mut().insert(name, t)
        {
            Some(_) => true,
            _ => false,
        }
    }

    // Add multiple definitions
    pub fn add_definitions(&mut self, map: TypeMap)
    {
        for (name, t) in map.into_iter()
        {
            self.definitions.borrow_mut().insert(name, t);
        }
    }

    // Internal type lookup function that respects binding inheritance properties
    fn lookup(&self, name: &String, check_bindings: bool) -> Option<Indirect<Type>>
    {
        // Check local bindings first
        if check_bindings
        {
            if let Some(t) = self.bindings.borrow().get(name)
            {
                return Some(t.clone());
            }
        }
        // Check local definitions second
        if let Some(t) = self.definitions.borrow().get(name)
        {
            return Some(t.clone());
        }
        // Check parent scope last with a (hopefully) tail-recursive call
        //  note: propagates check_bindings=false
        if let Some(parent) = &self.parent
        {
            return parent
                .borrow()
                .lookup(name, check_bindings && self.inherit_bindings);
        }
        return None;
    }

    // Get a symbol from the current scope, or look it up in the parent scope
    pub fn get(&self, name: &String) -> Option<Indirect<Type>>
    {
        self.lookup(name, true)
    }
}
impl PassState for State
{
    fn empty() -> Self
    {
        return State {
            definitions:      Indirect::new(TypeMap::new()),
            bindings:         Indirect::new(TypeMap::new()),
            inherit_bindings: false,
            parent:           None,
        };
    }
}
