use super::common::*;

///
/// ## Flatten Definitions Pass
///
/// - Moves all definitions to the top-level sequence
/// - Definition names should have been made unique (and variable references updated) in FlattenNames
///
pub struct FlattenDefinitions
{
    functions: Vec<Node>,
}

impl FlattenDefinitions
{
    pub fn new() -> FlattenDefinitions
    {
        FlattenDefinitions {
            functions: Vec::new(),
        }
    }
}

///
/// Pass state for FlattenDefinitions
///
/// - Track if we should dump the accululated definitions at this node
///
pub struct PassState
{
    pub is_root: bool,
}

impl PassState
{
    pub fn new(is_root: bool) -> PassState
    {
        PassState { is_root }
    }
}

impl RecurTransform<Node, PassState, Error> for FlattenDefinitions
{
    fn get_root_state(&mut self, _node: &Node) -> PassState
    {
        PassState::new(true)
    }

    fn get_child_states(&mut self, state: &PassState, _node: &Node) -> Vec<ChildState<PassState>>
    {
        if state.is_root
        {
            vec![ChildState::New(PassState::new(false))]
        }
        else
        {
            vec![ChildState::Inherit]
        }
    }

    fn exit(&mut self, node: &mut Node, state: &mut PassState) -> ResultLog<(), Error>
    {
        match node
        {
            Node::Sequence(sequence) if state.is_root =>
            {
                // Put the original root sequence nodes after all definitions

                let mut new_nodes = Vec::new();
                let mut original_nodes = std::mem::take(sequence.get_nodes_mut());

                new_nodes.append(&mut self.functions);
                new_nodes.append(&mut original_nodes);

                *sequence.get_nodes_mut() = new_nodes;
            }
            Node::Function(function) =>
            {
                // Extract functions and replace with a placeholder comment

                let comment = format!("fn {}", function.get_name());
                let original_node = node.extract_comment(comment);

                self.functions.push(original_node);
            }
            _ => (),
        }

        ResultLog::Ok(())
    }
}
