use super::*;
use crate::language::symbols;

pub fn apply(expression: &mut SExpression)
{
    match expression
    {
        SExpression::List(source_bracket_type, elements, _) =>
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

// Group arguments in a flat list that may contain multiple functions
fn group_arguments(elements: &mut Vec<SExpression>)
{
    // [fn name]
    // [fn name -> return]
    const NAME_SLICE_SIZE: usize = 2;
    const NAME_RETURN_SLICE_SIZE: usize = 4;

    if elements.len() <= NAME_SLICE_SIZE
    {
        // Skip lists that are definitely not a function with arguments
        return;
    }

    let element_count = elements.len();
    let mut skip_count = 0;
    let mut empty_count = 0;

    for i in 0..element_count
    {
        // Skip over elements we looked ahead at
        if skip_count > 0
        {
            skip_count -= 1;
            continue;
        }

        let remaining_elements = element_count - i;

        // Get the offset that arguments could start, if any
        //  - after [fn _ -> _] or [fn _]
        let argument_start = if remaining_elements >= NAME_RETURN_SLICE_SIZE
        {
            let name_slice = &elements.as_slice()[i..i + NAME_RETURN_SLICE_SIZE];
            match name_slice
            {
                // fn _ -> _
                [SExpression::Symbol(x1, _), _, SExpression::Symbol(x2, _), _]
                    if x1 == symbols::keywords::FUNCTION && x2 == symbols::keywords::RETURNS =>
                {
                    // Arguments could start after return
                    NAME_RETURN_SLICE_SIZE
                }
                // fn _ _ _
                [SExpression::Symbol(x1, _), _, _, _] if x1 == symbols::keywords::FUNCTION =>
                {
                    // Arguments could start after name
                    NAME_SLICE_SIZE
                }
                _ =>
                {
                    // Not a function
                    continue;
                }
            }
        }
        else if remaining_elements >= NAME_SLICE_SIZE
        {
            let name_slice = &elements.as_slice()[i..i + NAME_SLICE_SIZE];
            match name_slice
            {
                // fn _
                [SExpression::Symbol(x1, _), _] if x1 == symbols::keywords::FUNCTION =>
                {
                    // Arguments could start after name
                    NAME_SLICE_SIZE
                }
                // Not a function
                _ =>
                {
                    continue;
                }
            }
        }
        else
        {
            // Not a function
            continue;
        };

        // Read ahead to see how many arguments there are to be grouped
        let mut argument_count = 0;
        {
            let search_slice = &elements.as_slice()[i + argument_start..];
            for i in 0..search_slice.len()
            {
                match &search_slice[i]
                {
                    // [...] argument
                    SExpression::List(BracketType::Square, _, _) =>
                    {
                        argument_count += 1;
                    }
                    // Not an argument
                    _ =>
                    {
                        break;
                    }
                }
            }
        }

        if argument_count > 0
        {
            // Group the arguments into a <...> list
            let arguments_slice = &mut elements.as_mut_slice()
                [i + argument_start..i + argument_start + argument_count];
            utilities::group(argument_count, BracketType::None, arguments_slice);

            // Keep track of how many empty spots we've made by grouping
            empty_count += argument_count - 1;
        }

        // We can skip over the parts of the full list that we know are part of the current function
        skip_count += argument_start + argument_count - 1;
    }

    // Clean up empty spots from groups
    if empty_count > 0
    {
        utilities::remove_empty(elements, empty_count);
    }
}

fn group_functions(source_bracket_type: BracketType, elements: &mut Vec<SExpression>)
{
    fn filter_return_with_arguments(slice: &[SExpression]) -> bool
    {
        use SExpression::*;
        match slice
        {
            // fn _ -> _ <...> {...}
            [Symbol(function_keyword, _), _, Symbol(arrow_keyword, _), _, List(BracketType::None, _, _), List(BracketType::Curly, _, _)]
                if function_keyword == symbols::keywords::FUNCTION
                    && arrow_keyword == symbols::keywords::RETURNS =>
            {
                true
            }
            // fn _ <...> -> _ {...}
            [Symbol(function_keyword, _), _, List(BracketType::None, _, _), Symbol(arrow_keyword, _), _, List(BracketType::Curly, _, _)]
                if function_keyword == symbols::keywords::FUNCTION
                    && arrow_keyword == symbols::keywords::RETURNS =>
            {
                true
            }
            _ => false,
        }
    }
    fn filter_return_no_arguments(slice: &[SExpression]) -> bool
    {
        use SExpression::*;
        match slice
        {
            // fn _ -> _ {...}
            [Symbol(function_keyword, _), _, Symbol(arrow_keyword, _), _, List(BracketType::Curly, _body, _)]
                if function_keyword == symbols::keywords::FUNCTION
                    && arrow_keyword == symbols::keywords::RETURNS =>
            {
                true
            }
            _ => false,
        }
    }
    fn filter_no_return_no_arguments(slice: &[SExpression]) -> bool
    {
        use SExpression::*;
        match slice
        {
            // fn _ {...}
            [Symbol(function_keyword, _), _, List(BracketType::Curly, _, _)]
                if function_keyword == symbols::keywords::FUNCTION =>
            {
                true
            }
            _ => false,
        }
    }
    fn filter_no_return_with_arguments(slice: &[SExpression]) -> bool
    {
        use SExpression::*;
        match slice
        {
            // fn _ <...> {...}
            [Symbol(function_keyword, _), _, List(BracketType::None, _, _), List(BracketType::Curly, _, _)]
                if function_keyword == symbols::keywords::FUNCTION =>
            {
                true
            }
            _ => false,
        }
    }
    utilities::make_groups(
        3,
        filter_no_return_no_arguments,
        source_bracket_type,
        BracketType::Round,
        elements,
    );
    utilities::make_groups(
        4,
        filter_no_return_with_arguments,
        source_bracket_type,
        BracketType::Round,
        elements,
    );
    utilities::make_groups(
        5,
        filter_return_no_arguments,
        source_bracket_type,
        BracketType::Round,
        elements,
    );
    utilities::make_groups(
        6,
        filter_return_with_arguments,
        source_bracket_type,
        BracketType::Round,
        elements,
    );
}
