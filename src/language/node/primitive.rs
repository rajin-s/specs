/* -------------------------------------------------------------------------- */
/*                                  Operators                                 */
/* -------------------------------------------------------------------------- */

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Operator
{
    // Arithmetic operators
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,

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

    // Memory operators
    Create,
    HeapAllocate,
    HeapFree,
}

/* -------------------------------------------------------------------------- */
/*                                   Display                                  */
/* -------------------------------------------------------------------------- */

impl std::fmt::Display for Operator
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result
    {
        let s = match self
        {
            // Arithmetic operators
            Operator::Add => "+",
            Operator::Subtract => "-",
            Operator::Multiply => "*",
            Operator::Divide => "/",
            Operator::Modulo => "%",

            // Comparison operators
            Operator::Equal => "==",
            Operator::NotEqual => "=/=",
            Operator::Less => "<",
            Operator::Greater => ">",
            Operator::LessEqual => "<=",
            Operator::GreaterEqual => ">=",

            // Logical operators
            Operator::And => "and",
            Operator::Or => "or",
            Operator::ExclusiveOr => "xor",

            // Memory operators
            Operator::Create => "create",
            Operator::HeapAllocate => "heap-alloc",
            Operator::HeapFree => "heap-free",
        };
        write!(f, "{}", s)
    }
}