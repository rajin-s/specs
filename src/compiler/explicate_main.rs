use super::common::*;

///
/// ## Explicate Main Pass
///
/// - Moved all non-definition nodes from the root sequence into a main function
///
pub struct ExplicateMain
{
    name: String,
}

impl ExplicateMain
{
    pub fn new(name: &str) -> ExplicateMain
    {
        ExplicateMain {
            name: String::from(name),
        }
    }
}

///
/// Pass state for ExplicateMain
///
/// - ...
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

impl RecurTransform<Node, PassState, Error> for ExplicateMain
{
    fn get_root_state(&mut self, node: &Node) -> PassState
    {
        PassState::new(true)
    }

    fn get_child_states(&mut self, state: &PassState, node: &Node) -> Vec<ChildState<PassState>>
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
                let mut main_nodes = Vec::new();
                for node in sequence.get_nodes_mut()
                {
                    if !node.is_definition()
                    {
                        main_nodes.push(node.extract_temp());
                    }
                }

                let main_function = Function::new(
                    self.name.clone(),
                    Vec::new(),
                    basic_types::integer(),
                    Sequence::new(SequenceMode::Scope, main_nodes, sequence.get_source()).to_node(),
                    sequence.get_source(),
                )
                .to_node();

                sequence.get_nodes_mut().push(main_function);
                sequence.set_mode(SequenceMode::Transparent);
            }
            _ => (),
        }

        ResultLog::Ok(())
    }
}
