use super::common::*;

///
/// ## Flatten Operands Pass
///
/// - Substitutes complex nodes used in operand context with a temporary binding such that all
///     operands are simple (either atomic or a call on simple operands)
///
pub struct FlattenOperands
{
    temp_names: TempNameGenerator,
}

impl FlattenOperands
{
    pub fn new() -> FlattenOperands
    {
        FlattenOperands {
            temp_names: TempNameGenerator::new("xop"),
        }
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

impl FlattenOperands
{
    ///
    /// Extract complex nodes, replacing each with a reference to a temporary variable
    ///
    /// - Returns a list of Binding nodes for each temporary variable (which should go before the original node)
    ///
    fn bind_complex(&mut self, nodes: Vec<&mut Node>) -> Vec<Node>
    {
        let mut bindings = Vec::new();
        for node in nodes
        {
            if node.is_complex()
            {
                let source = node.get_source();

                let temp_name = self.temp_names.next();
                let temp_variable = Variable::new(temp_name.clone(), source.clone()).to_node();
                let original_node = node.extract(temp_variable);

                let binding = Binding::new(temp_name, original_node, source).to_node();

                bindings.push(binding);
            }
        }
        bindings
    }

    ///
    /// Takes a node and a prepends it with list of bindings for all temporary complex operand variables
    ///
    fn prepend_bindings(node: &mut Node, mut bindings: Vec<Node>)
    {
        if bindings.is_empty()
        {
            // Don't do anything if we don't have any bindings to prepend

            return;
        }

        let source = node.get_source();

        // Get the original node, add it after the bindings, and create an enclosing sequence

        let original_node = node.extract_temp();
        bindings.push(original_node);
        let sequence = Sequence::new(SequenceMode::Transparent, bindings, source).to_node();

        // Put the new sequence back in place

        *node = sequence;
    }
}

impl RecurTransform<Node, PassState, Error> for FlattenOperands
{
    fn get_root_state(&mut self, _node: &Node) -> PassState
    {
        PassState::new()
    }

    fn exit(&mut self, node: &mut Node, _state: &mut PassState) -> ResultLog<(), Error>
    {
        match node
        {
            Node::Call(_)
            | Node::Reference(_)
            | Node::Dereference(_)
            | Node::Assign(_)
            | Node::Access(_) =>
            {
                // All children of call-like nodes must be simple

                let bindings = self.bind_complex(node.get_children_mut());
                Self::prepend_bindings(node, bindings);
            }
            Node::Conditional(conditional) =>
            {
                // Only the condition node of a Conditional must be simple

                let bindings = self.bind_complex(vec![conditional.get_condition_mut()]);
                Self::prepend_bindings(node, bindings);
            }
            _ => (),
        }

        ResultLog::Ok(())
    }
}
