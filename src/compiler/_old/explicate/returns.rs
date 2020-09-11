use crate::compiler::internal::*;

/* Pass: explicate::returns
    - Inserts return operators at result nodes
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

/* -------------------------------------------------------------------------- */
/*                                    Pass                                    */
/* -------------------------------------------------------------------------- */

fn make_return(node: Node) -> Node
{
    use atomic::PrimitiveOperator;
    use operator::Call;

    Call::new(
        PrimitiveOperator::new(primitive::Operator::Return).to_node(),
        vec![node],
    )
    .to_node()
}

impl CompilerPass<PassStateEmpty> for Pass
{
    fn transform(
        &mut self,
        node: &mut Node,
        _state: Indirect<PassStateEmpty>,
        _messages: &mut PassMessageContext,
    )
    {
        use crate::compiler::wrap_pass::WrapPass;

        match node
        {
            Node::Function(function) =>
            {
                let wrap_pass = WrapPass::new(make_return);
                apply_compiler_pass(wrap_pass, function.get_body_mut());
            }
            Node::Class(_class) =>
            {}

            _ =>
            {}
        }
    }

    // Get the state to use for a node's children based on the current state
    //  - Child nodes have not yet been visited
    fn get_child_states(
        &mut self,
        _node: &Node,
        parent: Indirect<PassStateEmpty>,
        _messages: &mut PassMessageContext,
    ) -> Vec<Indirect<PassStateEmpty>>
    {
        vec![parent.clone()]
    }

    // Get the name of the pass (for debugging)
    fn get_name(&self) -> String
    {
        "ExplicateReturns".to_owned()
    }
}
