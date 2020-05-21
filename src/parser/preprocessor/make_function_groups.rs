use super::*;
use crate::language::symbols;

pub fn apply(expression: &mut SExpression)
{
    match expression
    {
        SExpression::List(source_bracket_type, elements) =>
        {
            // Make all groups in this list
            group_arguments(elements);
            group_functions(*source_bracket_type, elements);

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

fn group_arguments(elements: &mut Vec<SExpression>)
{
    // [fn name]
    const NAME_SLICE_SIZE: usize = 2;

    if elements.len() > NAME_SLICE_SIZE
    {
        let max_slice_start = elements.len() - (NAME_SLICE_SIZE - 1);
        let elements_slice = elements.as_mut_slice();

        let mut skip_count = 0;
        let mut empty_count = 0;

        for i in 0..max_slice_start
        {
            if skip_count > 0
            {
                skip_count -= 1;
                continue;
            }

            let name_slice = &elements_slice[i..i + 2];
            match name_slice
            {
                // fn name
                [SExpression::Symbol(function_keyword), _]
                    if function_keyword == symbols::keywords::FUNCTION =>
                {
                    // Read elements until we reach a non-argument
                    let arguments_start = i + 2;
                    let mut argument_count = 0;
                    loop
                    {
                        let index = arguments_start + argument_count;
                        if index >= elements_slice.len()
                        {
                            // End of the list
                            break;
                        }

                        if let SExpression::List(BracketType::Square, _) = &elements_slice[index]
                        {
                            // Each [...] list is considered an argument
                            argument_count += 1;
                        }
                        else
                        {
                            // Any expression that is not a [...] list
                            break;
                        }
                    }

                    // ... fn name [A] [B] [C] ...
                    // ... fn name <>  <>  <[A] [B] [C]> ...
                    if argument_count > 0
                    {
                        let slice =
                            &mut elements_slice[arguments_start..arguments_start + argument_count];
                        utilities::group(argument_count, BracketType::None, slice);

                        empty_count += argument_count - 1;
                        skip_count += argument_count - 1;
                    }
                }
                _ =>
                {}
            }
        }

        if empty_count > 0
        {
            utilities::remove_empty(elements, empty_count);
        }
    }
}

fn group_functions(source_bracket_type: BracketType, elements: &mut Vec<SExpression>)
{
    fn filter_no_arguments(slice: &[SExpression]) -> bool
    {
        use SExpression::*;
        match slice
        {
            // fn name -> T {Body}
            [Symbol(function_keyword), _name, Symbol(arrow_keyword), _return_type, List(BracketType::Curly, _body)]
                if function_keyword == symbols::keywords::FUNCTION
                    && arrow_keyword == symbols::keywords::ARROW =>
            {
                true
            }
            _ => false,
        }
    }
    fn filter_with_arguments(slice: &[SExpression]) -> bool
    {
        use SExpression::*;
        match slice
        {
            // fn name <Arguments> -> T {Body}
            [Symbol(function_keyword), _name, List(BracketType::None, _arguments), Symbol(arrow_keyword), _return_type, List(BracketType::Curly, _body)]
                if function_keyword == symbols::keywords::FUNCTION
                    && arrow_keyword == symbols::keywords::ARROW =>
            {
                true
            }
            _ => false,
        }
    }
    fn filter_no_arguments_no_return(slice: &[SExpression]) -> bool
    {
        use SExpression::*;
        match slice
        {
            // fn name {Body}
            [Symbol(function_keyword), _name, List(BracketType::Curly, _body)]
                if function_keyword == symbols::keywords::FUNCTION =>
            {
                true
            }
            _ => false,
        }
    }
    fn filter_with_arguments_no_return(slice: &[SExpression]) -> bool
    {
        use SExpression::*;
        match slice
        {
            // fn name <Arguments> {Body}
            [Symbol(function_keyword), _name, List(BracketType::None, _arguments), List(BracketType::Curly, _body)]
                if function_keyword == symbols::keywords::FUNCTION =>
            {
                true
            }
            _ => false,
        }
    }
    utilities::make_groups(
        3,
        filter_no_arguments_no_return,
        source_bracket_type,
        BracketType::Round,
        elements,
    );
    utilities::make_groups(
        4,
        filter_with_arguments_no_return,
        source_bracket_type,
        BracketType::Round,
        elements,
    );
    utilities::make_groups(
        5,
        filter_no_arguments,
        source_bracket_type,
        BracketType::Round,
        elements,
    );
    utilities::make_groups(
        6,
        filter_with_arguments,
        source_bracket_type,
        BracketType::Round,
        elements,
    );
}
