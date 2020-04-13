use super::*;
use crate::language::symbols;

pub fn apply(expression: &mut SExpression)
{
    match expression
    {
        SExpression::List(source_bracket_type, elements) =>
        {
            // Make all groups in this list
            group_bindings(*source_bracket_type, elements);
            group_assigns(*source_bracket_type, elements);

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

fn group_bindings(source_bracket_type: BracketType, elements: &mut Vec<SExpression>)
{
    fn filter(slice: &[SExpression]) -> bool
    {
        match slice
        {
            [SExpression::Symbol(binding_keyword), _name, SExpression::Symbol(assign_keyword), _binding]
                if binding_keyword == symbols::keywords::BINDING
                    && assign_keyword == symbols::operators::ASSIGN =>
            {
                true
            }
            _ => false,
        }
    }

    utilities::make_groups(4, filter, source_bracket_type, BracketType::Round, elements);
}

fn group_assigns(source_bracket_type: BracketType, elements: &mut Vec<SExpression>)
{
    fn filter(slice: &[SExpression]) -> bool
    {
        match slice
        {
            [_lhs, SExpression::Symbol(assign_keyword), _rhs]
                if assign_keyword == symbols::operators::ASSIGN =>
            {
                true
            }
            _ => false,
        }
    }
    fn exclude_filter(elements: &[SExpression], index: usize) -> bool
    {
        // Make sure we aren't grouping something that's actually part of a let binding
        if index > 0
        {
            if let SExpression::Symbol(binding_keyword) = &elements[index - 1]
            {
                if binding_keyword == symbols::keywords::BINDING
                {
                    return true;
                }
            }
        }

        return false;
    }

    utilities::make_groups_exclude(
        3,
        filter,
        exclude_filter,
        source_bracket_type,
        BracketType::Round,
        elements,
    );
}