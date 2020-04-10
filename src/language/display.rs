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
            Node::Variable(data) => write!(f, "[var {}]", data.get_name()),

            Node::PrimitiveOperator(data) => match data.get_operator()
            {
                PrimitiveOperator::Add => write!(f, "+"),
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
        }
    }
}

impl fmt::Display for Type
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        match self.get_data_type()
        {
            DataType::Unknown => write!(f, "unknown"),
            DataType::Void => write!(f, "void"),
            DataType::Integer => write!(f, "int"),
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
