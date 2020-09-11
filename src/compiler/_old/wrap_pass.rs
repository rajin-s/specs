use crate::compiler::internal::*;

/* Pass: wrap_pass
    - Wraps nodes that are in a returning context with the result of some Node->Node function
    - Used internally by other passes 
        - converting sequence results into assignments
        - replacing function results with return operators
*/

// Compiler pass instance
pub struct WrapPass<TFunction>
where
    TFunction: Fn(Node) -> Node,
{
    function: TFunction,
}
impl<TFunction> WrapPass<TFunction>
where
    TFunction: Fn(Node) -> Node,
{
    pub fn new(function: TFunction) -> Self
    {
        Self { function }
    }
}

// Pass state
//  - Generated when descending the AST
//  - Potentially modified while ascending the AST (in execution order)
pub struct WrapPassState
{
    pub is_return_context: bool,
}
impl PassState for WrapPassState
{
    fn empty() -> Self
    {
        return WrapPassState {
            is_return_context: true,
        };
    }
}

/* -------------------------------------------------------------------------- */
/*                                    Pass                                    */
/* -------------------------------------------------------------------------- */

impl<TFunction> CompilerPass<WrapPassState> for WrapPass<TFunction>
where
    TFunction: Fn(Node) -> Node,
{
    // Modify a node given some state
    //  - All child nodes have already been transformed at this point
    fn transform(
        &mut self,
        node: &mut Node,
        state: Indirect<WrapPassState>,
        _messages: &mut PassMessageContext,
    )
    {
        // We only want to wrap nodes which have no children in return contexts
        if !node.is_complex()
        {
            if state.borrow().is_return_context
            {
                // Extract the original node
                let mut original = Node::nothing();
                std::mem::swap(&mut original, node);

                // Wrap the original node
                let wrapper = &self.function;
                let mut wrapped = wrapper(original);

                // Put the original node back
                std::mem::swap(&mut wrapped, node);
            }
        }
    }

    // Get the state to use for a node's children based on the current state
    //  - Child nodes have not yet been visited
    fn get_child_states(
        &mut self,
        node: &Node,
        parent: Indirect<WrapPassState>,
        _messages: &mut PassMessageContext,
    ) -> Vec<Indirect<WrapPassState>>
    {
        let non_returning_state = Indirect::new(WrapPassState::new(false));

        match node
        {
            // Result body node inherits context from parent, other nodes are never in a return context
            Node::Sequence(sequence) =>
            {
                // Only worry about finding the result node if it will be in a return context
                if parent.borrow().is_return_context
                {
                    match sequence.get_result_index()
                    {
                        Some(index) =>
                        {
                            let node_count = sequence.get_nodes().len();
                            let mut result = Vec::new();
                            result.reserve_exact(node_count);

                            for _ in 0..index
                            {
                                result.push(non_returning_state.clone());
                            }

                            result.push(parent.clone());

                            for _ in index + 1..node_count
                            {
                                result.push(non_returning_state.clone());
                            }

                            result
                        }
                        None =>
                        {
                            // No child node is in a return context (void sequence)
                            vec![non_returning_state]
                        }
                    }
                }
                else
                {
                    // Sequence isn't in a return context, so no child nodes will be either
                    vec![non_returning_state]
                }
            }
            Node::Conditional(_conditional) =>
            {
                // Condition is never in a return context, branches inherit context from parent
                vec![non_returning_state, parent.clone(), parent.clone()]
            }

            _ =>
            {
                // Children of other (non-complex) nodes will never be in a return context
                vec![non_returning_state]
            }
        }
    }

    // Get the name of the pass (for debugging)
    fn get_name(&self) -> String
    {
        "WrapPass".to_owned()
    }
}

/* -------------------------------------------------------------------------- */
/*                                    State                                   */
/* -------------------------------------------------------------------------- */

impl WrapPassState
{
    pub fn new(is_return_context: bool) -> Self
    {
        return Self { is_return_context };
    }
}
