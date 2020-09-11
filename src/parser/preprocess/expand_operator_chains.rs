use super::*;
use crate::language::symbols;
use crate::source::Source;

pub fn apply(expression: &mut SExpression)
{
    match expression
    {
        SExpression::Symbol(symbol, source) =>
        {
            // A.B.C => ((A . B) . C)
            if let Some(new_expression) = expand_chain(symbols::operators::ACCESS, symbol, source)
            {
                *expression = new_expression;
            }
        }
        SExpression::List(_, elements, _) =>
        {
            for element in elements.iter_mut()
            {
                apply(element);
            }
        }
        SExpression::Empty(_) => (),
    }
}

fn expand_chain(operator: &str, symbol: &String, source: &Source) -> Option<SExpression>
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
                SExpression::Symbol(String::from(first), source.clone()),
                SExpression::Symbol(String::from(operator), source.clone()),
                SExpression::Symbol(String::from(second), source.clone()),
            ],
            source.clone(),
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
                        SExpression::Symbol(String::from(operator), source.clone()),
                        SExpression::Symbol(String::from(token), source.clone()),
                    ],
                    source.clone(),
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
