use super::common::*;

///
/// ## Explicate Returns Pass
///
/// - Inserts return operators at result nodes in functions
///
pub struct ExplicateReturns {}

impl ExplicateReturns
{
    pub fn new() -> ExplicateReturns
    {
        ExplicateReturns {}
    }
}

pub struct PassState {}

impl PassState
{
    pub fn new() -> PassState
    {
        PassState {}
    }
}

impl RecurTransform<Node, PassState, Error> for ExplicateReturns
{
    fn get_root_state(&mut self, _node: &Node) -> PassState
    {
        PassState::new()
    }

    fn exit(&mut self, node: &mut Node, _state: &mut PassState) -> ResultLog<(), Error>
    {
        match node
        {
            Node::Function(function) =>
            {
                let wrap_pass = WrapPass::new(|result_node| {
                    let source = result_node.get_source();
                    let return_operator =
                        PrimitiveOperator::new(primitive::Operator::Return, source.clone())
                            .to_node();
                    Call::new(return_operator, vec![result_node], source).to_node()
                });
                wrap_pass.apply(function.get_body_mut());
            }
            _ => (),
        }
        ResultLog::Ok(())
    }
}
