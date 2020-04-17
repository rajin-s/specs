use crate::language::nodes::*;
use std::fmt;

pub enum NodeError
{
    InvalidOperator(String),
    InvalidOperand(String),
    InvalidReference(String),
    InvalidDereference(String),
    InvalidBinding(String),
    InvalidLHS(String),
    InvalidRHS(String),
    EmptySequence,
    InvalidCondition(String),
}
type NodeErrors = Vec<NodeError>;

pub fn apply(root: &Node) -> Option<NodeErrors>
{
    let mut errors = NodeErrors::new();
    check(root, &mut errors);

    if errors.is_empty()
    {
        None
    }
    else
    {
        Some(errors)
    }
}

// Accumulate errors from checking a node
fn check(node: &Node, errors: &mut NodeErrors)
{
    node.recur_parse(check, errors);
    if let Some(mut new_errors) = check_node(node)
    {
        errors.append(&mut new_errors);
    }
}

// Check if a node is an atomic value or variable
fn is_atomic(node: &Node) -> bool
{
    match node
    {
        Node::Integer(_) | Node::Boolean(_) | Node::Variable(_) => true,
        _ => false,
    }
}

// Check that a nodes structural constraints are met
fn check_node(node: &Node) -> Option<NodeErrors>
{
    match node
    {
        Node::Nothing | Node::Integer(_) | Node::Boolean(_) | Node::Variable(_) =>
        {
            return None;
        }

        Node::Call(data) =>
        {
            // Operator can be
            //  - Variable
            //  - Call
            //  - PrimitiveOperator
            //  - Dereference
            //  - Conditional
            let operator_error = match data.get_operator()
            {
                Node::Variable(_)
                | Node::Call(_)
                | Node::PrimitiveOperator(_)
                | Node::Dereference(_)
                | Node::Conditional(_) => None,
                _ => Some(NodeError::InvalidOperator(format!(
                    "{}",
                    data.get_operator()
                ))),
            };

            // Operands can be
            //  - Any atomic
            //  - Call
            //  - Reference / Dereference
            //  - Conditional
            let mut operand_errors: NodeErrors = NodeErrors::new();
            for operand in data.get_operands().iter()
            {
                let operand_errors = match operand
                {
                    Node::Call(_)
                    | Node::Reference(_)
                    | Node::Dereference(_)
                    | Node::Conditional(_) =>
                    {}
                    n if is_atomic(n) =>
                    {}
                    _ =>
                    {
                        operand_errors.push(NodeError::InvalidOperand(format!("{}", operand)));
                    }
                };
            }

            match (operator_error, operand_errors.is_empty())
            {
                (None, true) => None,
                (None, false) => Some(operand_errors),
                (Some(error), true) => Some(vec![error]),
                (Some(error), false) =>
                {
                    let mut errors = vec![error];
                    errors.append(&mut operand_errors);
                    Some(errors)
                }
            }
        }
        Node::PrimitiveOperator(_) =>
        {
            return None;
        }

        Node::Reference(data) =>
        {
            // Target can be
            //  - Variable
            //  - Dereference
            //  - Conditional
            let target_error = match data.get_target()
            {
                Node::Variable(_) | Node::Dereference(_) | Node::Conditional(_) => None,
                _ => Some(NodeError::InvalidReference(format!(
                    "{}",
                    data.get_target()
                ))),
            };

            match target_error
            {
                None => None,
                Some(error) => Some(vec![error]),
            }
        }
        Node::Dereference(data) =>
        {
            // Target can be
            //  - Variable
            //  - Call
            //  - Reference
            //  - Dereference
            //  - Conditional
            let target_error = match data.get_target()
            {
                Node::Variable(_)
                | Node::Call(_)
                | Node::Reference(_)
                | Node::Dereference(_)
                | Node::Conditional(_) => None,
                _ => Some(NodeError::InvalidDereference(format!(
                    "{}",
                    data.get_target()
                ))),
            };

            match target_error
            {
                None => None,
                Some(error) => Some(vec![error]),
            }
        }

        Node::Binding(data) =>
        {
            // Binding can be
            //  - Nothing (for bindings introduced by passes)
            //  - Any atomic
            //  - Call
            //  - Reference
            //  - Dereference
            //  - Conditional
            let binding_error = match data.get_binding()
            {
                Node::Nothing
                | Node::Call(_)
                | Node::Reference(_)
                | Node::Dereference(_)
                | Node::Conditional(_) => None,
                n if is_atomic(n) => None,
                _ => Some(NodeError::InvalidBinding(format!("{}", data.get_binding()))),
            };

            match binding_error
            {
                None => None,
                Some(error) => Some(vec![error]),
            }
        }
        Node::Assignment(data) =>
        {
            // LHS can be
            //  - Variable
            //  - Dereference
            //  - Conditional
            let lhs_error = match data.get_lhs()
            {
                Node::Variable(_) | Node::Dereference(_) | Node::Conditional(_) => None,
                _ => Some(NodeError::InvalidLHS(format!("{}", data.get_lhs()))),
            };

            // RHS can be
            //  - Any atomic
            //  - Call
            //  - Reference
            //  - Dereference
            //  - Conditional
            let rhs_error = match data.get_rhs()
            {
                Node::Call(_)
                | Node::Reference(_)
                | Node::Dereference(_)
                | Node::Conditional(_) => None,
                n if is_atomic(n) => None,
                _ => Some(NodeError::InvalidRHS(format!("{}", data.get_rhs()))),
            };

            match (lhs_error, rhs_error)
            {
                (None, None) => None,
                (None, Some(error)) => Some(vec![error]),
                (Some(error), None) => Some(vec![error]),
                (Some(lhs_error), Some(rhs_error)) => Some(vec![lhs_error, rhs_error]),
            }
        }

        Node::Sequence(data) =>
        {
            // Final node can be
            //  - Anything
            let final_node_error = match data.get_final_node()
            {
                Some(_) => None,
                None => Some(NodeError::EmptySequence),
            };

            match final_node_error
            {
                None => None,
                Some(error) => Some(vec![error]),
            }
        }
        Node::Conditional(data) =>
        {
            // Condition can be
            //  - Any atomic
            //  - Call
            //  - Dereference
            //  - Conditional
            let condition_error = match data.get_condition()
            {
                Node::Call(_) | Node::Dereference(_) | Node::Conditional(_) => None,
                n if is_atomic(n) => None,
                _ => Some(NodeError::InvalidCondition(format!(
                    "{}",
                    data.get_condition()
                ))),
            };

            // Then can be
            //  - Anything
            // Else can be
            //  - Anything
            match condition_error
            {
                None => None,
                Some(error) => Some(vec![error]),
            }
        }

        Node::Function(_data) =>
        {
            // Function body can be
            //  - Anything
            None
        }
    }
}

impl fmt::Display for NodeError
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        match self
        {
            NodeError::InvalidOperator(message) => write!(f, "Invalid operator node: {}", message),
            NodeError::InvalidOperand(message) => write!(f, "Invalid operand node: {}", message),
            NodeError::InvalidReference(message) =>
            {
                write!(f, "Invalid reference target: {}", message)
            }
            NodeError::InvalidDereference(message) =>
            {
                write!(f, "Invalid dereference target: {}", message)
            }
            NodeError::InvalidBinding(message) => write!(f, "Invalid binding: {}", message),
            NodeError::InvalidLHS(message) => write!(f, "Invalid assignment lhs: {}", message),
            NodeError::InvalidRHS(message) => write!(f, "Invalid assignment rhs: {}", message),
            NodeError::EmptySequence => write!(f, "Empty sequence"),
            NodeError::InvalidCondition(message) =>
            {
                write!(f, "Invalid branch condition: {}", message)
            }
        }
    }
}
