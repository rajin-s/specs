use super::*;

pub fn remove_empty(elements: &mut Vec<SExpression>, empty_count: usize)
{
    // Remove empty lists so subsequent grouping operation can work properly
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

pub fn group(slice_size: usize, bracket_type: BracketType, expressions: &mut [SExpression])
{
    // Create a temporary vector to hold ownership of the grouped elements
    let mut temp_vec = vec![SExpression::empty(); slice_size];
    let temp_slice = temp_vec.as_mut_slice();

    // Extract the group slice from the original list
    expressions.swap_with_slice(temp_slice);

    // Make a single group with the extracted elements
    let mut group = [SExpression::List(bracket_type, temp_vec)];

    // Insert the new group into the original list (leaving the preceding empties)
    //  (... a b c ...) => (... <> <> (a b c) ...)

    expressions[(slice_size - 1)..slice_size].swap_with_slice(&mut group);
}
pub fn group_front(slice_size: usize, bracket_type: BracketType, expressions: &mut [SExpression])
{
    // Create a temporary vector to hold ownership of the grouped elements
    let mut temp_vec = vec![SExpression::empty(); slice_size];
    let temp_slice = temp_vec.as_mut_slice();

    // Extract the group slice from the original list
    expressions.swap_with_slice(temp_slice);

    // Make a single group with the extracted elements
    let mut group = [SExpression::List(bracket_type, temp_vec)];

    // Insert the new group into the original list (leaving the trailing empties)
    //  (... a b c ...) => (... (a b c) <> <> ...)

    expressions[0..1].swap_with_slice(&mut group);
}

pub fn make_groups_exclude<TFilter, TExcludeFilter>(
    slice_size: usize,
    filter: TFilter,
    exlude_filter: TExcludeFilter,
    source_bracket_type: BracketType,
    group_bracket_type: BracketType,
    elements: &mut Vec<SExpression>,
) where
    TFilter: Fn(&[SExpression]) -> bool,
    TExcludeFilter: Fn(&[SExpression], usize) -> bool,
{
    let same_bracket = source_bracket_type == group_bracket_type;

    // Consider lists larger than a single group, or single-group lists with a different bracket type
    if elements.len() > slice_size || (elements.len() == slice_size && !same_bracket)
    {
        let max_slice_start = elements.len() - (slice_size - 1);
        let elements_slice = elements.as_mut_slice();

        let mut skip_count = 0;

        let mut ungrouped_element_count = 0;
        let mut empty_count = 0;

        for i in 0..max_slice_start
        {
            // Skip over elements that were part of another group
            if skip_count > 0
            {
                skip_count -= 1;
                continue;
            }

            if i == max_slice_start - 1 && ungrouped_element_count == 0 && same_bracket
            {
                // All other elements have been grouped, so a final group would already be grouped in the original list
                continue;
            }

            if exlude_filter(elements_slice, i)
            {
                continue;
            }

            let slice = &mut elements_slice[i..i + slice_size];

            if filter(slice)
            {
                //  (... a b c ...) => (... <> <> (a b c) ...)
                group(slice_size, group_bracket_type, slice);

                // Skip the resulting empty elements
                skip_count = slice_size - 2;

                // Keep track of how many empty elements were created
                empty_count += slice_size - 1;
            }
            else
            {
                // The first element (at least) is not in a group
                ungrouped_element_count += 1;
            }
        }

        // Remove elements that were left empty after being moved to a group
        if empty_count > 0
        {
            remove_empty(elements, empty_count);
        }
    }
}

pub fn make_groups<TFilter>(
    slice_size: usize,
    filter: TFilter,
    source_bracket_type: BracketType,
    group_bracket_type: BracketType,
    elements: &mut Vec<SExpression>,
) where
    TFilter: Fn(&[SExpression]) -> bool,
{
    fn no_filter(_elements: &[SExpression], _index: usize) -> bool
    {
        return false;
    }

    make_groups_exclude(
        slice_size,
        filter,
        no_filter,
        source_bracket_type,
        group_bracket_type,
        elements,
    );
}