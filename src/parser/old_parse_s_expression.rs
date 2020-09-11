use super::s_expression::*;
use crate::errors::s_expression_error::*;

use super::Parser;

impl Parser
{
    pub fn parse_s_expression(&self, text: String) -> SExpressionResult
    {
        let queue = ParseQueue::new(Rc::new(text));
        queue.expand_all()
    }
}

use crate::language::symbols;

use std::collections::VecDeque;
use std::rc::Rc;

struct ParseQueue
{
    shared_text: Rc<String>,
    root:        ParseNode,
    queue:       VecDeque<*mut ParseNode>,

    errors:   Vec<Error>,
    warnings: Vec<Error>,
}

impl ParseQueue
{
    pub fn new(shared_text: Rc<String>) -> ParseQueue
    {
        // The parse root includes the full source text enclosed in {...} braces

        let mut root = ParseNode::Unparsed(
            BracketType::Curly,
            Source::new(1, 0, shared_text.len(), &shared_text),
        );

        ParseQueue {
            shared_text,
            root,

            queue: VecDeque::new(),
            errors: Vec::new(),
            warnings: Vec::new(),
        }
    }

    pub fn expand_all(mut self) -> SExpressionResult
    {
        let root_ptr = (&mut self.root) as *mut ParseNode;
        self.queue.push_front(root_ptr);

        loop
        {
            // Get the next ParseNode pointer off the queue
            let next_ptr = match self.queue.pop_front()
            {
                Some(next_ptr) => next_ptr,
                None => break,
            };

            // Convert the pointer to a mutable reference
            let next = unsafe { &mut (*next_ptr) };

            // Expand the next ParseNode and check for any errors/warnings
            match next.expand()
            {
                ExpandResult::Ok(()) =>
                {}
                ExpandResult::Warn((), mut warnings) =>
                {
                    self.warnings.append(&mut warnings);
                }
                ExpandResult::Error(mut errors, mut warnings) =>
                {
                    self.errors.append(&mut errors);
                    self.warnings.append(&mut warnings);
                }
            }

            // Make sure the ParseNode was properly expanded
            match next
            {
                ParseNode::Parsed(..) | ParseNode::Comment(..) =>
                {
                    // Leave fully-parsed nodes as-is (ie. owned by the ParseQueue as the root
                    //  or by another ParseNode::List as a child)
                }
                ParseNode::List(_bracket_type, children, _source) =>
                {
                    // Come back to the current list AFTER handling all child nodes

                    self.queue.push_front(next_ptr);

                    for child in children.iter_mut()
                    {
                        let child_ptr = child as *mut ParseNode;
                        self.queue.push_front(child_ptr);
                    }
                }
                _ =>
                {
                    // A ParseNode will only be left unexpanded in the event of an error, which
                    //  we just checked. Don't add it back onto the queue, but do continue in case
                    //  there are more errors we can catch
                }
            }
        }

        // Fail if any errors were encountered
        if !self.errors.is_empty()
        {
            return SExpressionResult::Error(self.errors, self.warnings);
        }

        // Check if the root was expanded properly
        match self.root
        {
            ParseNode::Parsed(expression) =>
            {
                SExpressionResult::maybe_warn(expression, self.warnings)
            }
            ParseNode::Comment(..) =>
            {
                let error = Error::new(
                    ErrorKind::Internal("Root parse node is comment".to_owned()),
                    Source::empty(),
                );
                self.errors.push(error);

                SExpressionResult::Error(self.errors, self.warnings)
            }

            root =>
            {
                let error = Error::new(
                    ErrorKind::Internal(format!("Failed to expand root node: {:?}", root)),
                    Source::empty(),
                );
                self.errors.push(error);

                SExpressionResult::Error(self.errors, self.warnings)
            }
        }
    }
}

#[derive(Debug)]
enum ParseNode
{
    Unparsed(BracketType, Source),
    List(BracketType, Vec<ParseNode>, Source),
    Comment(String, Source),
    Parsed(SExpression),
    Error(Source),
}

impl ParseNode
{
    pub fn get_source(&self) -> Source
    {
        match self
        {
            ParseNode::Unparsed(_, source) => source.clone(),
            ParseNode::List(_, _, source) => source.clone(),
            ParseNode::Comment(_, source) => source.clone(),
            ParseNode::Parsed(s_expression) => s_expression.get_source(),
            ParseNode::Error(source) => source.clone(),
        }
    }
}

