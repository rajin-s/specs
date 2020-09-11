use super::*;
use crate::language::symbols;

pub fn apply(expression: &mut SExpression)
{
    match expression
    {
        SExpression::List(source_bracket_type, elements, _) =>
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
            [SExpression::Symbol(binding_keyword, _), _name, SExpression::Symbol(assign_keyword, _), _binding]
                if binding_keyword == symbols::keywords::BINDING
                    && assign_keyword == symbols::operators::ASSIGN_BINDING =>
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
            [_lhs, SExpression::Symbol(assign_keyword, _), _rhs]
                if assign_keyword == symbols::operators::ASSIGN
                    || assign_keyword == symbols::operators::ASSIGN_REVERSE =>
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
            if let SExpression::Symbol(binding_keyword, _) = &elements[index - 1]
            {
                if binding_keyword == symbols::keywords::BINDING
                {
                    return true;
                }
            }
        }

        // Make sure we aren't messing with part of a function definition header
        match &elements[0]
        {
            SExpression::Symbol(function_keyword, _)
                if function_keyword == symbols::keywords::FUNCTION =>
            {
                return true;
            }
            _ =>
            {}
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
