use crate::utilities::*;

use crate::language::s_expression::*;
use crate::language::symbols;

use crate::errors::s_expression_error::*;
use crate::source::Source;

impl super::Parser
{
    pub fn make_s_expression(&self, text: String) -> ResultLog<SExpression, Error>
    {
        let line_count = text.split('\n').fold(0, |n, _e| n + 1);
        let root_source = Source::new(0, line_count, 0, text.len(), text);

        let mut root_node = ParseNode::Unparsed(BracketType::Curly, root_source);
        let mut transform = ParseTransform::new();

        let warnings = match transform.apply(&mut root_node)
        {
            ResultLog::Ok(()) => Vec::new(),
            ResultLog::Warn((), warnings) => warnings,
            ResultLog::Error(errors, warnings) =>
            {
                return ResultLog::Error(errors, warnings);
            }
        };

        match root_node
        {
            ParseNode::Parsed(s_expression) => ResultLog::maybe_warn(s_expression, warnings),
            root_node =>
            {
                let error = Error::Internal(format!("Failed to parse root node: {:?}", root_node));
                ResultLog::Error(vec![error], warnings)
            }
        }
    }
}

#[derive(Debug)]
enum ParseNode
{
    Parsed(SExpression),
    Unparsed(BracketType, Source),
    PartialList(BracketType, Vec<ParseNode>, Source),
    Comment(Source),
}

impl Recur<ParseNode> for ParseNode
{
    fn get_children(&self) -> Vec<&ParseNode>
    {
        match self
        {
            ParseNode::PartialList(_, children, _) => children.iter().collect(),
            _ => Vec::new(),
        }
    }
    fn get_children_mut(&mut self) -> Vec<&mut ParseNode>
    {
        match self
        {
            ParseNode::PartialList(_, children, _) => children.iter_mut().collect(),
            _ => Vec::new(),
        }
    }
}

///
/// Parse transformation requires no state
/// 
struct ParseState {}

///
/// Recursive parse transformation
/// 
struct ParseTransform {}

impl ParseTransform
{
    pub fn new() -> ParseTransform
    {
        ParseTransform {}
    }
}

impl RecurTransform<ParseNode, ParseState, Error> for ParseTransform
{
    fn get_root_state(&mut self, _root: &ParseNode) -> ParseState
    {
        ParseState {}
    }

    fn enter(&mut self, node: &mut ParseNode, _state: &mut ParseState) -> ResultLog<(), Error>
    {
        match node
        {
            ParseNode::Unparsed(bracket, source) =>
            {
                // Take unparsed nodes and try to create a partial list

                match scan_text(source)
                {
                    ResultLog::Ok(children) =>
                    {
                        let new_node = ParseNode::PartialList(*bracket, children, source.clone());
                        *node = new_node;

                        ResultLog::Ok(())
                    }
                    ResultLog::Warn(children, warnings) =>
                    {
                        let new_node = ParseNode::PartialList(*bracket, children, source.clone());
                        *node = new_node;

                        ResultLog::Warn((), warnings)
                    }
                    ResultLog::Error(errors, warnings) => ResultLog::Error(errors, warnings),
                }
            }

            _ =>
            {
                // All other nodes can just be left as-is

                ResultLog::Ok(())
            }
        }
    }
    fn exit(&mut self, node: &mut ParseNode, _state: &mut ParseState) -> ResultLog<(), Error>
    {
        match node
        {
            ParseNode::Unparsed(..) =>
            {
                // All nodes should have been expanded into something at this point

                ResultLog::new_error(Error::Internal(format!(
                    "Failed to expand parse node: {:?}",
                    node
                )))
            }
            ParseNode::PartialList(bracket, children, source) =>
            {
                // Take a partial list and turn it into an SExpression list

                let original_children = std::mem::take(children);

                let mut new_children = Vec::new();
                let mut errors = Vec::new();

                for child in original_children
                {
                    match child
                    {
                        ParseNode::Parsed(s_expression) =>
                        {
                            // Keep fully parsed child nodes

                            new_children.push(s_expression);
                        }
                        ParseNode::Comment(..) =>
                        {
                            // Discard child comment nodes
                        }
                        child =>
                        {
                            let error = Error::Internal(format!(
                                "Failed to expand child parse node: {:?}",
                                child
                            ));
                            errors.push(error);
                        }
                    }
                }

                let mut new_source = source.clone();
                new_source.extend(1);

                *node = ParseNode::Parsed(SExpression::List(*bracket, new_children, new_source));

                ResultLog::maybe_error((), Vec::new(), errors)
            }
            _ =>
            {
                // Leave all other nodes as-is

                ResultLog::Ok(())
            }
        }
    }
}

