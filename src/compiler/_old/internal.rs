use std::collections::VecDeque;

// All passes get access to basic stuff
pub use crate::language::*;
pub use crate::utilities::Indirect;
pub use node::all::*;

use crate::errors::compile_error::*;

pub trait PassState
{
    fn empty() -> Self;
}

pub trait CompilerPass<TState: PassState>
{
    // Get the name of the pass for logging
    fn get_name(&self) -> String;

    // Transform a node (once all its children have been transformed)
    fn transform(&mut self, node: &mut Node, state: Indirect<TState>, messages: &mut PassResult);

    // Get the state to pass to a node's children (before child nodes have been transformed)
    fn get_child_states(
        &mut self,
        node: &Node,
        parent: Indirect<TState>,
        messages: &mut PassResult,
    ) -> Vec<Indirect<TState>>;
}

// note: uses unsafe pointer conversions to simulate recursion with normal references
pub fn apply_compiler_pass<TState: PassState, TPass: CompilerPass<TState>>(
    mut pass: TPass,
    root_node: &mut Node,
) -> PassResult
{
    // Create the queue for expanding/transforming nodes
    let mut queue = VecDeque::new();

    // Create a message context to use for the whole pass
    let mut result = PassResult::Ok(());

    // Start with the root node on the queue
    let root_ptr = root_node as *mut Node;
    let root_state = TState::empty();
    let root_item = (0, false, Indirect::new(root_state), root_ptr);
    queue.push_front(root_item);

    // Continue expanding/transforming nodes until none are left
    loop
    {
        match queue.pop_front()
        {
            // The next node has not been expanded
            Some((depth, false, state, node_ptr)) =>
            {
                let node = unsafe { &*node_ptr };

                // Get all child nodes and new child states
                let children = node.get_children();
                let mut new_states = pass.get_child_states(node, state.clone(), &mut result);

                // Put the parent item back on the queue, marking that it has been expanded
                let parent_item = (depth, true, state, node_ptr);
                queue.push_front(parent_item);

                // Ensure we have one state per child
                let child_count = children.len();
                let child_states = match new_states.len()
                {
                    1 =>
                    {
                        // 1 child state => share among all children

                        let child_state = new_states.pop().unwrap();
                        std::iter::repeat(child_state).take(child_count).collect()
                    }
                    n if n == child_count =>
                    {
                        // N child states => each child has unique state
                        new_states
                    }
                    n =>
                    {
                        result.push_error(Error::new(
                            ErrorKind::Internal(format!(
                                "Invalid child state count {}, expected 1 or {}",
                                n, child_count
                            )),
                            node.get_source(),
                        ));
                        continue;
                    }
                };
                // Put all child nodes on the queue BEFORE the parent
                //  note: added in reverse so they're processed in-order
                let child_iter = children.into_iter().rev();
                let child_state_iter = child_states.into_iter().rev();

                for (child, state) in child_iter.zip(child_state_iter)
                {
                    // Turn the child reference into a pointer
                    let child_ptr = (child as *const Node) as *mut Node;

                    // Create the child item
                    let child_item = (depth + 1, false, state, child_ptr);
                    queue.push_front(child_item);
                }
            }

            // The next node has already been expanded, so it's ready to be transformed
            //  note: All child nodes would have been handled already, so this is the same as recursive order
            //  note: Children are processed in-order and may modify their shared state in an imperative fashion
            Some((depth, true, state, node_ptr)) =>
            {
                message_context.set_indent(depth);

                let node = unsafe { &mut *node_ptr };
                pass.transform(node, state, &mut message_context);
            }

            // No nodes left on the queue
            None =>
            {
                break;
            }
        }
    }

    let (warnings, errors) = message_context.destructure();
    if has_errors
    {
        PassResult::Error(errors, warnings);
    }
    else
    {
        PassResult::maybe_warn((), warnings);
    }
}

/* -------------------------------------------------------------------------- */
/*                      Empty PassState for simple passes                     */
/* -------------------------------------------------------------------------- */

pub struct PassStateEmpty {}
impl PassStateEmpty
{
    pub fn get_state(
        _node: &Node,
        parent: Indirect<PassStateEmpty>,
        _messages: &mut Passre,
    ) -> Indirect<PassStateEmpty>
    {
        return parent.clone();
    }
}
impl PassState for PassStateEmpty
{
    fn empty() -> Self
    {
        return Self {};
    }
}
