use crate::compiler::internal::*;

/* Pass: specs_c::convert
    - Converts the whole tree into a CNode by recursively converting all nodes into CNodes and formatting C text
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

impl CompilerPass<State> for Pass
{
    // Modify a node given some state
    //  - All child nodes have already been transformed at this point
    fn transform(
        &mut self,
        node: &mut Node,
        _state: Indirect<State>,
        messages: &mut PassMessageContext,
    )
    {
        use crate::language::node::internal::{CNode, CNodeType};

        fn get_text_non_bare(c_node: &CNode) -> String
        {
            match c_node.get_node_type()
            {
                CNodeType::Bare => format!("({})", c_node.get_text()),
                _ => format!("{}", c_node.get_text()),
            }
        }

        fn get_text_wrapped(c_node: &CNode) -> String
        {
            match c_node.get_node_type()
            {
                CNodeType::Wrapped => format!("{}", c_node.get_text()),
                _ => format!("({})", c_node.get_text()),
            }
        }
        fn get_text_block(c_node: &CNode) -> String
        {
            match c_node.get_node_type()
            {
                CNodeType::Block => format!("{}", c_node.get_text()),
                _ => format!("{{ {}; }}", c_node.get_text()),
            }
        }

        fn get_text_type(t: &types::Type) -> String
        {
            return format!("{}", t);
        }

        match node
        {
            Node::Comment(comment) =>
            {
                let text = format!("/* {} */", comment.get_content());
                *node = CNode::new(CNodeType::Comment, text, comment.get_type()).to_node();
            }
            Node::Integer(integer) =>
            {
                let text = format!("{}", integer.get_value());
                *node = CNode::new(CNodeType::Atomic, text, integer.get_type()).to_node();
            }
            Node::Boolean(boolean) =>
            {
                let text = format!("{}", boolean.get_value());
                *node = CNode::new(CNodeType::Atomic, text, boolean.get_type()).to_node();
            }
            Node::Variable(variable) =>
            {
                let text = format!("{}", variable.get_name());
                *node = CNode::new(CNodeType::Atomic, text, variable.get_type()).to_node();
            }
            Node::Nothing(nothing) =>
            {
                *node = CNode::new(CNodeType::Nothing, String::new(), nothing.get_type()).to_node();
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
                    primitive::Operator::Return => (CNodeType::Atomic, "return"),

                    // Anything else should have been converted already
                    _ => (CNodeType::Atomic, "unknown_operator"),
                };

                *node = CNode::new(node_type, text.to_owned(), operator.get_type()).to_node();
            }
            Node::Call(call) =>
            {
                if let Node::CNode(c_node) = call.get_operator()
                {
                    match c_node.get_node_type()
                    {
                        CNodeType::InfixOperator if call.get_operands().len() == 2 =>
                        {
                            let operands = call.get_operands();
                            let (a_cnode, b_cnode) = match (&operands[0], &operands[1])
                            {
                                (Node::CNode(a_cnode), Node::CNode(b_cnode)) => (a_cnode, b_cnode),
                                _ =>
                                {
                                    messages.add_error(
                                        "InvalidNode",
                                        "Expected call operands to be CNodes",
                                        node,
                                    );
                                    return;
                                }
                            };

                            // Make sure the operands aren't bare operator applications
                            //  (potentially messing with order of operations)
                            let text = format!(
                                "{} {} {}",
                                get_text_non_bare(a_cnode),
                                call.get_operator(),
                                get_text_non_bare(b_cnode)
                            );

                            *node = CNode::new(CNodeType::Bare, text, call.get_type()).to_node();
                        }
                        CNodeType::PrefixOperator | CNodeType::InfixOperator =>
                        {
                            let a_cnode = match &call.get_operands()[0]
                            {
                                Node::CNode(a_node) => a_node,
                                _ =>
                                {
                                    messages.add_error(
                                        "InvalidNode",
                                        "Expected call operand to be CNodes",
                                        node,
                                    );
                                    return;
                                }
                            };

                            // Make sure the operand isn't a bare operator application
                            let text =
                                format!("{} {}", call.get_operator(), get_text_non_bare(a_cnode));

                            *node = CNode::new(CNodeType::Bare, text, call.get_type()).to_node();
                        }
                        _ =>
                        {
                            let mut text = format!("{}(", call.get_operator());

                            for (i, operand) in call.get_operands().iter().enumerate()
                            {
                                let operand_cnode = match operand
                                {
                                    Node::CNode(cnode) => cnode,
                                    _ =>
                                    {
                                        messages.add_error(
                                            "InvalidNode",
                                            "Expected call operands to be CNodes",
                                            node,
                                        );
                                        return;
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
                            *node = CNode::new(CNodeType::Call, text, call.get_type()).to_node();
                        }
                    }
                }
                else
                {
                    messages.add_error("InvalidNode", "Expected call operator to be a CNode", node);
                }
            }
            Node::Reference(reference) =>
            {
                let target_cnode = match reference.get_target()
                {
                    Node::CNode(cnode) => cnode,
                    _ =>
                    {
                        messages.add_error(
                            "InvalidNode",
                            "Expected reference target to be CNode",
                            node,
                        );
                        return;
                    }
                };

                let text = format!("&{}", get_text_non_bare(target_cnode));
                *node = CNode::new(CNodeType::Bare, text, reference.get_type()).to_node();
            }
            Node::Dereference(dereference) =>
            {
                let target_cnode = match dereference.get_target()
                {
                    Node::CNode(cnode) => cnode,
                    _ =>
                    {
                        messages.add_error(
                            "InvalidNode",
                            "Expected dereference target to be a CNode",
                            node,
                        );
                        return;
                    }
                };

                let text = format!("&{}", get_text_non_bare(target_cnode));
                *node = CNode::new(CNodeType::Bare, text, dereference.get_type()).to_node();
            }
            Node::Assign(assign) =>
            {
                let (lhs_cnode, rhs_cnode) = match (assign.get_lhs(), assign.get_rhs())
                {
                    (Node::CNode(lhs_cnode), Node::CNode(rhs_cnode)) => (lhs_cnode, rhs_cnode),
                    _ =>
                    {
                        messages.add_error(
                            "InvalidNode",
                            "Expected assign lhs and rhs to be CNodes",
                            node,
                        );
                        return;
                    }
                };
                let text = format!("{} = {}", lhs_cnode, rhs_cnode);
                *node = CNode::new(CNodeType::Bare, text, assign.get_type()).to_node();
            }
            Node::Access(access) =>
            {
                let target_cnode = match access.get_target()
                {
                    Node::CNode(cnode) => cnode,
                    _ =>
                    {
                        messages.add_error(
                            "InvalidNode",
                            "Expected access target to be a CNode",
                            node,
                        );
                        return;
                    }
                };

                let text = format!(
                    "{}.{}",
                    get_text_non_bare(target_cnode),
                    access.get_property()
                );
                *node = CNode::new(CNodeType::Bare, text, access.get_type()).to_node();
            }
            Node::Binding(binding) =>
            {
                let binding_cnode = match binding.get_binding()
                {
                    Node::CNode(cnode) => cnode,
                    _ =>
                    {
                        messages.add_error("InvalidNode", "Expected binding to be a CNode", node);
                        return;
                    }
                };

                let lhs_text = format!(
                    "{} {}",
                    get_text_type(&binding_cnode.borrow_type()),
                    binding.get_name()
                );

                let rhs_text = match binding_cnode.get_node_type()
                {
                    CNodeType::Nothing => String::new(),
                    _ => format!(" = {}", binding_cnode),
                };

                let text = format!("{}{}", lhs_text, rhs_text);
                *node = CNode::new(CNodeType::Bare, text, binding.get_type()).to_node();
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
                        _ =>
                        {
                            messages.add_error(
                                "InvalidNode",
                                "Expected sequence node to be a CNode",
                                node,
                            );
                            return;
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
                *node = CNode::new(cnode_type, text, sequence.get_type()).to_node();
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
                    _ =>
                    {
                        messages.add_error(
                            "InvalidNode",
                            "Expected conditional nodes to be CNodes",
                            node,
                        );
                        return;
                    }
                };

                let condition_text = get_text_wrapped(condition_cnode);
                let then_text = get_text_block(then_cnode);
                let else_text = match else_cnode.get_node_type()
                {
                    CNodeType::Nothing => String::new(),
                    _ => format!(" else {}", get_text_block(else_cnode)),
                };

                let text = format!("if {} {}{}", condition_text, then_text, else_text);
                *node = CNode::new(CNodeType::Conditional, text, conditional.get_type()).to_node();
            }
            Node::Function(function) =>
            {
                let name_text = format!(
                    "{} {}",
                    get_text_type(&function.get_return_type().borrow()),
                    function.get_name()
                );

                let mut arguments_text = format!("(");
                for (i, argument) in function.get_arguments().iter().enumerate()
                {
                    arguments_text = format!(
                        "{}{}{} {}",
                        arguments_text,
                        if i == 0 { "" } else { ", " },
                        get_text_type(&argument.get_type().borrow()),
                        argument.get_name()
                    );
                }
                arguments_text = format!("{})", arguments_text);

                let body_cnode = match function.get_body()
                {
                    Node::CNode(cnode) => cnode,
                    _ =>
                    {
                        messages.add_error(
                            "InvalidNode",
                            "Expected function body to be a CNode",
                            node,
                        );
                        return;
                    }
                };

                let body_text = get_text_block(body_cnode);

                let text = format!("{}{}{}", name_text, arguments_text, body_text);
                *node = CNode::new(CNodeType::Function, text, function.get_type()).to_node();
            }
            Node::Class(_) =>
            {
                unimplemented!();
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
        vec![parent.clone()]
    }

    // Get the name of the pass (for debugging)
    fn get_name(&self) -> String
    {
        "ConvertC".to_owned()
    }
}

/* -------------------------------------------------------------------------- */
/*                                    State                                   */
/* -------------------------------------------------------------------------- */

impl State {}
