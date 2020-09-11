use crate::compiler::internal::*;
use crate::compiler::utilities::TempNameGenerator;

/* Pass: flatten::bindings
    - Converts complex bindings into an empty temporary followed by a complex expression that assigns to the temporary
    
    ex.
    let foo = { bar }
    ==>
        /
            let _temp = [nothing]
            {
                temp = bar
            }
            let foo = temp
        \

*/

// Compiler pass instance
pub struct Pass
{
    names: TempNameGenerator,
}
impl Pass
{
    pub fn new() -> Self
    {
        Self {
            names: TempNameGenerator::new("xbind"),
        }
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

// Complex bindings need to turned into empty bindings followed by an assignment

impl CompilerPass<State> for Pass
{
    // Modify a node given some state
    //  - All child nodes have already been transformed at this point
    fn transform(
        &mut self,
        node: &mut Node,
        _state: Indirect<State>,
        _messages: &mut PassMessageContext,
    )
    {
        use crate::compiler::wrap_pass::WrapPass;
        use node::all::*;

        match node
        {
            Node::Binding(binding) =>
            {
                if binding.get_binding().is_complex()
                {
                    let binding_type = binding.get_binding().get_type();

                    // Create a new temporary variable with the same type as the binding
                    let temporary_name = self.names.next();
                    let bind_temporary = Binding::new(
                        temporary_name.clone(),
                        Node::nothing_typed(binding_type.clone()),
                    )
                    .to_node();

                    // Extract the original binding, replacing it with the temporary
                    let mut assign_temporary =
                        Variable::new_typed(temporary_name.clone(), binding_type.clone()).to_node();
                    std::mem::swap(&mut assign_temporary, binding.get_binding_mut());

                    // Convert the result of the original binding into an assignment using a WrapPass
                    //  ie. {... node} => {... (temp = node)}
                    let wrap_pass = WrapPass::new(|n| {
                        Assign::new(
                            Variable::new_typed(temporary_name.clone(), binding_type.clone())
                                .to_node(),
                            n,
                        )
                        .to_node()
                    });
                    apply_compiler_pass(wrap_pass, &mut assign_temporary);

                    // Extract the original node
                    let mut use_temporary = Node::nothing();
                    std::mem::swap(&mut use_temporary, node);

                    // Wrap the temporary, assignment, and original node into a transparent sequence
                    let mut sequence = Sequence::new(
                        SequenceMode::Transparent,
                        vec![bind_temporary, assign_temporary, use_temporary],
                    )
                    .to_node();

                    // Put the new sequence back in place of the original node
                    std::mem::swap(&mut sequence, node);
                }
            }

            _ =>
            {
                // We only care about complex bindings
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
        vec![parent.clone()]
    }

    // Get the name of the pass (for debugging)
    fn get_name(&self) -> String
    {
        "FlattenBindings".to_owned()
    }
}

/* -------------------------------------------------------------------------- */
/*                                    State                                   */
/* -------------------------------------------------------------------------- */

impl State {}
