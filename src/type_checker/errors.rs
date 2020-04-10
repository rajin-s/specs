use crate::language::types::*;
use std::fmt;

pub type TypeErrorList = Vec<TypeError>;

pub enum TypeError
{
    UnknownSymbolType(String),
    InvalidOperatorType(Type),
    InvalidArgumentType(usize, Type, Type),
    InvalidArgumentCount(usize, usize),
    InvalidBindingType(String, Type),
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
            InvalidBindingType(name, t) =>
            {
                write!(f, "Invalid type for binding '{}': found {}", name, t)
            }
        }
    }
}
