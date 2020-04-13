#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum PrimitiveOperator
{
    // Arithmetic operators
    Add,

    // Comparison operators
    Equal,
    NotEqual,
    Less,
    Greater,
    LessEqual,
    GreaterEqual,

    // Logical operators
    And,
    Or,
    ExclusiveOr,
}
