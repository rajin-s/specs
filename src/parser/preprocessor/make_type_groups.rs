use super::*;
use crate::language::symbols;

pub fn apply(expression: &mut SExpression)
{
    match expression
    {
        SExpression::List(source_bracket_type, elements) =>
        {
            // Make all groups in this list
            group_types(*source_bracket_type, elements);
            group_regions(*source_bracket_type, elements);
            group_traits(*source_bracket_type, elements);

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

fn group_types(source_bracket_type: BracketType, elements: &mut Vec<SExpression>)
{
    fn filter(slice: &[SExpression]) -> bool
    {
        use SExpression::*;
        match slice
        {
            // type Type { ... }
            [Symbol(type_keyword), Symbol(_type_name), List(BracketType::Curly, _content)]
                if type_keyword == symbols::keywords::TYPE =>
            {
                true
            }
            _ => false,
        }
    }

    utilities::make_groups(3, filter, source_bracket_type, BracketType::Round, elements);
}
fn group_regions(source_bracket_type: BracketType, elements: &mut Vec<SExpression>)
{
    fn filter_region(slice: &[SExpression]) -> bool
    {
        use SExpression::*;
        match slice
        {
            // (data|private|public) { ... }
            [Symbol(region_keyword), List(BracketType::Curly, _content)]
                if region_keyword == symbols::keywords::DATA
                    || region_keyword == symbols::keywords::PUBLIC
                    || region_keyword == symbols::keywords::PRIVATE =>
            {
                true
            }
            _ => false,
        }
    }

    utilities::make_groups(
        2,
        filter_region,
        source_bracket_type,
        BracketType::None,
        elements,
    );
}
fn group_traits(source_bracket_type: BracketType, elements: &mut Vec<SExpression>)
{
    fn filter_trait_block(slice: &[SExpression]) -> bool
    {
        use SExpression::*;
        match slice
        {
            // is Type { ... }
            [Symbol(is_keyword), Symbol(_), List(BracketType::Curly, _content)]
                if is_keyword == symbols::keywords::IS =>
            {
                true
            }
            _ => false,
        }
    }
    fn filter_trait(slice: &[SExpression]) -> bool
    {
        use SExpression::*;
        match slice
        {
            // is Type
            [Symbol(is_keyword), Symbol(_)] if is_keyword == symbols::keywords::IS => true,
            _ => false,
        }
    }
    fn exclude_trait_block(elements: &[SExpression], index: usize) -> bool
    {
        // Make sure we aren't grouping something that has an implementation block
        if index < elements.len() - 2
        {
            // is Type { ... }
            // ^index  +2
            if let SExpression::List(BracketType::Curly, _content) = &elements[index + 2]
            {
                return true;
            }
        }

        return false;
    }
    utilities::make_groups(
        3,
        filter_trait_block,
        source_bracket_type,
        BracketType::Round,
        elements,
    );
    utilities::make_groups_exclude(
        2,
        filter_trait,
        exclude_trait_block,
        source_bracket_type,
        BracketType::Round,
        elements,
    );
}
