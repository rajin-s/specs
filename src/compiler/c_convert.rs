use super::common::*;

///
/// ## C Convert Pass
///
/// - Try and collapse the whole node tree into a CNode with the final C output program
///
pub struct Convert {}

impl Convert
{
    pub fn new() -> Convert
    {
        Convert {}
    }
}

///
/// Pass state for Convert
///
/// - ...
///
pub struct PassState {}

impl PassState
{
    pub fn new() -> PassState
    {
        PassState {}
    }
}

impl RecurTransform<Node, PassState, Error> for Convert
{
    fn get_root_state(&mut self, _node: &Node) -> PassState
    {
        PassState::new()
    }

    fn exit(&mut self, node: &mut Node, _state: &mut PassState) -> ResultLog<(), Error>
    {
        match node
        {
            Node::Comment(comment) =>
            {
                let text = format!("/* {} */", comment.get_content());
                *node = CNode::new(
                    CNodeType::Comment,
                    text,
                    comment.get_type(),
                    comment.get_source(),
                )
                .to_node();
            }
            Node::Integer(integer) =>
            {
                let text = format!("{}", integer.get_value());
                *node = CNode::new(
                    CNodeType::Atomic,
                    text,
                    integer.get_type(),
                    integer.get_source(),
                )
                .to_node();
            }
            Node::Boolean(boolean) =>
            {
                let text = format!("{}", boolean.get_value());
                *node = CNode::new(
                    CNodeType::Atomic,
                    text,
                    boolean.get_type(),
                    boolean.get_source(),
                )
                .to_node();
            }
            Node::Variable(variable) =>
            {
                let text = format!("{}", variable.get_name());
                *node = CNode::new(
                    CNodeType::Atomic,
                    text,
                    variable.get_type(),
                    variable.get_source(),
                )
                .to_node();
            }
            Node::Nothing(nothing) =>
            {
                *node = CNode::new(
                    CNodeType::Nothing,
                    String::new(),
                    nothing.get_type(),
                    nothing.get_source(),
                )
                .to_node();
            }
            Node::PrimitiveOperator(operator) =>
            {
                let (node_type, text) = match operator.get_value()
                {
                    // Arithmetic operators
                    primitive::Operator::Add => (CNodeType::InfixOperator, "+"),
                    primitive::Operator::Subtract => (CNodeType::InfixOperator, "-"),
                    primitive::Operator::Multiply => (CNodeType::InfixOperator, "*"),
                    primitive::Operator::Divide => (CNodeType::InfixOperator, "/"),
                    primitive::Operator::Modulo => (CNodeType::InfixOperator, "%"),

                    // Comparison operators
                    primitive::Operator::Equal => (CNodeType::InfixOperator, "=="),
                    primitive::Operator::NotEqual => (CNodeType::InfixOperator, "!="),
                    primitive::Operator::Less => (CNodeType::InfixOperator, "<"),
                    primitive::Operator::Greater => (CNodeType::InfixOperator, ">"),
                    primitive::Operator::LessEqual => (CNodeType::InfixOperator, "<="),
                    primitive::Operator::GreaterEqual => (CNodeType::InfixOperator, ">="),

                    // Logical operators
                    primitive::Operator::Not => (CNodeType::PrefixOperator, "!"),
                    primitive::Operator::And => (CNodeType::InfixOperator, "&&"),
                    primitive::Operator::Or => (CNodeType::InfixOperator, "||"),
                    primitive::Operator::ExclusiveOr => (CNodeType::InfixOperator, "^"),

                    // Memory operators
                    // primitive::Operator::HeapAllocate => (CNodeType::Atomic, ""),
                    // primitive::Operator::HeapFree => (CNodeType::Atomic, ""),

                    // Other operators
                    primitive::Operator::Return => (CNodeType::PrefixOperator, "return"),

                    // Anything else should have been converted already
                    _ => (CNodeType::Atomic, "unknown_operator"),
                };

                *node = CNode::new(
                    node_type,
                    text.to_owned(),
                    operator.get_type(),
                    operator.get_source(),
                )
                .to_node();
            }
            Node::Call(call) =>
            {
                match call.get_operator()
                {
                    Node::CNode(operator_cnode) =>
                    {
                        match operator_cnode.get_node_type()
                        {
                            CNodeType::InfixOperator if call.get_operands().len() > 1 =>
                            {
                                let operands = call.get_operands();
                                let (a_cnode, b_cnode) = match (&operands[0], &operands[1])
                                {
                                    (Node::CNode(a_cnode), Node::CNode(b_cnode)) =>
                                    {
                                        (a_cnode, b_cnode)
                                    }
                                    (a_node, b_node) =>
                                    {
                                        return ResultLog::new_error(Error::Internal(format!(
                                            "Expected infix operands to be CNodes: {}, {}",
                                            a_node, b_node,
                                        )));
                                    }
                                };

                                // Make sure the operands aren't bare operator applications
                                //  (potentially messing with order of operations)

                                let text = format!(
                                    "{} {} {}",
                                    a_cnode.get_text_non_bare(),
                                    call.get_operator(),
                                    b_cnode.get_text_non_bare(),
                                );
                                *node = CNode::new(
                                    CNodeType::Bare,
                                    text,
                                    call.get_type(),
                                    call.get_source(),
                                )
                                .to_node();
                            }

                            CNodeType::PrefixOperator | CNodeType::InfixOperator =>
                            {
                                // NOTE: We handle infix operators with 1 operand as prefix
                                //          operators for ambiguous unary/binary -
                                let a_cnode = match &call.get_operands()[0]
                                {
                                    Node::CNode(cnode) => cnode,
                                    operand =>
                                    {
                                        return ResultLog::new_error(Error::Internal(format!(
                                            "Expected prefix operand to be a CNode: {}",
                                            operand,
                                        )));
                                    }
                                };
                                // Make sure the operand isn't a bare operator application

                                let text = format!(
                                    "{} {}",
                                    call.get_operator(),
                                    a_cnode.get_text_non_bare()
                                );
                                *node = CNode::new(
                                    CNodeType::Bare,
                                    text,
                                    call.get_type(),
                                    call.get_source(),
                                )
                                .to_node();
                            }

                            _ =>
                            {
                                let mut text = format!("{}(", call.get_operator());
                                for (i, operand) in call.get_operands().iter().enumerate()
                                {
                                    let operand_cnode = match operand
                                    {
                                        Node::CNode(cnode) => cnode,
                                        operand =>
                                        {
                                            return ResultLog::new_error(Error::Internal(format!(
                                                "Expected call operand to be a CNode: {}",
                                                operand,
                                            )));
                                        }
                                    };
                                    let operand_text = format!("{}", operand_cnode);
                                    text = if i == 0
                                    {
                                        format!("{}{}", text, operand_text)
                                    }
                                    else
                                    {
                                        format!("{}, {}", text, operand_text)
                                    };
                                }

                                text = format!("{})", text);
                                *node = CNode::new(
                                    CNodeType::Call,
                                    text,
                                    call.get_type(),
                                    call.get_source(),
                                )
                                .to_node();
                            }
                        }
                    }
                    operator_node =>
                    {
                        return ResultLog::new_error(Error::Internal(format!(
                            "Expected call operator to be a CNode: {}",
                            operator_node,
                        )))
                    }
                }
            }

            Node::Reference(reference) =>
            {
                let target_cnode = match reference.get_target()
                {
                    Node::CNode(cnode) => cnode,
                    target_node =>
                    {
                        return ResultLog::new_error(Error::Internal(format!(
                            "Expected reference target to be a CNode: {}",
                            target_node,
                        )));
                    }
                };

                let text = format!("&{}", target_cnode.get_text_non_bare());
                *node = CNode::new(
                    CNodeType::Bare,
                    text,
                    reference.get_type(),
                    reference.get_source(),
                )
                .to_node();
            }

            Node::Dereference(dereference) =>
            {
                let target_cnode = match dereference.get_target()
                {
                    Node::CNode(cnode) => cnode,
                    target_node =>
                    {
                        return ResultLog::new_error(Error::Internal(format!(
                            "Expected dereference target to be a CNode: {}",
                            target_node,
                        )));
                    }
                };

                let text = format!("*{}", target_cnode.get_text_non_bare());
                *node = CNode::new(
                    CNodeType::Bare,
                    text,
                    dereference.get_type(),
                    dereference.get_source(),
                )
                .to_node();
            }

            Node::Assign(assign) =>
            {
                let (lhs_cnode, rhs_cnode) = match (assign.get_lhs(), assign.get_rhs())
                {
                    (Node::CNode(lhs_cnode), Node::CNode(rhs_cnode)) => (lhs_cnode, rhs_cnode),
                    (lhs_node, rhs_node) =>
                    {
                        return ResultLog::new_error(Error::Internal(format!(
                            "Expected assign lhs and rhs to be CNodes: {}, {}",
                            lhs_node, rhs_node,
                        )));
                    }
                };

                let text = format!("{} = {}", lhs_cnode, rhs_cnode);
                *node = CNode::new(
                    CNodeType::Bare,
                    text,
                    assign.get_type(),
                    assign.get_source(),
                )
                .to_node();
            }

            Node::Access(access) =>
            {
                let target_cnode = match access.get_target()
                {
                    Node::CNode(cnode) => cnode,
                    target_node =>
                    {
                        return ResultLog::new_error(Error::Internal(format!(
                            "Expected access target to be a CNode: {}",
                            target_node,
                        )));
                    }
                };

                let text = format!(
                    "{}.{}",
                    target_cnode.get_text_non_bare(),
                    access.get_property(),
                );
                *node = CNode::new(
                    CNodeType::Bare,
                    text,
                    access.get_type(),
                    access.get_source(),
                )
                .to_node();
            }

            Node::Binding(binding) =>
            {
                let binding_cnode = match binding.get_binding()
                {
                    Node::CNode(cnode) => cnode,
                    binding_node =>
                    {
                        return ResultLog::new_error(Error::Internal(format!(
                            "Expected binding to be a CNode: {}",
                            binding_node,
                        )));
                    }
                };

                let lhs_text = format!(
                    "{} {}",
                    &binding_cnode.borrow_type().get_c_text(),
                    binding.get_name()
                );

                let rhs_text = match binding_cnode.get_node_type()
                {
                    CNodeType::Nothing => String::new(),
                    _ => format!(" = {}", binding_cnode),
                };

                let text = format!("{}{}", lhs_text, rhs_text);
                *node = CNode::new(
                    CNodeType::Bare,
                    text,
                    binding.get_type(),
                    binding.get_source(),
                )
                .to_node();
            }

            Node::Sequence(sequence) =>
            {
                let (start_brace, end_brace, cnode_type) = match sequence.get_mode()
                {
                    SequenceMode::Scope => ("{", "}", CNodeType::Block),
                    SequenceMode::Transparent => ("", "", CNodeType::BlockTransparent),
                };

                let mut text = format!("{}", start_brace);
                for node in sequence.get_nodes()
                {
                    let cnode = match node
                    {
                        Node::CNode(cnode) => cnode,
                        sequence_node =>
                        {
                            return ResultLog::new_error(Error::Internal(format!(
                                "Expected sequence node to be a CNode: {}",
                                sequence_node,
                            )));
                        }
                    };

                    text = match cnode.get_node_type()
                    {
                        CNodeType::Nothing => text,
                        CNodeType::Block
                        | CNodeType::BlockTransparent
                        | CNodeType::Conditional
                        | CNodeType::Function
                        | CNodeType::Comment => format!("{} {}", text, cnode),
                        _ => format!("{} {};", text, cnode),
                    };
                }

                text = format!("{} {}", text, end_brace);
                *node = CNode::new(cnode_type, text, sequence.get_type(), sequence.get_source())
                    .to_node();
            }

            Node::Conditional(conditional) =>
            {
                let (condition_cnode, then_cnode, else_cnode) = match (
                    conditional.get_condition(),
                    conditional.get_then(),
                    conditional.get_else(),
                )
                {
                    (
                        Node::CNode(condition_cnode),
                        Node::CNode(then_cnode),
                        Node::CNode(else_cnode),
                    ) => (condition_cnode, then_cnode, else_cnode),
                    (condition_node, then_node, else_node) =>
                    {
                        return ResultLog::new_error(Error::Internal(format!(
                            "Expected conditional nodes to be CNodes: {}, {}, {}",
                            condition_node, then_node, else_node,
                        )));
                    }
                };

                let condition_text = condition_cnode.get_text_wrapped();
                let then_text = then_cnode.get_text_block();
                let else_text = match else_cnode.get_node_type()
                {
                    CNodeType::Nothing => String::new(),
                    _ => format!(" else {}", else_cnode.get_text_block()),
                };

                let text = format!("if {} {}{}", condition_text, then_text, else_text);
                *node = CNode::new(
                    CNodeType::Conditional,
                    text,
                    conditional.get_type(),
                    conditional.get_source(),
                )
                .to_node();
            }

            Node::Function(function) =>
            {
                let name_text = format!(
                    "{} {}",
                    function.get_return_type().borrow().get_c_text(),
                    function.get_name()
                );

                let mut arguments_text = format!("(");
                for (i, argument) in function.get_arguments().iter().enumerate()
                {
                    arguments_text = format!(
                        "{}{}{} {}",
                        arguments_text,
                        if i == 0 { "" } else { ", " },
                        argument.get_type().borrow().get_c_text(),
                        argument.get_name()
                    );
                }
                arguments_text = format!("{})", arguments_text);

                let body_cnode = match function.get_body()
                {
                    Node::CNode(cnode) => cnode,
                    body_node =>
                    {
                        return ResultLog::new_error(Error::Internal(format!(
                            "Expected function body to be a CNode: {}",
                            body_node,
                        )));
                    }
                };

                let body_text = body_cnode.get_text_block();

                let text = format!("{}{}{}", name_text, arguments_text, body_text);
                *node = CNode::new(
                    CNodeType::Function,
                    text,
                    function.get_type(),
                    function.get_source(),
                )
                .to_node();
            }

            _ =>
            {
                return ResultLog::new_error(Error::Internal(format!("Expected node: {}", node,)));
            }
        }

        ResultLog::Ok(())
    }
}

impl CNode
{
    ///
    /// Wrap a CNode's text in parentheses if it's a bare operator application
    ///
    pub fn get_text_non_bare(&self) -> String
    {
        match self.get_node_type()
        {
            CNodeType::Bare => format!("({})", self.get_text()),
            _ => self.get_text().clone(),
        }
    }

    ///
    /// Wrap a CNode's text in parentheses if it isn't already wrapped
    ///
    pub fn get_text_wrapped(&self) -> String
    {
        match self.get_node_type()
        {
            CNodeType::Wrapped => self.get_text().clone(),
            _ => format!("({})", self.get_text()),
        }
    }

    ///
    /// Wrap a CNode's text in braces if it isn't already a block
    ///
    pub fn get_text_block(&self) -> String
    {
        match self.get_node_type()
        {
            CNodeType::Block => self.get_text().clone(),
            _ => format!("{{ {}; }}", self.get_text()),
        }
    }
}

impl Type
{
    ///
    /// Get the C version of a type
    ///
    pub fn get_c_text(&self) -> String
    {
        format!("{}", self)
    }
}
