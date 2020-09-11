pub use super::*;

pub enum Error
{
    Internal(String),
    UnclosedBracket(String, Source),
    FailedToParse(Source),
}

impl ErrorTrait for Error
{
    fn get_source(&self) -> Option<&Source>
    {
        match self
        {
            Error::Internal(_) => None,
            Error::UnclosedBracket(_, source) => Some(source),
            Error::FailedToParse(source) => Some(source),
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
            Error::Internal(message) => write!(f, "Internal error: {}", message,),
            Error::UnclosedBracket(bracket, _) => write!(f, "Unclosed Bracket '{}'", bracket,),
            Error::FailedToParse(_) => write!(f, "Failed to parse text"),
        }
    }
}
