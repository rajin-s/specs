#![allow(unused_must_use)]

use super::nodes::*;
use std::fmt;

impl fmt::Display for Node
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        match self
        {
            Node::Nothing => write!(f, "nothing"),
            Node::Integer(data) => write!(f, "[int {}]", data.get_value()),
            Node::Boolean(data) => write!(f, "[bool {}]", data.get_value()),
            Node::Variable(data) => write!(f, "[var {}]", data.get_name()),

            Node::PrimitiveOperator(data) => match data.get_operator()
            {
                // Arithmetic operators
                PrimitiveOperator::Add => write!(f, "+"),

                // Comparison operators
                PrimitiveOperator::Equal => write!(f, "=="),
                PrimitiveOperator::NotEqual => write!(f, "=/="),
                PrimitiveOperator::Less => write!(f, "<"),
                PrimitiveOperator::Greater => write!(f, ">"),
                PrimitiveOperator::LessEqual => write!(f, "<="),
                PrimitiveOperator::GreaterEqual => write!(f, ">="),

                // Logical operators
                PrimitiveOperator::And => write!(f, "and"),
                PrimitiveOperator::Or => write!(f, "or"),
                PrimitiveOperator::ExclusiveOr => write!(f, "xor"),
            },
            Node::Call(data) =>
            {
                write!(f, "({}", data.get_operator());
                for operand in data.get_operands().iter()
                {
                    write!(f, " {}", operand);
                }
                write!(f, ")")
            }

            Node::Reference(data) => match data.get_reference_type()
            {
                Reference::Immutable => write!(f, "(ref {})", data.get_target()),
                Reference::Mutable => write!(f, "(mut-ref {})", data.get_target()),
            },
            Node::Dereference(data) => write!(f, "(deref {})", data.get_target()),

            Node::Binding(data) => write!(f, "(let {} = {})", data.get_name(), data.get_binding()),
            Node::Assignment(data) => write!(f, "({} := {})", data.get_lhs(), data.get_rhs()),

            Node::Sequence(data) =>
            {
                write!(f, "{{");
                for (i, operand) in data.get_nodes().iter().enumerate()
                {
                    if i == 0
                    {
                        write!(f, "{}", operand);
                    }
                    else
                    {
                        write!(f, " {}", operand);
                    }
                }
                write!(f, "}}")
            }
            Node::Conditional(data) =>
            {
                if data.has_else()
                {
                    write!(
                        f,
                        "(if {} then {} else {})",
                        data.get_condition(),
                        data.get_then(),
                        data.get_else()
                    )
                }
                else
                {
                    write!(f, "(if {} then {})", data.get_condition(), data.get_then())
                }
            }
        }
    }
}

impl fmt::Display for Type
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        for layer in self.get_reference_layers().iter().rev()
        {
            match layer
            {
                Reference::Immutable =>
                {
                    write!(f, "&");
                }
                Reference::Mutable =>
                {
                    write!(f, "&mut ");
                }
            }
        }

        match self.get_data_type()
        {
            DataType::Unknown => write!(f, "Unknown"),
            DataType::Void => write!(f, "Void"),
            DataType::Integer => write!(f, "Int"),
            DataType::Boolean => write!(f, "Bool"),
            DataType::Callable(data) =>
            {
                write!(f, "(");
                for argument_type in data.get_argument_types().iter()
                {
                    write!(f, "{} ", argument_type);
                }
                write!(f, "-> {})", data.get_return_type())
            }
        }
    }
}
