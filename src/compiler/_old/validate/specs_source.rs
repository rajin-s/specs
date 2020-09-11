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

fn is_valid_operand(node: &Node) -> bool
{
    match node
    {
        // Binding / assign can't be used as operands
        Node::Binding(_) | Node::Assign(_) => false,

        // Definitions can't be used as operands
        Node::Function(_) | Node::Class(_) => false,

        // All other nodes are fine to use as operands
        _ => true,
    }
}

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
                if !is_valid_operand(call.get_operator())
                {
                    messages.add_error(
                        "InvalidOperand",
                        format!(
                            "Can't use {} as call operator",
                            call.get_operator().get_name()
                        ),
                        node,
                    );
                }
                
                for (i, operand) in call.get_operands().iter().enumerate()
                {
                    if !is_valid_operand(operand)
                    {
                        messages.add_error(
                            "InvalidOperand",
                            format!(
                                "Can't use {} as call operand {}",
                                operand.get_name(), i
                            ),
                            node,
                        );
                    }
                }
            }
            Node::Reference(reference) =>
            {
                if !is_valid_operand(reference.get_target())
                {
                    messages.add_error(
                        "InvalidOperand",
                        format!(
                            "Can't use {} as reference target",
                            reference.get_target().get_name()
                        ),
                        node,
                    );
                }
            }
            Node::Dereference(dereference) =>
            {
                if !is_valid_operand(dereference.get_target())
                {
                    messages.add_error(
                        "InvalidOperand",
                        format!(
                            "Can't use {} as dereference target",
                            dereference.get_target().get_name()
                        ),
                        node,
                    );
                }
            }
            Node::Assign(assign) =>
            {
                if !is_valid_operand(assign.get_lhs())
                {
                    messages.add_error(
                        "InvalidOperand",
                        format!("Can't use {} as assign LHS", assign.get_lhs().get_name()),
                        node,
                    );
                }
                if !is_valid_operand(assign.get_rhs())
                {
                    messages.add_error(
                        "InvalidOperand",
                        format!("Can't use {} as assign RHS", assign.get_rhs().get_name()),
                        node,
                    );
                }
            }
            Node::Access(access) =>
            {
                if !is_valid_operand(access.get_target())
                {
                    messages.add_error(
                        "InvalidOperand",
                        format!(
                            "Can't use {} as access target",
                            access.get_target().get_name()
                        ),
                        node,
                    );
                }
            }
            Node::Binding(binding) =>
            {
                if !is_valid_operand(binding.get_binding())
                {
                    messages.add_error(
                        "InvalidOperand",
                        format!(
                            "Can't use {} as binding expression",
                            binding.get_binding().get_name()
                        ),
                        node,
                    );
                }
            }
            Node::Conditional(conditional) =>
            {
                if !is_valid_operand(conditional.get_condition())
                {
                    messages.add_error(
                        "InvalidOperand",
                        format!(
                            "Can't use {} as condition",
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
        "ValidateSpecsSource".to_owned()
    }
}
