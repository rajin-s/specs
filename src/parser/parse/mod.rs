mod internal;

mod parse_node;
mod parse_type;
mod parse_atomic;
mod parse_function;

use crate::s_expression::*;
use crate::language::nodes::*;

// Parse an expression into a node using a ParseQueue
pub fn parse_root_expression(expression: &SExpression, context: &mut Context) -> Option<Node>
{
    use std::rc::Rc;
    use internal::*;

    let mut queue = ParseQueue::new(expression);
    while queue.expand_next(context)
    {
        // Keep expanding until the queue is empty
    }

    let root = queue.take_root();
    match Rc::try_unwrap(root)
    {
        Ok(root) => match root
        {
            ParseItem::CompleteNode(node) => Some(node),
            item =>
            {
                context.add_error(errors::INTERNAL, "Parse root isn't completed node", item);
                None
            }
        },
        Err(_) =>
        {
            context.add_error(errors::INTERNAL, "Failed to unwrap parse root", "");
            None
        }
    }
}

pub struct Message
{
    name:    String,
    message: String,
    source:  String,
}
pub struct Context
{
    errors:   Vec<Message>,
    warnings: Vec<Message>,
}

impl Message
{
    pub fn new<TName, TMessage, TSource>(name: TName, message: TMessage, source: TSource) -> Self
    where
        TName: ToString,
        TMessage: ToString,
        TSource: ToString,
    {
        return Self {
            name:    name.to_string(),
            message: message.to_string(),
            source:  source.to_string(),
        };
    }
}
impl Context
{
    pub fn new() -> Self
    {
        return Self {
            errors:   Vec::new(),
            warnings: Vec::new(),
        };
    }
    // Deconstruct the context to get the error and warning lists
    pub fn get_messages(self) -> (Vec<Message>, Vec<Message>)
    {
        return (self.errors, self.warnings);
    }
    pub fn add_error<TName, TMessage, TSource>(
        &mut self,
        name: TName,
        message: TMessage,
        source: TSource,
    ) where
        TName: ToString,
        TMessage: ToString,
        TSource: ToString,
    {
        self.errors.push(Message::new(name, message, source));
    }
    pub fn add_warning<TName, TMessage, TSource>(
        &mut self,
        name: TName,
        message: TMessage,
        source: TSource,
    ) where
        TName: ToString,
        TMessage: ToString,
        TSource: ToString,
    {
        self.warnings.push(Message::new(name, message, source));
    }
}
impl std::fmt::Display for Message
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result
    {
        write!(f, "[{}] : {}\n\t@ {}", self.name, self.message, self.source)
    }
}
