use std::fmt;

/* -------------------------------------------------------------------------- */
/*                                 Structures                                 */
/* -------------------------------------------------------------------------- */

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BracketType
{
    None,
    Round,
    Square,
    Curly,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SExpression
{
    Symbol(String),
    List(BracketType, Vec<SExpression>),
}

impl SExpression
{
    pub fn empty() -> SExpression
    {
        return SExpression::List(BracketType::None, Vec::new());
    }

    pub fn is_empty(&self) -> bool
    {
        if let SExpression::List(BracketType::None, elements) = self
        {
            return elements.len() == 0;
        }
        else
        {
            return false;
        }
    }
}

/* -------------------------------------------------------------------------- */
/*                           Display Implementation                           */
/* -------------------------------------------------------------------------- */

impl fmt::Display for SExpression
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        match self
        {
            SExpression::Symbol(content) => write!(f, "{}", content),
            SExpression::List(bracket, elements) =>
            {
                let (open, close) = match bracket
                {
                    BracketType::None => ("<", ">"),
                    BracketType::Round => ("(", ")"),
                    BracketType::Square => ("[", "]"),
                    BracketType::Curly => ("{", "}"),
                };

                let _ = write!(f, "{}", open);
                for (i, element) in elements.iter().enumerate()
                {
                    if i == 0
                    {
                        let _ = write!(f, "{}", element);
                    }
                    else
                    {
                        let _ = write!(f, " {}", element);
                    }
                }
                write!(f, "{}", close)
            }
        }
    }
}
