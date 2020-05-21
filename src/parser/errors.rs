use super::s_expression::SExpression;
use std::fmt;

pub type ParseErrorList = Vec<ParseError>;

pub enum ParseError
{
    Internal(String),
    InvalidSymbol(String),
    InvalidType(SExpression),
    InvalidWhenBranch(SExpression),
    InvalidFunctionName(String),
    InvalidFunctionBody(String, String),
    InvalidFunctionArgument(SExpression),
    InvalidTypeExpression(String, String, SExpression),
    InvalidMemberExpression(SExpression),
    InvalidSExpression(SExpression),
}

impl fmt::Display for ParseError
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        use ParseError::*;

        match self
        {
            Internal(message) => write!(f, "Internal error: {}", message),
            InvalidSymbol(symbol) => write!(f, "Invalid symbol: '{}'", symbol),
            InvalidType(expression) => write!(f, "Invalid type expression: '{}'", expression),
            InvalidWhenBranch(expression) => write!(f, "Invalid when branch: '{}'", expression),
            InvalidFunctionName(name) =>
            {
                write!(f, "Invalid function name '{}'", name)
            }
            InvalidFunctionBody(name, message) =>
            {
                write!(f, "Invalid body for function '{}': {}", name, message)
            }
            InvalidFunctionArgument(expression) =>
            {
                write!(f, "Invalid function argument: {}", expression)
            }
            InvalidTypeExpression(name, message, expression) => write!(
                f,
                "Invalid definition expression for type '{}': {} ({})",
                name, expression, message
            ),
            InvalidMemberExpression(expression) =>
            {
                write!(f, "Invalid member expression: {}", expression)
            }
            InvalidSExpression(expression) => write!(f, "Invalid S-Expression: {}", expression),
        }
    }
}
