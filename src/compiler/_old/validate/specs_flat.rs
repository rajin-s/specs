use crate::compiler::internal::*;

// Compiler pass instance
pub struct Pass {}
impl Pass
{
    pub fn new() -> Self
    {
        Self {}
    }
}

/* -------------------------------------------------------------------------- */
/*                                    Pass                                    */
/* -------------------------------------------------------------------------- */

impl CompilerPass<PassStateEmpty> for Pass
{
    fn transform(
        &mut self,
        node: &mut Node,
        _state: Indirect<PassStateEmpty>,
        messages: &mut PassMessageContext,
    )
    {
        let node = &node;

        match node
        {
            Node::Call(call) =>
            {
                if call.get_operator().is_complex()
                {
                    messages.add_error(
                        "InvalidOperand",
                        format!(
                            "{} as call operator should have been flattened",
                            call.get_operator().get_name()
                        ),
                        node,
                    );
                }
                
                for (i, operand) in call.get_operands().iter().enumerate()
                {
                    if operand.is_complex()
                    {
                        messages.add_error(
                            "InvalidOperand",
                            format!(
                                "{} as call operand {} should have been flattened",
                                operand.get_name(), i
                            ),
                            node,
                        );
                    }
                }
            }
            Node::Reference(reference) =>
            {
                if reference.get_target().is_complex()
                {
                    messages.add_error(
                        "InvalidOperand",
                        format!(
                            "{} as reference target should have been flattened",
                            reference.get_target().get_name()
                        ),
                        node,
                    );
                }
            }
            Node::Dereference(dereference) =>
            {
                if dereference.get_target().is_complex()
                {
                    messages.add_error(
                        "InvalidOperand",
                        format!(
                            "{} as dereference target should have been flattened",
                            dereference.get_target().get_name()
                        ),
                        node,
                    );
                }
            }
            Node::Assign(assign) =>
            {
                if assign.get_lhs().is_complex()
                {
                    messages.add_error(
                        "InvalidOperand",
                        format!("{} as assign LHS should have been flattened", assign.get_lhs().get_name()),
                        node,
                    );
                }
                if assign.get_rhs().is_complex()
                {
                    messages.add_error(
                        "InvalidOperand",
                        format!("{} as assign RHS should have been flattened", assign.get_rhs().get_name()),
                        node,
                    );
                }
            }
            Node::Access(access) =>
            {
                if access.get_target().is_complex()
                {
                    messages.add_error(
                        "InvalidOperand",
                        format!(
                            "{} as access target should have been flattened",
                            access.get_target().get_name()
                        ),
                        node,
                    );
                }
            }
            Node::Binding(binding) =>
            {
                if binding.get_binding().is_complex()
                {
                    messages.add_error(
                        "InvalidOperand",
                        format!(
                            "{} as binding expression should have been flattened",
                            binding.get_binding().get_name()
                        ),
                        node,
                    );
                }
            }
            Node::Conditional(conditional) =>
            {
                if conditional.get_condition().is_complex()
                {
                    messages.add_error(
                        "InvalidOperand",
                        format!(
                            "{} as condition should have been flattened",
                            conditional.get_condition().get_name()
                        ),
                        node,
                    );
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
        parent: Indirect<PassStateEmpty>,
        _messages: &mut PassMessageContext,
    ) -> Vec<Indirect<PassStateEmpty>>
    {
        vec![parent.clone()]
    }

    // Get the name of the pass (for debugging)
    fn get_name(&self) -> String
    {
        "ValidateSpecsFlat".to_owned()
    }
}
