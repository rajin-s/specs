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

                // Memory operators
                PrimitiveOperator::Create => write!(f, "create"),
                PrimitiveOperator::HeapAllocate => write!(f, "heap-allocate"),
                PrimitiveOperator::HeapFree => write!(f, "heap-free"),
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

            Node::Function(data) =>
            {
                write!(f, "<fn {} ", data.get_name());
                for argument in data.get_arguments().iter()
                {
                    write!(f, "[{} {}] ", argument.get_name(), argument.get_type());
                }
                write!(f, "-> {} {}>", data.get_return_type(), data.get_body())
            }

            Node::Type(data) =>
            {
                write!(f, "(type {} {{ ", data.get_name());

                write!(f, "<data {{ ");
                for member in data.get_members().iter()
                {
                    let scope_prefix = match member.get_scope()
                    {
                        MemberScope::Instance => "self.",
                        MemberScope::Static => "",
                    };
                    let read_visibility = match member.get_read_visibility()
                    {
                        Visibility::Private => "private",
                        Visibility::Public => "public",
                    };
                    let write_visibility = match member.get_write_visibility()
                    {
                        Visibility::Private => "private",
                        Visibility::Public => "public",
                    };

                    write!(
                        f,
                        "[{}{} {} ({} read / {} write)] ",
                        scope_prefix,
                        member.get_name(),
                        member.get_type(),
                        read_visibility,
                        write_visibility
                    );
                }
                write!(f, "}}> ");
                write!(f, "<methods {{ ");
                for method in data.get_methods().iter()
                {
                    let scope_prefix = match method.get_scope()
                    {
                        MemberScope::Instance => "self.",
                        MemberScope::Static => "",
                    };
                    let visibility = match method.get_visibility()
                    {
                        Visibility::Private => "private",
                        Visibility::Public => "public",
                    };

                    let function = method.get_function_data();

                    write!(
                        f,
                        "<{} fn {}{} ",
                        visibility,
                        scope_prefix,
                        function.get_name()
                    );
                    for argument in function.get_arguments().iter()
                    {
                        write!(f, "[{} {}] ", argument.get_name(), argument.get_type());
                    }
                    write!(
                        f,
                        "-> {} {}> ",
                        function.get_return_type(),
                        function.get_body()
                    );
                }
                write!(f, "}}>");

                write!(f, " }})")
            }
            Node::Access(data) => write!(f, "({} . {})", data.get_target(), data.get_property()),
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
            DataType::Function(data) =>
            {
                write!(f, "(");
                for argument_type in data.get_argument_types().iter()
                {
                    write!(f, "{} ", argument_type);
                }
                write!(f, "-> {})", data.get_return_type())
            }

            DataType::Type(data) =>
            {
                write!(f, "[type {}]", data.get_name())
                // match data.get_name()
                // {
                //     Some(name) =>
                //     {
                //         write!(f, "(type {}", name);
                //     }
                //     None =>
                //     {
                //         write!(f, "(type");
                //     }
                // }

                // if !data.get_traits().is_empty()
                // {
                //     write!(f, " is {{ ");
                //     for t in data.get_traits().iter()
                //     {
                //         write!(f, "{} ", t.get_name());
                //     }
                //     write!(f, "}}");
                // }

                // write!(f, " {} members)", data.get_members().len())
            }
            DataType::Instance(data) => write!(f, "{}", data.get_name()),
        }
    }
}
