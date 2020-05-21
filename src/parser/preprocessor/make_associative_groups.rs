use super::*;
use crate::language::symbols;

pub fn apply(expression: &mut SExpression)
{
    match expression
    {
        SExpression::List(source_bracket_type, elements) =>
        {
            // Make all groups in this list
            convert_inline_associations(*source_bracket_type, elements);

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

// A : B => [A B]
// note: [A : B] => [A : B]
fn convert_inline_associations(source_bracket_type: BracketType, elements: &mut Vec<SExpression>)
{
    let same_bracket = source_bracket_type == BracketType::Square;

    // A : B
    const SLICE_SIZE: usize = 3;

    // Consider lists larger than a single group, or single-group lists with a different bracket type
    if elements.len() > SLICE_SIZE || (elements.len() == SLICE_SIZE && !same_bracket)
    {
        let max_slice_start = elements.len() - (SLICE_SIZE - 1);
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

            let slice = &mut elements_slice[i..i + SLICE_SIZE];
            match slice
            {
                [_a, SExpression::Symbol(associate_keyword), _b]
                    if associate_keyword == symbols::keywords::ASSOCIATE_TYPE =>
                {
                    //  (... a : b ...) => (... <> <> [a b] ...)
                    {
                        // Create a temporary vector to hold ownership of the grouped elements
                        let mut temp_vec = vec![SExpression::empty(); SLICE_SIZE];
                        let temp_slice = temp_vec.as_mut_slice();

                        // Extract the group slice from the original list
                        slice.swap_with_slice(temp_slice);

                        // Remove the middle element
                        temp_vec.remove(1);

                        // Make a single group with the extracted elements
                        let mut group = [SExpression::List(BracketType::Square, temp_vec)];

                        // Insert the new group into the original list (leaving the preceding empties)
                        //  (... a b c ...) => (... <> <> (a b c) ...)

                        slice[(SLICE_SIZE - 1)..SLICE_SIZE].swap_with_slice(&mut group);
                    }

                    // Skip the resulting empty elements
                    skip_count = SLICE_SIZE - 2;

                    // Keep track of how many empty elements were created
                    empty_count += SLICE_SIZE - 1;
                }
                _ =>
                {
                    // The first element (at least) is not in a group
                    ungrouped_element_count += 1;
                }
            }
        }

        // Remove elements that were left empty after being moved to a group
        if empty_count > 0
        {
            utilities::remove_empty(elements, empty_count);
        }
    }
}