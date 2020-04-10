use super::*;
use crate::language::symbols;

pub fn apply(expression: &mut SExpression)
{
    match expression
    {
        SExpression::List(bracket_type, elements) =>
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
            // group_binary(make_filter![ACCESS], elements);
            // group_binary(make_filter![INDEX], elements);

            // Arithmetic operators
            // group_binary(make_filter![POW], elements);
            // group_binary(make_filter![TIMES, DIVIDE, MODULO], elements);
            group_binary(make_filter![PLUS, MINUS], *bracket_type, elements);

            // Logical operators
            // group_binary(
            //     make_filter![LESS, GREATER, LESS_EQUAL, GREATER_EQUAL],
            //     elements,
            // );
            // group_binary(make_filter![EQUAL, NOT_EQUAL], elements);
            // group_binary(make_filter![AND, OR, XOR], elements);

            // Structural operators
            group_binding(*bracket_type, elements);
            // group_binary(make_filter![ASSIGN], elements);

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

fn group_binary<TFilter>(
    operator_filter: TFilter,
    bracket_type: BracketType,
    elements: &mut Vec<SExpression>,
) where
    TFilter: Fn(&str) -> bool,
{
    const SLICE_SIZE: usize = 3;

    // Consider (...) lists larger than a single group, or other lists of one or more groups
    if elements.len() > SLICE_SIZE
        || (elements.len() == SLICE_SIZE && bracket_type != BracketType::Round)
    {
        let max_slice_start = elements.len() - (SLICE_SIZE - 1);
        let elements_slice = elements.as_mut_slice();

        let mut skip_count = 0;

        let mut ungrouped_element_count = 0;
        let mut empty_count = 0;

        for i in 0..max_slice_start
        {
            // Skip elements that were part of another group
            if skip_count > 0
            {
                skip_count -= 1;
                continue;
            }

            // All other elements have been grouped, so the last slice will be grouped already (in the original list)
            if i == max_slice_start - 1
                && ungrouped_element_count == 0
                && bracket_type == BracketType::Round
            {
                continue;
            }

            let slice = &mut elements_slice[i..i + SLICE_SIZE];
            match slice
            {
                [_a, SExpression::Symbol(op), _b] if operator_filter(op.as_str()) =>
                {
                    // Extract the group slice from the original list
                    let mut temp = [
                        SExpression::empty(),
                        SExpression::empty(),
                        SExpression::empty(),
                    ];
                    slice.swap_with_slice(&mut temp);

                    // Make a single group with the extracted elements
                    let mut group = [SExpression::List(BracketType::Round, temp.to_vec())];

                    // Insert the new group into the original list
                    //  (... a ~ b ...) => (... <> <> (a ~ b) ...)
                    slice[(SLICE_SIZE - 1)..SLICE_SIZE].swap_with_slice(&mut group);

                    // Skip the next element
                    skip_count = SLICE_SIZE - 2;

                    // Two elements of the original list are now empty
                    empty_count += SLICE_SIZE - 1;
                }
                _ =>
                {
                    ungrouped_element_count += 1;
                }
            }
        }

        // Remove empty lists so subsequent grouping operation can work properly
        if empty_count > 0
        {
            let new_element_count = elements.len() - empty_count;

            let mut new_elements: Vec<SExpression> = Vec::new();
            new_elements.reserve(new_element_count);

            for element in elements.drain(0..)
            {
                if !element.is_empty()
                {
                    new_elements.push(element);
                }
            }

            *elements = new_elements;
        }
    }
}

fn group_binding(bracket_type: BracketType, elements: &mut Vec<SExpression>)
{
    const SLICE_SIZE: usize = 4;

    // Consider (...) lists larger than a single group, or other lists of one or more groups
    if elements.len() > SLICE_SIZE
        || (elements.len() == SLICE_SIZE && bracket_type != BracketType::Round)
    {
        let max_slice_start = elements.len() - (SLICE_SIZE - 1);
        let elements_slice = elements.as_mut_slice();

        let mut skip_count = 0;

        let mut ungrouped_element_count = 0;
        let mut empty_count = 0;

        for i in 0..max_slice_start
        {
            // Skip elements that were part of another group
            if skip_count > 0
            {
                skip_count -= 1;
                continue;
            }

            // All other elements have been grouped, so the last slice will be grouped already (in the original list)
            if i == max_slice_start - 1
                && ungrouped_element_count == 0
                && bracket_type == BracketType::Round
            {
                continue;
            }

            let slice = &mut elements_slice[i..i + SLICE_SIZE];
            match slice
            {
                [SExpression::Symbol(binding_keyword), _name, SExpression::Symbol(assign_keyword), _binding]
                    if binding_keyword == symbols::keywords::BINDING
                        && assign_keyword == symbols::operators::ASSIGN =>
                {
                    // Extract the group slice from the original list
                    let mut temp = [
                        SExpression::empty(),
                        SExpression::empty(),
                        SExpression::empty(),
                        SExpression::empty(),
                    ];
                    slice.swap_with_slice(&mut temp);

                    // Make a single group with the extracted elements
                    let mut group = [SExpression::List(BracketType::Round, temp.to_vec())];

                    // Insert the new group into the original list
                    //  (... let a = b ...) => (... <> <> <> (let a = b) ...)
                    slice[(SLICE_SIZE - 1)..SLICE_SIZE].swap_with_slice(&mut group);

                    // Skip the next elements
                    skip_count = SLICE_SIZE - 2;

                    // Three elements of the original list are now empty
                    empty_count += SLICE_SIZE - 1;
                }
                _ =>
                {
                    ungrouped_element_count += 1;
                }
            }
        }

        // Remove empty lists so subsequent grouping operation can work properly
        if empty_count > 0
        {
            let new_element_count = elements.len() - empty_count;

            let mut new_elements: Vec<SExpression> = Vec::new();
            new_elements.reserve(new_element_count);

            for element in elements.drain(0..)
            {
                if !element.is_empty()
                {
                    new_elements.push(element);
                }
            }

            *elements = new_elements;
        }
    }
}
