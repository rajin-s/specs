use super::s_expression::SExpression;
use std::fmt;

pub type ParseErrorList = Vec<ParseError>;

pub enum ParseError
{
    Internal(String),
    InvalidSymbol(String),
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
            InvalidSExpression(expression) => write!(f, "Invalid S-Expression: {}", expression),
        }
    }
}
