use crate::language::nodes::*;
use std::fmt;

pub enum TypeError
{
    UnknownSymbolType(String),

    InvalidOperatorType(Type),
    InvalidArgumentType(usize, Type, Type),
    InvalidArgumentCount(usize, usize),

    InvalidDereference(Type),

    InvalidBindingType(String, Type),
    InvalidAssignment(Type, Type),

    InvalidConditionType(Type),
    NonmatchingBranchTypes(Type, Type),
}
type TypeErrors = Vec<TypeError>;

pub fn apply(root: &Node) -> Option<TypeErrors>
{
    let mut errors = TypeErrors::new();
    root.parse_recursive(check_type, &mut errors);

    if errors.is_empty()
    {
        None
    }
    else
    {
        Some(errors)
    }
}

fn check_type(node: &Node, errors: &mut TypeErrors)
{
    let mut new_errors: Vec<TypeError> = Vec::new();

    match node
    {
        Node::Nothing | Node::Integer(_) | Node::Boolean(_) =>
        {}
        Node::Variable(data) =>
        {
            // Ensure that all variables have types
            if data.get_type().is_unknown()
            {
                new_errors.push(TypeError::UnknownSymbolType(data.get_name().clone()));
            }
        }

        Node::Call(data) =>
        {
            // Ensure that all calls have
            //  - callable operator types
            //  - correct argument types
            let operator_type = data.get_operator().get_type();

            if let DataType::Callable(callable_data) = operator_type.get_data_type()
            {
                let operands = data.get_operands();
                let argument_types = callable_data.get_argument_types();

                // Make sure the number of arguments is correct
                if argument_types.len() == operands.len()
                {
                    // Make sure each argument type is correct
                    for (i, argument_type) in argument_types.iter().enumerate()
                    {
                        if argument_type != operands[i].get_type()
                        {
                            new_errors.push(TypeError::InvalidArgumentType(
                                i,
                                argument_type.clone(),
                                operands[i].get_type().clone(),
                            ));
                        }
                    }
                }
                else
                {
                    new_errors.push(TypeError::InvalidArgumentCount(
                        argument_types.len(),
                        operands.len(),
                    ));
                }
            }
            else
            {
                new_errors.push(TypeError::InvalidOperatorType(operator_type.clone()));
            }
        }
        Node::PrimitiveOperator(_) =>
        {}

        Node::Reference(_) =>
        {}
        Node::Dereference(data) =>
        {
            if data.get_target().get_type().is_value()
            {
                new_errors.push(TypeError::InvalidDereference(
                    data.get_target().get_type().clone(),
                ));
            }
        }

        Node::Binding(data) =>
        {
            // Ensure that all variable bindings have types
            if data.get_binding().get_type().is_unknown()
            {
                new_errors.push(TypeError::InvalidBindingType(
                    data.get_name().clone(),
                    data.get_binding().get_type().clone(),
                ));
            }
        }
        Node::Assignment(data) =>
        {
            // Ensure the LHS and RHS have the same types
            if data.get_lhs().get_type() != data.get_rhs().get_type()
            {
                new_errors.push(TypeError::InvalidAssignment(
                    data.get_lhs().get_type().clone(),
                    data.get_rhs().get_type().clone(),
                ))
            }
        }

        Node::Sequence(_) =>
        {}
        Node::Conditional(data) =>
        {
            const BOOLEAN_TYPE: Type = Type::new_constant(DataType::Boolean);
            if data.get_condition().get_type() != &BOOLEAN_TYPE
            {
                new_errors.push(TypeError::InvalidConditionType(
                    data.get_condition().get_type().clone(),
                ));
            }

            if data.has_else()
            {
                if data.get_else().get_type() != data.get_then().get_type()
                {
                    new_errors.push(TypeError::NonmatchingBranchTypes(
                        data.get_then().get_type().clone(),
                        data.get_else().get_type().clone(),
                    ));
                }
            }
        }
    }

    errors.append(&mut new_errors);
}

impl fmt::Display for TypeError
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        use TypeError::*;

        match self
        {
            UnknownSymbolType(symbol) => write!(f, "Unknown symbol '{}'", symbol),

            InvalidOperatorType(t) => write!(f, "Invalid operator type {}", t),
            InvalidArgumentType(index, expected, found) => write!(
                f,
                "Invalid argument type (index={}): expected {}, found {}",
                index, expected, found
            ),
            InvalidArgumentCount(expected, found) => write!(
                f,
                "Invalid argument count: expected {}, found {}",
                expected, found
            ),

            InvalidDereference(t) =>
            {
                write!(f, "Invalid dereference: can't dereference value-type {}", t)
            }

            InvalidBindingType(name, t) =>
            {
                write!(f, "Invalid type for binding '{}': found {}", name, t)
            }
            InvalidAssignment(lhs, rhs) =>
            {
                write!(f, "Invalid assignment: assigning {} to target {}", rhs, lhs)
            }

            InvalidConditionType(t) => write!(f, "Invalid condition type: found {}", t),
            NonmatchingBranchTypes(then_t, else_t) => write!(
                f,
                "Else branch doesn't match then branch: expected {}, found {}",
                then_t, else_t
            ),
        }
    }
}
