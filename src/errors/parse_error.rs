pub use super::*;

const SHOW_INTERNAL_ERRORS: bool = false;

pub enum Error
{
    UnknownExpression(String, Source),
    UnknownSymbol(String, Source),

    UnexpectedKeyword(String, Source),

    BadFunctionName(String, Source),
    BadFunctionLayout(String, Source),
    BadFunctionArgument(String, Source),

    Internal(String),
}

impl ErrorTrait for Error
{
    fn get_source(&self) -> Option<&Source>
    {
        match self
        {
            Error::UnknownExpression(_, source) => Some(source),
            Error::UnknownSymbol(_, source) => Some(source),

            Error::UnexpectedKeyword(_, source) => Some(source),

            Error::BadFunctionName(_, source) => Some(source),
            Error::BadFunctionLayout(_, source) => Some(source),
            Error::BadFunctionArgument(_, source) => Some(source),

            Error::Internal(..) => None,
        }
    }
    fn get_description(&self) -> Option<&str>
    {
        match self
        {
            Error::UnknownExpression(description, _) => Some(description),
            Error::UnknownSymbol(..) => None,

            Error::UnexpectedKeyword(..) => None,

            Error::BadFunctionName(description, _) => Some(description),
            Error::BadFunctionLayout(description, _) => Some(description),
            Error::BadFunctionArgument(description, _) => Some(description),

            Error::Internal(..) => None,
        }
    }
    fn show(&self) -> bool
    {
        match self
        {
            Error::Internal(..) => SHOW_INTERNAL_ERRORS,
            _ => true,
        }
    }
}

use std::fmt;

impl fmt::Display for Error
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        match &self
        {
            Error::UnknownExpression(..) => write!(f, "Unknown expression",),
            Error::UnknownSymbol(symbol, _) => write!(f, "Unknown symbol '{}'", symbol),

            Error::UnexpectedKeyword(symbol, _) => write!(f, "Unexpected keyword '{}'", symbol),

            Error::BadFunctionName(..) => write!(f, "Bad function name"),
            Error::BadFunctionLayout(..) => write!(f, "Bad function layout"),
            Error::BadFunctionArgument(..) => write!(f, "Bad function argument"),

            Error::Internal(message) => write!(f, "Internal '{}'", message),
        }
    }
}
