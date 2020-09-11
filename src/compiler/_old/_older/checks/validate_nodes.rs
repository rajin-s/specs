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
    InvalidBlock(String),
    InvalidAccessTarget(String),
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
    macro_rules! check {
        ($target:expr, pass : [$( $pass:path ),*,], error : $error:path) => {
            match $target
            {
                Node::Nothing => Some($error(format!("{}", $target))),
                $( $pass(_) => None, )*
                _ => Some($error(format!("{}", $target))),
            }
        };
        ($target:expr, block : [$( $block:path ),*,], error : $error:path) => {
            match $target
            {
                Node::Nothing => Some($error(format!("{}", $target))),
                $( $block(_) => Some($error(format!("{}", $target))), )*
                _ => None,
            }
        };
        ($target:expr, allow_nothing, pass : [$( $pass:path ),*,], error : $error:path) => {
            match $target
            {
                Node::Nothing => None,
                $( $pass(_) => None, )*
                _ => Some($error(format!("{}", $target))),
            }
        };
        ($target:expr, allow_nothing, block : [$( $block:path ),*,], error : $error:path) => {
            match $target
            {
                Node::Nothing => None,
                $( $block(_) => Some($error(format!("{}", $target))), )*
                _ => None,
            }
        };
    }

    use Node::*;
    match node
    {
        Node::Nothing | Node::Integer(_) | Node::Boolean(_) | Node::Variable(_) =>
        {
            return None;
        }

        Node::Call(data) =>
        {
            let operator_error = check!(
                data.get_operator(),
                block : [
                    Integer,
                    Boolean,
                    Reference,
                    Binding,
                    Assignment,
                    Type,
                ],
                error : NodeError::InvalidOperator
            );

            let mut operand_errors: NodeErrors = NodeErrors::new();
            for operand in data.get_operands().iter()
            {
                let operand_error = check!(
                    operand,
                    block : [
                        PrimitiveOperator,
                        Binding,
                        Assignment,
                        Type,
                    ],
                    error : NodeError::InvalidOperand
                );

                if let Some(new_error) = operand_error
                {
                    operand_errors.push(new_error);
                }
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
            let target_error = check!(
                data.get_target(),
                pass : [
                    Variable,
                    Dereference,
                    Sequence,
                    Conditional,
                    Access,
                ],
                error : NodeError::InvalidReference
            );

            match target_error
            {
                None => None,
                Some(error) => Some(vec![error]),
            }
        }
        Node::Dereference(data) =>
        {
            let target_error = check!(
                data.get_target(),
                pass : [
                    Variable,
                    Call,
                    Reference,
                    Dereference,
                    Sequence,
                    Conditional,
                    Access,
                ],
                error : NodeError::InvalidDereference
            );

            match target_error
            {
                None => None,
                Some(error) => Some(vec![error]),
            }
        }

        Node::Binding(data) =>
        {
            let binding_error = check!(
                data.get_binding(),
                block : [
                    PrimitiveOperator,
                    Binding,
                    Assignment,
                    Type,
                ],
                error : NodeError::InvalidBinding
            );

            match binding_error
            {
                None => None,
                Some(error) => Some(vec![error]),
            }
        }
        Node::Assignment(data) =>
        {
            let lhs_error = check!(
                data.get_lhs(),
                pass : [
                    Variable,
                    Dereference,
                    Conditional,
                    Access,
                ],
                error : NodeError::InvalidLHS
            );

            let rhs_error = check!(
                data.get_rhs(),
                block : [
                    PrimitiveOperator,
                    Binding,
                    Assignment,
                    Type,
                ],
                error : NodeError::InvalidBinding
            );

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
            let condition_error = check!(
                data.get_condition(),
                pass : [
                    Boolean,
                    Variable,
                    Call,
                    Dereference,
                    Sequence,
                    Conditional,
                    Access,
                ],
                error : NodeError::InvalidCondition
            );

            let then_error = check!(
                data.get_then(),
                block : [
                    Type,
                ],
                error : NodeError::InvalidBlock
            );

            let else_error = check!(
                data.get_else(),
                allow_nothing,
                block : [
                    Type,
                ],
                error : NodeError::InvalidBlock
            );

            match (&condition_error, &then_error, &else_error)
            {
                (None, None, None) => None,
                _ =>
                {
                    let mut errors: Vec<NodeError> = Vec::new();
                    if let Some(error) = condition_error
                    {
                        errors.push(error);
                    }
                    if let Some(error) = then_error
                    {
                        errors.push(error);
                    }
                    if let Some(error) = else_error
                    {
                        errors.push(error);
                    }

                    Some(errors)
                }
            }
        }

        Node::Function(data) =>
        {
            let body_error = check!(
                data.get_body(),
                block : [
                    Type,
                ],
                error : NodeError::InvalidBlock
            );

            match body_error
            {
                None => None,
                Some(error) => Some(vec![error]),
            }
        }

        Node::Type(data) =>
        {
            let mut method_body_errors: Vec<NodeError> = Vec::new();

            for method in data.get_methods().iter()
            {
                let body_error = check!(
                    method.get_function_data().get_body(),
                    block : [
                        Type,
                    ],
                    error : NodeError::InvalidBlock
                );

                if let Some(error) = body_error
                {
                    method_body_errors.push(error);
                }
            }

            if method_body_errors.is_empty()
            {
                None
            }
            else
            {
                Some(method_body_errors)
            }
        }
        Node::Access(data) =>
        {
            let target_error = check!(
                data.get_target(),
                block : [
                    PrimitiveOperator,
                    Binding,
                    Assignment,
                    Function,
                    Type,
                ],
                error : NodeError::InvalidAccessTarget
            );

            match target_error
            {
                None => None,
                Some(error) => Some(vec![error]),
            }
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
            NodeError::InvalidBlock(message) => write!(f, "Invalid block: {}", message),
            NodeError::InvalidAccessTarget(message) =>
            {
                write!(f, "Invalid access target: {}", message)
            }
        }
    }
}
