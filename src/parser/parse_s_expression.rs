use super::s_expression::*;
use crate::language::symbols;
use std::collections::VecDeque;

#[derive(Debug)]
enum ParseNode<'a>
{
    Symbol(&'a str), // An atomic symbol (no further parsing can be done)
    List(BracketType, Vec<ParseNode<'a>>), // A list of potentially parsed nodes
    Unparsed(BracketType, &'a str), // An unparsed node that still needs to be expanded
}
type NodeQueue<'a> = VecDeque<&'a mut ParseNode<'a>>;

pub fn parse_expression(source: &String) -> Option<SExpression>
{
    // Perform raw text operations
    let stripped = strip_comments(source.as_str());

    // Create the parse tree root
    let mut parse_root = ParseNode::Unparsed(BracketType::None, stripped.as_str());

    // Expand and traverse the parse tree in breadth-first order
    let mut parse_queue = NodeQueue::new();
    parse_queue.push_back(&mut parse_root);

    // Keep expanding nodes until none are left
    while let Some(node) = parse_queue.pop_front()
    {
        expand_node(node);

        // Add child nodes to the parse queue
        if let ParseNode::List(_, child_nodes) = node
        {
            for child_node in child_nodes.iter_mut()
            {
                parse_queue.push_back(child_node);
            }
        }
    }

    // Convert the parse tree to an S-Expression
    return make_s_expression(parse_root);
}

fn strip_comments(source: &str) -> String
{
    // Accumulate segments of the source string
    let mut segments: Vec<&str> = Vec::new();

    let mut start = 0;
    let mut end = 0;

    let mut inside_line_comment = false;

    let mut block_comment_open_count = 0;
    let mut block_comment_start_count = 0;
    let mut block_comment_end_count = 0;

    for (i, character) in source.chars().enumerate()
    {
        match character
        {
            // <
            symbols::keywords::BLOCK_COMMENT_START_CHAR =>
            {
                block_comment_start_count += 1;
                block_comment_end_count = 0;

                // This is potentially just a normal character, so it might end up in the output
                end = i + 1;

                if block_comment_start_count == symbols::keywords::BLOCK_COMMENT_CHAR_COUNT
                {
                    // This and the preceding open chars weren't normal characters, so remove them from the output
                    end -= symbols::keywords::BLOCK_COMMENT_CHAR_COUNT;

                    // This was the last start char needed to open a new block
                    block_comment_open_count += 1;
                    block_comment_start_count = 0;

                    if block_comment_open_count == 1
                    {
                        // This is the first open, so...

                        if start < end && !inside_line_comment
                        {
                            // Push any non-commented text if needed
                            // note: skip the other preceding open chars
                            start -= symbols::keywords::BLOCK_COMMENT_CHAR_COUNT - 1;
                            segments.push(&source[start..end]);

                            start = i + 1;
                            end = 0;
                        }
                    }
                }
            }
            // >
            symbols::keywords::BLOCK_COMMENT_END_CHAR =>
            {
                block_comment_end_count += 1;
                block_comment_start_count = 0;

                // This is potentially just a normal character, so it might end up in the output
                end = i + 1;

                if block_comment_end_count == symbols::keywords::BLOCK_COMMENT_CHAR_COUNT
                {
                    // This and the preceding open chars weren't normal characters, so remove them from the output
                    end -= symbols::keywords::BLOCK_COMMENT_CHAR_COUNT;

                    // This was the last end char needed to close a block

                    // Make sure there is a block to close
                    if block_comment_open_count > 0
                    {
                        block_comment_open_count -= 1;
                        block_comment_end_count = 0;

                        if block_comment_open_count == 0
                        {
                            // This closes the first open
                            start = i + 1;
                            end = 0;
                        }
                    }
                    else
                    {
                        panic!("Parse error: invalid block-comment closer!");
                    }
                }
            }
            c =>
            {
                // Not a block comment start or end character
                block_comment_start_count = 0;
                block_comment_end_count = 0;

                match c
                {
                    symbols::keywords::LINE_COMMENT_CHAR if !inside_line_comment =>
                    {
                        // We've started a new line comment
                        inside_line_comment = true;

                        // If non-comment text has been found and we aren't also in a block comment...
                        if start < end && block_comment_open_count == 0
                        {
                            segments.push(&source[start..end]);

                            // Reset the next segment
                            start = i + 1;
                            end = 0;
                        }
                    }
                    '\n' if inside_line_comment =>
                    {
                        // We're ending a line comment
                        inside_line_comment = false;

                        // The next segment starts after the newline
                        start = i + 1;
                        end = start;
                    }
                    '\r' if !inside_line_comment =>
                    {
                        // Skip over carriage returns in non-commented text
                        if start < end && block_comment_open_count == 0
                        {
                            segments.push(&source[start..end]);

                            // Reset the next segment
                            start = i + 1;
                            end = 0;
                        }
                        else
                        {
                            start = i + 1;
                        }
                    }
                    _ =>
                    {
                        if inside_line_comment || block_comment_open_count > 0
                        {
                            // Skip characters from commented text
                            start = i + 1;
                            end = 0;
                        }
                        else
                        {
                            // Extend the current segment with non-commented text
                            end = i + 1;
                        }
                    }
                }
            }
        }
    }

    if start < end && !inside_line_comment && block_comment_open_count == 0
    {
        // There are characters left that have not been pushed into a segment
        if start == 0
        {
            // There is only one segment, so just re-use the original string
            return String::from(&source[0..end]);
        }
        else
        {
            // Add the final segment
            segments.push(&source[start..end]);
        }
    }

    // Join all segments into the output String
    let mut result = String::new();
    for segment in segments
    {
        result.push_str(segment);
        result.push('\n');
    }
    return result;
}

fn expand_node(node: &mut ParseNode)
{
    // Helper functions
    fn is_pair(open: char, close: char) -> bool
    {
        match (open, close)
        {
            ('(', ')') => true,
            ('[', ']') => true,
            ('{', '}') => true,
            _ => false,
        }
    }
    fn push_symbol<'a>(
        source: &'a str,
        start: usize,
        length: usize,
        result: &mut Vec<ParseNode<'a>>,
    )
    {
        let range: &str = &source[start..start + length];
        result.push(ParseNode::Symbol(range));
    }
    fn push_unparsed<'a>(
        bracket: BracketType,
        source: &'a str,
        start: usize,
        length: usize,
        result: &mut Vec<ParseNode<'a>>,
    )
    {
        let range: &str = &source[start..start + length];
        result.push(ParseNode::Unparsed(bracket, range));
    }

    // Expand an unparsed node
    if let ParseNode::Unparsed(source_bracket, source) = node
    {
        // Index and length of the next node's source
        let mut start = 0;
        let mut length = 0;

        // Information about the current enclosing bracket
        let mut open_char = ' ';
        let mut open_count = 0;

        // Accumulate new parse nodes
        let mut result: Vec<ParseNode> = Vec::new();

        // Go through the source string character by character
        for (i, c) in source.chars().enumerate()
        {
            match c
            {
                ' ' | '\t' | '\n' | '\r' =>
                {
                    // Any whitespace character
                    if open_count > 0
                    {
                        // We're inside a list, so...
                        if length > 0
                        {
                            // Add to the length if non-whitespace characters have already been found
                            length += 1;
                        }
                        else
                        {
                            // Wait to start the list otherwise
                            start = i + 1;
                        }
                    }
                    else
                    {
                        // We're not inside a list
                        if length > 0
                        {
                            // This whitespace is the end of some symbol
                            push_symbol(source, start, length, &mut result);

                            start = i + 1;
                            length = 0;
                        }
                        else
                        {
                            // We're not inside a symbol (just in a sequence of whitespace)
                            start = i + 1;
                        }
                    }
                }

                '(' | '[' | '{' =>
                {
                    // Any opening bracket

                    if open_count > 0
                    {
                        // We're already inside a list, so...

                        // Extend the list to include this character
                        length += 1;

                        // Make sure this open is closed before the current list is closed
                        // (if the bracket type is the same)
                        if open_char == c
                        {
                            open_count += 1;
                        }
                    }
                    else
                    {
                        // We're not inside a list, so...

                        if length > 0
                        {
                            // This open is the end of some symbol
                            push_symbol(source, start, length, &mut result);
                        }

                        // This is now the opening bracket of the current list
                        open_char = c;
                        open_count = 1;

                        start = i + 1;
                        length = 0;
                    }
                }

                ')' | ']' | '}' =>
                {
                    // Any closing bracket

                    if open_count > 0 && is_pair(open_char, c)
                    {
                        // We're inside a list, and this is the same bracket type
                        open_count -= 1;

                        if open_count == 0
                        {
                            // This ends the current list
                            let bracket = match open_char
                            {
                                '(' => BracketType::Round,
                                '[' => BracketType::Square,
                                '{' => BracketType::Curly,
                                _ => BracketType::None,
                            };

                            // Create an unparsed child node
                            push_unparsed(bracket, source, start, length, &mut result);

                            start = i + 1;
                            length = 0;
                        }
                        else
                        {
                            // This is just a part of the current list
                            length += 1;
                        }
                    }
                    else
                    {
                        // We aren't inside a any list
                        // note: this is bad syntax
                        length += 1;
                    }
                }

                _ =>
                {
                    // Any other character
                    // note: not whitespace or bracket

                    // The current symbol or list is just extended
                    length += 1;
                }
            }
        }

        // All characters have been read

        if open_count > 0
        {
            // We ended inside an unclosed list
            // TODO: emit warning

            panic!("Unclosed list starting with `{}` @ `{}`", open_char, source);
        }

        if length > 0
        {
            // We ended inside a symbol before it could be ended by a whitespace character
            push_symbol(source, start, length, &mut result);
        }

        // Mark the node as expanded
        *node = ParseNode::List(*source_bracket, result);
    }
}

fn make_s_expression(node: ParseNode) -> Option<SExpression>
{
    match node
    {
        ParseNode::List(bracket, elements) =>
        {
            let mut new_elements: Vec<SExpression> = Vec::new();
            for element in elements
            {
                // Convert each child element into an S-Expression if possible
                if let Some(new_element) = make_s_expression(element)
                {
                    new_elements.push(new_element);
                }
            }

            return Some(SExpression::List(bracket, new_elements));
        }
        ParseNode::Symbol(symbol_str) =>
        {
            return Some(SExpression::Symbol(String::from(symbol_str)));
        }
        ParseNode::Unparsed(_, _) =>
        {
            // Some node has been left unparsed
            // note: this shouldn't happen?
            return None;
        }
    }
}
