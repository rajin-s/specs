use crate::compiler::internal::*;

/* Pass: flatten::definitions
    - Moves all definitions to the top-level sequence
    - Definition names have been made unique thanks to flatten::definition_names
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
    is_top_level: bool,
    definitions:  Indirect<Vec<Node>>,
}
impl PassState for State
{
    fn empty() -> Self
    {
        return Self {
            is_top_level: true,
            definitions:  Indirect::new(Vec::new()),
        };
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
        _messages: &mut PassMessageContext,
    )
    {
        match node
        {
            Node::Function(function) =>
            {
                // Extract the definition node (replacing a comment)
                let mut definition =
                    atomic::Comment::new(format!("fn {}", function.get_name())).to_node();
                std::mem::swap(&mut definition, node);

                // Leave the definition node to be used by the top-level sequence
                let state = state.borrow_mut();
                let mut definitions = state.definitions.borrow_mut();
                definitions.push(definition);
            }
            
            Node::Class(class) =>
            {
                // Extract the definition node (replacing a comment)
                let mut definition =
                    atomic::Comment::new(format!("fn {}", class.get_name())).to_node();
                std::mem::swap(&mut definition, node);

                // Leave the definition node to be used by the top-level sequence
                let state = state.borrow_mut();
                let mut definitions = state.definitions.borrow_mut();
                definitions.push(definition);
            }

            Node::Sequence(sequence) =>
            {
                // Pull in definition nodes for the top-level sequence
                if state.borrow().is_top_level
                {
                    // Extract the list of definitions
                    let state = state.borrow_mut();
                    let mut definition_nodes = Vec::new();
                    std::mem::swap(
                        &mut definition_nodes,
                        state.definitions.borrow_mut().as_mut(),
                    );

                    // Extract the sequence's nodes
                    let mut sequence_nodes = Vec::new();
                    std::mem::swap(&mut sequence_nodes, sequence.get_nodes_mut());
                    // Put the original sequence nodes after all the definitions
                    definition_nodes.append(&mut sequence_nodes);
                    // Put the new definitions + sequence nodes back in place
                    std::mem::swap(&mut definition_nodes, sequence.get_nodes_mut());
                }
            }

            _ =>
            {}
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
        let new_state = if parent.borrow().is_top_level
        {
            // Make a new non-top-level state if needed
            Indirect::new(State::new_child(parent))
        }
        else
        {
            // Reuse the same non-top-level state if possible
            parent.clone()
        };

        vec![new_state]
    }

    // Get the name of the pass (for debugging)
    fn get_name(&self) -> String
    {
        "FlattenDefinitions".to_owned()
    }
}

/* -------------------------------------------------------------------------- */
/*                                    State                                   */
/* -------------------------------------------------------------------------- */

impl State
{
    fn new_child(parent: Indirect<State>) -> Self
    {
        Self {
            is_top_level: false,
            definitions:  parent.borrow().definitions.clone(),
        }
    }
}
