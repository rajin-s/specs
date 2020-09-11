use super::*;
use crate::language::symbols;

pub fn apply(expression: &mut SExpression)
{
    match expression
    {
        SExpression::List(source_bracket_type, elements, _) =>
        {
            // Make groups in this list
            group_when_pairs(elements);
            group_when(*source_bracket_type, elements);
            group_when_else(*source_bracket_type, elements);

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

fn group_when(source_bracket_type: BracketType, elements: &mut Vec<SExpression>)
{
    fn filter(slice: &[SExpression]) -> bool
    {
        match slice
        {
            [SExpression::Symbol(when_keyword, _), SExpression::List(BracketType::Curly, _, _)]
                if when_keyword == symbols::keywords::WHEN =>
            {
                true
            }
            _ => false,
        }
    }
    fn exclude_filter(elements: &[SExpression], index: usize) -> bool
    {
        // Make sure we aren't grouping something that has an else branch
        if index < elements.len() - 2
        {
            // when {...} else
            // ^index     +2
            if let SExpression::Symbol(else_keyword, _) = &elements[index + 2]
            {
                if else_keyword == symbols::keywords::ELSE
                {
                    return true;
                }
            }
        }

        return false;
    }

    utilities::make_groups_exclude(
        2,
        filter,
        exclude_filter,
        source_bracket_type,
        BracketType::Round,
        elements,
    );
}
fn group_when_else(source_bracket_type: BracketType, elements: &mut Vec<SExpression>)
{
    fn filter(slice: &[SExpression]) -> bool
    {
        match slice
        {
            [SExpression::Symbol(when_keyword, _), SExpression::List(BracketType::Curly, _, _), SExpression::Symbol(else_keyword, _), _]
                if when_keyword == symbols::keywords::WHEN
                    && else_keyword == symbols::keywords::ELSE =>
            {
                true
            }
            _ => false,
        }
    }

    utilities::make_groups(4, filter, source_bracket_type, BracketType::Round, elements);
}

fn group_when_pairs(elements: &mut Vec<SExpression>)
{
    const SLICE_SIZE: usize = 2;

    if elements.len() > SLICE_SIZE
    {
        let element_count = elements.len();
        let max_slice_start = element_count - SLICE_SIZE;

        let elements_slice = elements.as_mut_slice();
        for i in 0..max_slice_start + 1
        {
            let slice = &mut elements_slice[i..i + SLICE_SIZE];
            match slice
            {
                // when { ... }
                [SExpression::Symbol(when_keyword, _), SExpression::List(BracketType::Curly, inner_elements, _)]
                    if when_keyword == symbols::keywords::WHEN =>
                {
                    group_pairs_with_associate(inner_elements);
                }
                _ =>
                {}
            }
        }

        fn group_pairs_with_associate(elements: &mut Vec<SExpression>)
        {
            fn filter(slice: &[SExpression]) -> bool
            {
                match slice
                {
                    [_, SExpression::Symbol(associate_keyword, _), _]
                        if associate_keyword == symbols::keywords::ASSOCIATE =>
                    {
                        true
                    }
                    _ => false,
                }
            }
            utilities::make_groups(3, filter, BracketType::None, BracketType::None, elements);
        }
    }
}
