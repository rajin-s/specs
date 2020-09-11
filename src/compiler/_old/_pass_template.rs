use crate::compiler::internal::*;

/* Pass: name
    - Does something
*/

// Compiler pass instance
pub struct Pass {}
impl Pass
{
    pub fn new() -> Self
    {
        Self {}
    }
}

// Pass state
//  - Generated when descending the AST
//  - Potentially modified while ascending the AST (in execution order)
pub struct State {}
impl PassState for State
{
    fn empty() -> Self
    {
        return State {};
    }
}

/* -------------------------------------------------------------------------- */
/*                                    Pass                                    */
/* -------------------------------------------------------------------------- */

impl CompilerPass<State> for Pass
{
    // Modify a node given some state
    //  - All child nodes have already been transformed at this point
    fn transform(&mut self, node: &mut Node, state: Indirect<State>, messages: &mut PassMessageContext)
    {
        match node
        {
            Node::Nothing(_) =>
            {}
            Node::Integer(integer) =>
            {}
            Node::Boolean(boolean) =>
            {}
            Node::Variable(variable) =>
            {}
            Node::PrimitiveOperator(operator) =>
            {}
            Node::Call(call) =>
            {}
            Node::Reference(reference) =>
            {}
            Node::Dereference(dereference) =>
            {}
            Node::Assign(assign) =>
            {}
            Node::Access(access) =>
            {}
            Node::Binding(binding) =>
            {}
            Node::Sequence(sequence) =>
            {}
            Node::Conditional(conditional) =>
            {}
            Node::Function(function) =>
            {}
            Node::Class(class) =>
            {}

            _ =>
            {}
        }
    }

    // Get the state to use for a node's children based on the current state
    //  - Child nodes have not yet been visited
    fn get_child_states(
        &mut self,
        node: &Node,
        parent: Indirect<State>,
        messages: &mut PassMessageContext,
    ) -> Vec<Indirect<State>>
    {
        vec![parent.clone()]
    }

    // Get the name of the pass (for debugging)
    fn get_name(&self) -> String
    {
        "PassName".to_owned()
    }
}

/* -------------------------------------------------------------------------- */
/*                                    State                                   */
/* -------------------------------------------------------------------------- */

impl State {}
