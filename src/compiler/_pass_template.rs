use super::common::*;

///
/// ## ___ Pass
///
/// - ...
///
pub struct PassName {}

impl PassName
{
    pub fn new() -> PassName
    {
        PassName {}
    }
}

///
/// Pass state for PassName
/// 
/// - ...
/// 
pub struct PassState {}

impl PassState
{
    pub fn new() -> PassState
    {
        PassState {}
    }
}

impl RecurTransform<Node, PassState, Error> for PassName
{
    fn get_root_state(&mut self, node: &Node) -> PassState
    {
        PassState::new()
    }

    fn get_child_states(&mut self, state: &PassState, node: &Node) -> Vec<ChildState<PassState>>
    {
        vec![ChildState::Inherit]
    }

    fn enter(&mut self, node: &mut Node, state: &mut PassState) -> ResultLog<(), Error>
    {
        ResultLog::Ok(())
    }

    fn exit(&mut self, node: &mut Node, state: &mut PassState) -> ResultLog<(), Error>
    {
        ResultLog::Ok(())
    }
}
