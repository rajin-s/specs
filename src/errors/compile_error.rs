pub use super::*;

use crate::language::node::*;
use crate::utilities::*;

type IndirectType = Indirect<Type>;

pub enum Error
{
    // Infer types
    UnboundSymbolType(String, Source),
    BadDereferenceType(IndirectType, Source),
    UnexpectedOperands(String, Source),
    FailedToInferOperator(String, Source),

    // Check types
    BadOperatorType(Indirect<Type>, Source),
    BadOperandTypes(Vec<IndirectType>, Vec<IndirectType>, Source),
    BadAssignTypes(IndirectType, IndirectType, Source),
    BadReturnType(IndirectType, IndirectType, Source),
    BadConditionType(IndirectType, Source),
    BadBranchTypes(IndirectType, IndirectType, Source),

    Internal(String),
}

impl ErrorTrait for Error
{
    fn get_source(&self) -> Option<&Source>
    {
        match self
        {
            Error::UnboundSymbolType(.., source)
            | Error::BadDereferenceType(.., source)
            | Error::UnexpectedOperands(.., source)
            | Error::FailedToInferOperator(.., source) => Some(source),

            Error::BadOperatorType(.., source)
            | Error::BadOperandTypes(.., source)
            | Error::BadAssignTypes(.., source)
            | Error::BadReturnType(.., source)
            | Error::BadConditionType(.., source)
            | Error::BadBranchTypes(.., source) => Some(source),

            _ => None,
        }
    }

    fn get_description(&self) -> Option<&str>
    {
        match self
        {
            Error::UnexpectedOperands(description, _)
            | Error::FailedToInferOperator(description, _) => Some(description),
            _ => None,
        }
    }

    fn show(&self) -> bool
    {
        true
    }
}

use std::fmt;

impl fmt::Display for Error
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        match &self
        {
            Error::UnboundSymbolType(name, _) => write!(f, "Unbound Symbol '{}'", name),
            Error::BadDereferenceType(t, _) => write!(f, "Can't dereference type '{}'", t),
            Error::UnexpectedOperands(_, _) => write!(f, "Unexpected operand types"),
            Error::FailedToInferOperator(_, _) => write!(f, "Failed to infer operator type"),

            Error::BadOperatorType(found_type, _) => write!(
                f,
                "Expected operator type to be function, found: {}",
                found_type
            ),
            Error::BadOperandTypes(found_types, expected_types, _) =>
            {
                let _ = write!(f, "Unexpected operands for call, expected: (");
                for (i, expected_type) in expected_types.into_iter().enumerate()
                {
                    let _ = match i
                    {
                        0 => write!(f, "{}", expected_type),
                        _ => write!(f, " {}", expected_type),
                    };
                }
                let _ = write!(f, "), found: (");
                for (i, found_type) in found_types.into_iter().enumerate()
                {
                    let _ = match i
                    {
                        0 => write!(f, "{}", found_type),
                        _ => write!(f, " {}", found_type),
                    };
                }
                write!(f, ")")
            }
            Error::BadAssignTypes(found_type, expected_type, _) => write!(
                f,
                "Unexpected types in assign statement, expected: {}, found: {}",
                expected_type, found_type
            ),
            Error::BadReturnType(found_type, expected_type, _) => write!(
                f,
                "Return type of function body doesn't match definition, expected: {}, found: {}",
                expected_type, found_type
            ),
            Error::BadConditionType(found_type, _) => write!(
                f,
                "Unexpected type in condition statement, expected: bool, found: {}",
                found_type
            ),
            Error::BadBranchTypes(then_type, else_type, _) => write!(
                f,
                "Conditional branch types do not match, then: {}, else: {}",
                then_type, else_type
            ),

            Error::Internal(message) => write!(f, "Internal '{}'", message),
        }
    }
}