impl ParseNode
{
    pub fn expand(&mut self) -> ExpandResult<()>
    {
        match self
        {
            ParseNode::Unparsed(bracket, source) =>
            {
                // Try to break an unparsed block of text into a list of child ParseNodes

                match scan_text(source)
                {
                    ExpandResult::Ok(children) =>
                    {
                        let mut new_node = ParseNode::List(*bracket, children, source.clone());
                        std::mem::swap(self, &mut new_node);

                        ExpandResult::Ok(())
                    }
                    ExpandResult::Warn(children, warnings) =>
                    {
                        let mut new_node = ParseNode::List(*bracket, children, source.clone());
                        std::mem::swap(self, &mut new_node);

                        ExpandResult::Warn((), warnings)
                    }
                    ExpandResult::Error(errors, warnings) =>
                    {
                        *self = ParseNode::Error(source.clone());
                        ExpandResult::Error(errors, warnings)
                    }
                }
            }
            ParseNode::List(bracket, list_children, source) =>
            {
                // Try to turn a list of potentially parsed items into a fully parsed SExpression

                // Extract the list's children to take ownership
                let mut children = Vec::new();
                std::mem::swap(&mut children, list_children);

                // Make sure all children are fully parsed
                let mut child_s_expressions = Vec::new();
                let mut errors = Vec::new();

                for child in children
                {
                    match child
                    {
                        ParseNode::Parsed(s_expression) => child_s_expressions.push(s_expression),
                        ParseNode::Comment(..) => (),
                        _ => errors.push(Error::new(ErrorKind::FailedToParse, child.get_source())),
                    }
                }

                if errors.is_empty()
                {
                    // If we have no errors, we can construct a fully parsed SExpression list

                    let s_expression =
                        SExpression::List(*bracket, child_s_expressions, source.clone());
                    *self = ParseNode::Parsed(s_expression);

                    ExpandResult::Ok(())
                }
                else
                {
                    *self = ParseNode::Error(source.clone());
                    ExpandResult::Error(errors, Vec::new())
                }
            }
            _ =>
            {
                // Other items don't need to be parsed further

                ExpandResult::Ok(())
            }
        }
    }
}

// Read one level of ParseNodes from a source block
fn scan_text(source: &Source) -> ExpandResult<Vec<ParseNode>>
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

                    let new_source = source.get_range(start_line, first + 1, last);
                    let new_node = ParseNode::Unparsed(bracket, new_source);
                    result.push(new_node);

                    // The end of a list is part of that list
                    check_starts_group = false;
                }
                Group::Symbol(first, last, start_line) =>
                {
                    let symbol = text[first..last + 1].to_owned();

                    let new_source = source.get_range(start_line, first, last + 1);
                    let new_node = ParseNode::Parsed(SExpression::Symbol(symbol, new_source));
                    result.push(new_node);

                    // A character that ends a symbol could start a list, comment, etc.
                    check_starts_group = true;
                }
                Group::LineComment(first, last, start_line) =>
                {
                    let content = text[first + 1..last].to_owned();
                    let new_source = source.get_range(start_line, first, last);
                    let new_node = ParseNode::Comment(content, new_source);
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
                let end = if text.len() > first + 30
                {
                    first + 30
                }
                else
                {
                    text.len()
                };

                let mut open_string = String::new();
                open_string.push(open);

                let new_source = source.get_range(start_line, first, end);
                errors.push(Error::new(
                    ErrorKind::UnclosedBracket(open_string),
                    new_source,
                ));
            }

            Group::Symbol(first, _, start_line) =>
            {
                let end = text.len();
                let symbol = text[first..end].to_owned();

                let new_source = source.get_range(start_line, first, end);
                let new_node = ParseNode::Parsed(SExpression::Symbol(symbol, new_source));
                result.push(new_node);
            }

            Group::LineComment(..) =>
            {
                // Don't do anything with line comments
            }
        }
    }

    if errors.is_empty()
    {
        ExpandResult::maybe_warn(result, warnings)
    }
    else
    {
        ExpandResult::Error(errors, warnings)
    }
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