// Read one level of ParseNodes from a source block
fn scan_text(source: &Source) -> ResultLog<Vec<ParseNode>, Error>
{
    enum Group
    {
        List(char, usize, usize, usize, usize),
        Symbol(usize, usize, usize),
        LineComment(usize, usize, usize),
    }

    let mut result = Vec::new();
    let mut errors = Vec::new();
    let mut warnings = Vec::new();
    let mut line = 0;

    let mut group_stack = Vec::new();

    let text = source.get_text();

    for (i, c) in text.chars().enumerate()
    {
        // Check if the current characters ends the current group

        let mut end_current_group = false;
        let mut inside_line_comment = false;

        if c == '\n'
        {
            line += 1;
        }

        match group_stack.last_mut()
        {
            None => (),
            Some(Group::List(open, open_count, _first, last, _start_line)) =>
            {
                // We're checking for open/close characters to end this list

                if c == *open
                {
                    *open_count += 1;
                }
                else if is_bracket_pair(*open, c)
                {
                    *open_count -= 1;

                    if *open_count == 0
                    {
                        // The current list ends at this character

                        *last = i;
                        end_current_group = true;
                    }
                }
            }
            Some(Group::Symbol(_first, last, _start_line)) =>
            {
                // We're checking for whitespace, list delimiters, or comment delimiters to end this symbol

                if !is_symbol_char(c)
                {
                    // The current symbol ends at the symbol before this one

                    *last = i - 1;
                    end_current_group = true;
                }
            }
            Some(Group::LineComment(_first, last, _start_line)) =>
            {
                // We're checking for a line ending to end this comment

                inside_line_comment = true;

                if is_line_comment_end(c)
                {
                    // The current line comment ends at the symbol before this one

                    *last = i - 1;
                    end_current_group = true;
                }
            }
        }

        // By default, only check for group starts if we have no groups
        let mut check_starts_group = group_stack.is_empty();

        if end_current_group
        {
            match group_stack.pop().expect("Unexpected some group")
            {
                Group::List(open, _, first, last, start_line) =>
                {
                    let bracket = match open
                    {
                        '(' => BracketType::Round,
                        '{' => BracketType::Curly,
                        '[' => BracketType::Square,
                        _ => BracketType::None,
                    };

                    let new_source = source.get_range(start_line, line, first + 1, last);
                    let new_node = ParseNode::Unparsed(bracket, new_source);
                    result.push(new_node);

                    // The end of a list is part of that list
                    check_starts_group = false;
                }
                Group::Symbol(first, last, start_line) =>
                {
                    let symbol = text[first..last + 1].to_owned();

                    let new_source = source.get_range(start_line, line, first, last + 1);
                    let new_node = ParseNode::Parsed(SExpression::Symbol(symbol, new_source));
                    result.push(new_node);

                    // A character that ends a symbol could start a list, comment, etc.
                    check_starts_group = true;
                }
                Group::LineComment(first, last, start_line) =>
                {
                    let new_source = source.get_range(start_line, line, first, last);
                    let new_node = ParseNode::Comment(new_source);
                    result.push(new_node);

                    // The end of a comment is part of that comment
                    check_starts_group = false;
                }
            }
        }

        if check_starts_group
        {
            if is_bracket_open(c)
            {
                // Start a list group if we hit an open character

                let new_group = Group::List(c, 1, i, i, line);
                group_stack.push(new_group);
            }
            else if is_symbol_char(c) && group_stack.is_empty()
            {
                // Start a symbol group if we don't already have something

                let new_group = Group::Symbol(i, i, line);
                group_stack.push(new_group);
            }
        }

        // Always check for line comments starting (if we aren't already in one)

        if !inside_line_comment && is_line_comment_start(c)
        {
            let new_group = Group::LineComment(i, i, line);
            group_stack.push(new_group);
        }
    }

    // Check remaining groups for symbols or unclosed lists

    for group in group_stack
    {
        match group
        {
            Group::List(open, _, first, _, start_line) =>
            {
                let mut open_string = String::new();
                open_string.push(open);

                let new_source = source.get_range(start_line, line, first, text.len());
                let error = Error::UnclosedBracket(open_string, new_source);

                errors.push(error);
            }

            Group::Symbol(first, _, start_line) =>
            {
                let end = text.len();
                let symbol = text[first..end].to_owned();

                let new_source = source.get_range(start_line, line, first, end);
                let new_node = ParseNode::Parsed(SExpression::Symbol(symbol, new_source));
                result.push(new_node);
            }

            Group::LineComment(..) =>
            {
                // Don't do anything with line comments
            }
        }
    }

    ResultLog::maybe_error(result, errors, warnings)
}

// Text scanning helper functions

fn is_bracket_pair(open: char, close: char) -> bool
{
    match (open, close)
    {
        ('(', ')') => true,
        ('[', ']') => true,
        ('{', '}') => true,
        _ => false,
    }
}

fn is_whitespace(c: char) -> bool
{
    match c
    {
        ' ' | '\n' | '\t' | '\r' => true,
        _ => false,
    }
}

fn is_bracket_open(c: char) -> bool
{
    match c
    {
        '(' | '[' | '{' => true,
        _ => false,
    }
}

fn is_bracket_close(c: char) -> bool
{
    match c
    {
        ')' | ']' | '}' => true,
        _ => false,
    }
}

fn is_line_comment_start(c: char) -> bool
{
    match c
    {
        symbols::keywords::LINE_COMMENT_CHAR => true,
        _ => false,
    }
}

fn is_line_comment_end(c: char) -> bool
{
    match c
    {
        '\n' => true,
        _ => false,
    }
}

fn is_symbol_char(c: char) -> bool
{
    if is_whitespace(c) || is_bracket_close(c) || is_bracket_open(c) || is_line_comment_start(c)
    {
        false
    }
    else
    {
        true
    }
}
