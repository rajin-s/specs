use super::*;
use crate::language::symbols;

pub fn apply(expression: &mut SExpression)
{
    match expression
    {
        SExpression::List(source_bracket_type, elements, _) =>
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
            group_binary_operators(
                make_filter![TIMES, DIVIDE, MODULO],
                *source_bracket_type,
                elements,
            );
            group_binary_operators(make_filter![PLUS, MINUS], *source_bracket_type, elements);
            group_unary_operators(make_filter![MINUS], *source_bracket_type, elements, true);

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
            group_unary_operators(make_filter![NOT], *source_bracket_type, elements, false);
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

fn group_unary_operators<TFilter>(
    operator_filter: TFilter,
    source_bracket_type: BracketType,
    elements: &mut Vec<SExpression>,
    check_binary_groups: bool,
) where
    TFilter: Fn(&str) -> bool,
{
    if elements.len() == 3 && check_binary_groups
    {
        // Check if we're looking at a group that was made by group_binary
        //  ex. Don't turn (a - b) into (a (- b))

        let is_infix_binary = match elements.as_slice()
        {
            [a, SExpression::Symbol(op, _), _] if operator_filter(op) => match a
            {
                SExpression::Symbol(a_symbol, _) => !symbols::is_structural(a_symbol),
                _ => true,
            },
            _ => false,
        };

        if is_infix_binary
        {
            return;
        }
    }

    let filter = |slice: &[SExpression]| -> bool {
        match slice
        {
            [SExpression::Symbol(op, _), a] if operator_filter(op) => match a
            {
                SExpression::Symbol(a_symbol, _) => !symbols::is_structural(a_symbol),
                _ => true,
            },
            _ => false,
        }
    };

    utilities::make_groups(2, filter, source_bracket_type, BracketType::Round, elements);
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
            [a, SExpression::Symbol(op, _), b] if operator_filter(op) => match (a, b)
            {
                (SExpression::Symbol(a_symbol, _), SExpression::Symbol(b_symbol, _)) =>
                {
                    !symbols::is_structural(a_symbol) && !symbols::is_structural(b_symbol)
                }
                _ => true,
            },
            _ => false,
        }
    };

    utilities::make_groups(3, filter, source_bracket_type, BracketType::Round, elements);
}
