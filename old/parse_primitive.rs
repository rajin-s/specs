use crate::lang_rml::nodes::*;
use crate::lang_rml::symbols;

use crate::lang_sexp::sexp::*;

pub fn parse_boolean(symbol: &String) -> Option<Node>
{
    if symbol == symbols::constants::TRUE
    {
        return Some(Node::Boolean(true));
    }
    else if symbol == symbols::constants::FALSE
    {
        return Some(Node::Boolean(false));
    }

    return None;
}

pub fn parse_integer(symbol: &String) -> Option<Node>
{
    // ex. 123, -999
    if let Ok(value) = symbol.parse::<i64>()
    {
        return Some(Node::Integer(value));
    }

    return None;
}

pub fn parse_float(symbol: &String) -> Option<Node>
{
    if let Ok(value) = symbol.parse::<f64>()
    {
        return Some(Node::Float(value));
    }

    return None;
}

pub fn parse_long_string(symbol: &String) -> Option<Node>
{
    // ex. "hello world"
    if symbol.starts_with(symbols::delimiters::STRING)
        && symbol.ends_with(symbols::delimiters::STRING)
        && symbol.len() > 1
    {
        let content = String::from(&symbol[1..symbol.len() - 1]);
        return Some(Node::LongString(content));
    }

    return None;
}

pub fn parse_primitive_operator(expression: &SExpression, operand_count: usize) -> Option<Node>
{
    use symbols::operators::*;
    use PrimitiveOperator::*;

    match expression
    {
        SExpression::List(_, _) => None,
        SExpression::Symbol(symbol) =>
        {
            if operand_count == 1
            {
                match symbol.as_str()
                {
                    NOT => Some(Node::Primitive(Primitive::Operator(Not))),
                    REFERENCE => Some(Node::Primitive(Primitive::Operator(Reference))),
                    MUTABLE_REFERENCE =>
                    {
                        Some(Node::Primitive(Primitive::Operator(MutableReference)))
                    }
                    DEREFERENCE => Some(Node::Primitive(Primitive::Operator(Dereference))),
                    _ => None,
                }
            }
            else
            {
                match symbol.as_str()
                {
                    ACCESS => Some(Node::Primitive(Primitive::Operator(Access))),
                    ASSIGN => Some(Node::Primitive(Primitive::Operator(Assign))),
                    INDEX => Some(Node::Primitive(Primitive::Operator(Index))),
                    PLUS => Some(Node::Primitive(Primitive::Operator(Plus))),
                    MINUS => Some(Node::Primitive(Primitive::Operator(Minus))),
                    TIMES => Some(Node::Primitive(Primitive::Operator(Times))),
                    DIVIDE => Some(Node::Primitive(Primitive::Operator(Divide))),
                    MODULO => Some(Node::Primitive(Primitive::Operator(Modulo))),
                    POW => Some(Node::Primitive(Primitive::Operator(Power))),
                    EQUAL => Some(Node::Primitive(Primitive::Operator(Equal))),
                    NOT_EQUAL => Some(Node::Primitive(Primitive::Operator(NotEqual))),
                    LESS => Some(Node::Primitive(Primitive::Operator(Less))),
                    GREATER => Some(Node::Primitive(Primitive::Operator(Greater))),
                    LESS_EQUAL => Some(Node::Primitive(Primitive::Operator(LessEqual))),
                    GREATER_EQUAL => Some(Node::Primitive(Primitive::Operator(GreaterEqual))),
                    AND => Some(Node::Primitive(Primitive::Operator(And))),
                    OR => Some(Node::Primitive(Primitive::Operator(Or))),
                    XOR => Some(Node::Primitive(Primitive::Operator(Xor))),
                    _ => None,
                }
            }
        }
    }
}
