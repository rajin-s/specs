use super::*;
use crate::language::symbols;

pub fn apply(expression: &mut SExpression)
{
    match expression
    {
        SExpression::List(source_bracket_type, elements) =>
        {
            match source_bracket_type
            {
                BracketType::None | BracketType::Curly =>
                {
                    // Make groups in this list
                    group_conditionals(*source_bracket_type, elements);
                }
                _ =>
                {}
            }

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

fn group_conditionals(source_bracket_type: BracketType, elements: &mut Vec<SExpression>)
{
    use SExpression::Symbol;

    let same_bracket = source_bracket_type == BracketType::Round;

    // note: This function assumes IF_THEN_SIZE < IF_ELSE_SIZE
    const IF_ELSE_SIZE: usize = 6;
    const IF_THEN_SIZE: usize = 4;

    if elements.len() > IF_THEN_SIZE || (elements.len() == IF_THEN_SIZE && !same_bracket)
    {
        let max_if_then_slice_start = elements.len() - (IF_THEN_SIZE - 1);
        let max_if_else_slice_start = elements.len() - (IF_ELSE_SIZE - 1);

        let element_count = elements.len();
        let elements_slice = elements.as_mut_slice();

        let mut empty_count = 0;

        let mut skip_count = 0;

        // note: make groups in *reverse* order so else-if branched are handled correctly
        // ex. if A then B else if C then D else E
        //  => (if A then B else (if C then D else E))
        for i in (0..max_if_then_slice_start).rev()
        {
            // Skip over starting positions that would include empties from a previous group
            if skip_count > 0
            {
                skip_count -= 1;
                continue;
            }

            if i < max_if_else_slice_start
            {
                let slice = &mut elements_slice[i..i + IF_ELSE_SIZE];
                match slice
                {
                    // if ... then ... else ...
                    [Symbol(if_keyword), _condition, Symbol(then_keyword), _then, Symbol(else_keyword), _else]
                        if if_keyword == symbols::keywords::IF
                            && then_keyword == symbols::keywords::THEN
                            && else_keyword == symbols::keywords::ELSE =>
                    {
                        if empty_count + IF_ELSE_SIZE == element_count && same_bracket
                        {
                            // Don't make a group if it would mean all elements being replaced
                            // ex. re-use the original list to avoid infinite recursion
                            continue;
                        }

                        //  (... if a then b else c ...) => (... (if a then b else c) <> <> <> <> <> ...)
                        // note: place the group at the *front* of the original slice so it can be used as part of the next group
                        utilities::group_front(IF_ELSE_SIZE, BracketType::Round, slice);

                        // x y z a b c => x y z (a b c) <> <>
                        //       ^          .<--^
                        skip_count = IF_ELSE_SIZE - 2;
                        empty_count += IF_ELSE_SIZE - 1;
                    }
                    // if ... then ... ... ...
                    [Symbol(if_keyword), _condition, Symbol(then_keyword), _then, _, _]
                        if if_keyword == symbols::keywords::IF
                            && then_keyword == symbols::keywords::THEN =>
                    {
                        //  (... if a then b ...) => (... (if a then b) <> <> <> ...)
                        // note: place the group at the *front* of the original slice so it can be used as part of the next group
                        let if_then_slice = &mut slice[0..IF_THEN_SIZE];
                        utilities::group_front(IF_THEN_SIZE, BracketType::Round, if_then_slice);
                        skip_count = IF_THEN_SIZE - 2;
                        empty_count += IF_THEN_SIZE - 1;
                    }
                    _ =>
                    {}
                }
            }
            else
            {
                let slice = &mut elements_slice[i..i + IF_THEN_SIZE];
                {
                    match slice
                    {
                        // if ... then ...
                        [Symbol(if_keyword), _condition, Symbol(then_keyword), _then]
                            if if_keyword == symbols::keywords::IF
                                && then_keyword == symbols::keywords::THEN =>
                        {
                            if empty_count + IF_THEN_SIZE == element_count && same_bracket
                            {
                                // Don't make a group if it would mean all elements being replaced
                                // ex. re-use the original list to avoid infinite recursion
                                continue;
                            }

                            utilities::group_front(IF_THEN_SIZE, BracketType::Round, slice);
                            skip_count = IF_THEN_SIZE - 2;
                            empty_count += IF_THEN_SIZE - 1;
                        }
                        _ =>
                        {}
                    }
                }
            }
        }

        if empty_count > 0
        {
            utilities::remove_empty(elements, empty_count);
        }
    }
}
