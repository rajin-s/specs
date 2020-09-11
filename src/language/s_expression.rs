use crate::source::Source;
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
    Empty(Source),
    Symbol(String, Source),
    List(BracketType, Vec<SExpression>, Source),
}

impl SExpression
{
    pub fn empty() -> SExpression
    {
        SExpression::Empty(Source::empty())
    }

    pub fn is_empty(&self) -> bool
    {
        if let SExpression::Empty(_) = self
        {
            true
        }
        else
        {
            false
        }
    }

    pub fn get_source(&self) -> Source
    {
        match self
        {
            SExpression::Empty(source) => source.clone(),
            SExpression::Symbol(_, source) => source.clone(),
            SExpression::List(_, _, source) => source.clone(),
        }
    }

    pub fn take(s_expression: &mut SExpression) -> SExpression
    {
        let source = s_expression.get_source();
        let mut temp = SExpression::Empty(source);
        std::mem::swap(&mut temp, s_expression);
        return temp;
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
            SExpression::Empty(_) => write!(f, "(empty)"),
            SExpression::Symbol(content, _) => write!(f, "{}", content),
            SExpression::List(bracket, elements, _) =>
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
