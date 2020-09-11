use crate::compiler::internal::*;

/* Pass: specs_c::make_user_main
    - Moves non-definition nodes in the top-level sequence into a main function
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
pub struct State
{
    pub is_root: bool,
}
impl PassState for State
{
    fn empty() -> Self
    {
        return State { is_root: true };
    }
}

/* -------------------------------------------------------------------------- */
/*                                    Pass                                    */
/* -------------------------------------------------------------------------- */

impl CompilerPass<State> for Pass
{
    // Modify a node given some state
    //  - All child nodes have already been transformed at this point
    fn transform(
        &mut self,
        node: &mut Node,
        state: Indirect<State>,
        messages: &mut PassMessageContext,
    )
    {
        let is_root = {
            let state = state.borrow();
            state.is_root
        };

        if is_root == false
        {
            return;
        }

        match node
        {
            Node::Sequence(sequence) =>
            {
                let mut user_main_nodes = Vec::new();
                for node in sequence.get_nodes_mut()
                {
                    match node
                    {
                        Node::Function(_) | Node::Class(_) =>
                        {}
                        _ =>
                        {
                            let mut temp = Node::nothing();
                            std::mem::swap(&mut temp, node);
                            user_main_nodes.push(temp);
                        }
                    }
                }

                let user_main_function = Function::new(
                    String::from("__SpecsMain__"),
                    Vec::new(),
                    types::basic_types::integer(),
                    Sequence::new(SequenceMode::Scope, user_main_nodes).to_node(),
                )
                .to_node();

                sequence.get_nodes_mut().push(user_main_function);
                sequence.set_mode(SequenceMode::Transparent);
            }

            _ =>
            {
                messages.add_error("InvalidRoot", "Expected root node to be a sequence", node);
            }
        }
    }

    // Get the state to use for a node's children based on the current state
    //  - Child nodes have not yet been visited
    fn get_child_states(
        &mut self,
        _node: &Node,
        parent: Indirect<State>,
        _messages: &mut PassMessageContext,
    ) -> Vec<Indirect<State>>
    {
        if parent.borrow().is_root
        {
            vec![Indirect::new(State::new(false))]
        }
        else
        {
            vec![parent.clone()]
        }
    }

    // Get the name of the pass (for debugging)
    fn get_name(&self) -> String
    {
        "MakeUserMain".to_owned()
    }
}

/* -------------------------------------------------------------------------- */
/*                                    State                                   */
/* -------------------------------------------------------------------------- */

impl State
{
    pub fn new(is_root: bool) -> State
    {
        State { is_root }
    }
}
