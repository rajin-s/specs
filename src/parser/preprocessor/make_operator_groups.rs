use super::*;
use crate::language::symbols;

pub fn apply(expression: &mut SExpression)
{
    match expression
    {
        SExpression::List(source_bracket_type, elements) =>
        {
            // Make all groups in this list
            use symbols::operators::*;
            macro_rules! make_filter {
                [$first:path] => {
                    |op| op == $first
                };
                [$first:path, $($rest:path),*] => {
                    |op| match op
                    {
                        $first $(| $rest)* => true,
                        _ => false
                    }
                };
            }

            // Access operators
            // group_binary_operators(make_filter![ACCESS], elements);
            // group_binary_operators(make_filter![INDEX], elements);

            // Arithmetic operators
            // group_binary_operators(make_filter![POW], elements);
            group_binary_operators(make_filter![TIMES, DIVIDE, MODULO], *source_bracket_type, elements);
            group_binary_operators(make_filter![PLUS, MINUS], *source_bracket_type, elements);

            // Logical operators
            group_binary_operators(
                make_filter![LESS, GREATER, LESS_EQUAL, GREATER_EQUAL],
                *source_bracket_type,
                elements,
            );
            group_binary_operators(
                make_filter![EQUAL, NOT_EQUAL],
                *source_bracket_type,
                elements,
            );
            group_binary_operators(make_filter![AND, OR, XOR], *source_bracket_type, elements);

            // Other operators
            // Should this be allowed?
            // group_binary_operators(make_filter![ACCESS], *source_bracket_type, elements);

            // Then traverse child lists
            for element in elements.iter_mut()
            {
                apply(element);
            }
        }
        _ =>
        {}
    }
}

fn group_binary_operators<TFilter>(
    operator_filter: TFilter,
    source_bracket_type: BracketType,
    elements: &mut Vec<SExpression>,
) where
    TFilter: Fn(&str) -> bool,
{
    let filter = |slice: &[SExpression]| -> bool {
        match slice
        {
            [_a, SExpression::Symbol(op), _b] if operator_filter(op) => true,
            _ => false,
        }
    };

    utilities::make_groups(3, filter, source_bracket_type, BracketType::Round, elements);
}
