use super::*;
use crate::language::symbols;

pub fn apply(expression: &mut SExpression)
{
    match expression
    {
        SExpression::Symbol(symbol) =>
        {
            // A.B.C => ((A . B) . C)
            if let Some(new_expression) = expand_chain(symbols::operators::ACCESS, symbol)
            {
                *expression = new_expression;
            }
        }
        SExpression::List(_, elements) =>
        {
            for element in elements.iter_mut()
            {
                apply(element);
            }
        }
    }
}

fn expand_chain(operator: &str, symbol: &String) -> Option<SExpression>
{
    if symbol.contains(operator)
    {
        let mut tokens = symbol.split(operator);

        // We know there are at least two elements
        let first = tokens.next().unwrap();
        let second = tokens.next().unwrap();

        if first == "" || second == ""
        {
            // Empty token in access chain
            return None;
        }

        // Start with the first two elements
        let mut result = SExpression::List(
            BracketType::Round,
            vec![
                SExpression::Symbol(String::from(first)),
                SExpression::Symbol(String::from(operator)),
                SExpression::Symbol(String::from(second)),
            ],
        );

        // Nest operators such that
        // A~B~C => ((A ~ B) ~ C)
        for token in tokens
        {
            if token == ""
            {
                // Empty token in access chain
            }
            else
            {
                result = SExpression::List(
                    BracketType::Round,
                    vec![
                        result,
                        SExpression::Symbol(String::from(operator)),
                        SExpression::Symbol(String::from(token)),
                    ],
                );
            }
        }

        return Some(result);
    }
    else
    {
        return None;
    }
}
