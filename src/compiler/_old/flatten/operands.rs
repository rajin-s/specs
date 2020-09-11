use crate::compiler::internal::*;
use crate::compiler::utilities::TempNameGenerator;

/* Pass: flatten::operands
    - Substitutes complex nodes used in an operand context with a temporary binding such that all operands are simple
        - simple:   atomic
                    (call simple*)
    - (resulting bindings are flattened in flatten::bindings)
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
            names: TempNameGenerator::new("xop"),
        }
    }
}
type State = PassStateEmpty;

/* -------------------------------------------------------------------------- */
/*                                    Pass                                    */
/* -------------------------------------------------------------------------- */

// Complex nodes (that can't be used as ex. a function argument in C) need to
//  be bound to a temporary before use. After this pass, all operands should be
//  C-compatible (ie. atomic, function calls, etc.)

// Extract complex nodes, replacing each with a reference to a temporary variable
// Returns a list of nodes to bind each temporary to its corresponding original expression
fn bind_complex(nodes: Vec<&mut Node>, names: &mut TempNameGenerator) -> Vec<Node>
{
    use atomic::Variable;
    use binding::Binding;

    let mut result = Vec::new();
    for node in nodes
    {
        if node.is_complex()
        {
            let binding_name = names.next();

            // Extract the complex node, replacing it with a temporary variable
            let original_node = {
                let mut temp = Variable::new(binding_name.clone()).to_node();
                std::mem::swap(&mut temp, node);
                temp
            };

            // Create a binding for the new temporary
            let binding = Binding::new(binding_name, original_node).to_node();
            result.push(binding);
        }
    }
    result
}

// Prepend a list of bindings to a node by packaging it into a transparent sequence
fn prepend_bindings(mut bindings: Vec<Node>, original: &mut Node)
{
    use control::{Sequence, SequenceMode};

    // Don't create a sequence if there are no bindings to worry about
    if !bindings.is_empty()
    {
        // Extract the original node
        let mut temp = Node::nothing();
        std::mem::swap(&mut temp, original);

        // Add the original node after the bindings and wrap with a sequence
        bindings.push(temp);
        let mut sequence = Sequence::new(SequenceMode::Transparent, bindings).to_node();

        // Put the new sequence back in place of the original
        std::mem::swap(&mut sequence, original);
    }
}

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
        let names = &mut self.names;
        match node
        {
            Node::Call(_)
            | Node::Reference(_)
            | Node::Dereference(_)
            | Node::Assign(_)
            | Node::Access(_) =>
            {
                // All children of operator-like nodes must be simple
                let bindings = bind_complex(node.get_children_mut(), names);
                prepend_bindings(bindings, node);
            }

            Node::Conditional(conditional) =>
            {
                // Only the condition node of a conditional must be simple
                let bindings = bind_complex(vec![conditional.get_condition_mut()], names);
                prepend_bindings(bindings, node);
            }

            _ =>
            {
                // Atomic nodes have no children
                // All sequence body nodes can be complex
                // Definition bodies can be complex

                // Bindings will be handled in the next pass (since we're creating a whole bunch of complex bindings)
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
        "FlattenOperands".to_owned()
    }
}
