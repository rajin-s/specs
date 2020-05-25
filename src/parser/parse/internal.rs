use std::collections::VecDeque;
use std::rc::Rc;

use crate::language::nodes::*;
use crate::s_expression::*;

use super::parse_node;
use super::parse_type;
use super::Context;

pub mod imports
{
    pub use std::rc::Rc;

    pub use crate::s_expression::*;
    pub use SExpression::*;

    pub use super::super::Context;
    pub use crate::language::nodes::*;
    pub use crate::language::symbols::*;
}

pub mod errors
{
    pub const INTERNAL: &str = "ParserInternal";
    pub const FAILED_NODE: &str = "InvalidNode";
    pub const FAILED_TYPE: &str = "InvalidType";
    pub const FAILED_SYMBOL: &str = "InvalidSymbol";
    pub const FAILED_ARGUMENT: &str = "InvalidArgument";
    pub const FAILED_FUNCTION: &str = "InvalidFunction";
    pub const FAILED_TYPE_DEFINITION: &str = "InvalidTypeDefinition";
    pub const FAILED_CONSTRUCT: &str = "ConstructFailed";
}

/* -------------------------------------------------------------------------- */
/*                                   Structs                                  */
/* -------------------------------------------------------------------------- */

// An item that represents some entity that is
//  - An unparsed expression that will eventually become a node/type
//  - An expanded partial node/type that is waiting for child items to be completed
//  - A completed node/type
pub enum ParseItem<'a>
{
    Empty,
    UnparsedNode(&'a SExpression),
    PartialNode(PartialNodeData<'a>),
    CompleteNode(Node),
    UnparsedType(&'a SExpression),
    PartialType(PartialTypeData<'a>),
    CompleteType(Type),
}

// A nicer name for Rc<ParseItem<'a>>
pub type ParseItemReference<'a> = Rc<ParseItem<'a>>;
pub fn track_unparsed_child_item<'a>(
    expression: &'a SExpression,
    f: fn(&'a SExpression) -> ParseItem<'a>,
    items: &mut Vec<ParseItemReference<'a>>,
) -> ParseItemReference<'a>
{
    let new_ref = Rc::new(f(expression));
    items.push(new_ref.clone());
    new_ref
}

// Data needed to construct a node
//  - References to other parse items start out as expressions
//  - Once all references point to completed nodes/types, the final node can be constructed
#[derive(Debug)]
pub enum PartialNodeData<'a>
{
    Empty,

    Call(ParseItemReference<'a>, Vec<ParseItemReference<'a>>),

    Reference(Reference, ParseItemReference<'a>),
    Dereference(ParseItemReference<'a>),

    Binding(String, ParseItemReference<'a>),
    Assignment(ParseItemReference<'a>, ParseItemReference<'a>),
    Sequence(bool, Vec<ParseItemReference<'a>>),
    Conditional(
        ParseItemReference<'a>,
        ParseItemReference<'a>,
        ParseItemReference<'a>,
    ),

    Function(PartialFunctionData<'a>),
    Type(PartialTypeDefinitionData<'a>),

    Access(ParseItemReference<'a>, String),
}

// More concise types for partial arguments, members, and methods
pub type PartialArgumentData<'a> = (String, ParseItemReference<'a>);
pub type PartialMemberData<'a> = (
    String,
    ParseItemReference<'a>,
    MemberScope,
    Visibility,
    Visibility,
);
pub type PartialFunctionData<'a> = (
    String,
    Vec<PartialArgumentData<'a>>,
    ParseItemReference<'a>,
    ParseItemReference<'a>,
);
pub type PartialMethodData<'a> = (PartialFunctionData<'a>, MemberScope, Visibility);
pub type PartialTypeDefinitionData<'a> = (
    String,
    Vec<PartialMemberData<'a>>,
    Vec<PartialMethodData<'a>>,
    Vec<String>,
);

// Data needed to construct a type
//  - References to other parse items start out as expressions
//  - Once all references point to completed types, the final type can be constructed
#[derive(Debug)]
pub enum PartialTypeData<'a>
{
    Empty,
    Function(Vec<ParseItemReference<'a>>, ParseItemReference<'a>),
}

/* -------------------------------------------------------------------------- */
/*                                 Parse Queue                                */
/* -------------------------------------------------------------------------- */

// A container structure to manage expansion of ParseItems
//  - Keeps a reference to the root item that can be used to extract the (hopefully) finished node
//  - Keeps references to incomplete ParseItems in a queue such that child items are fully expanded before their parent
pub struct ParseQueue<'a>
{
    root:  ParseItemReference<'a>,
    queue: VecDeque<ParseItemReference<'a>>,
}
impl<'a> ParseQueue<'a>
{
    // Create a new parse queue with the given root
    pub fn new(root: &'a SExpression) -> Self
    {
        let root_item = Rc::new(ParseItem::UnparsedNode(root));
        let mut new_queue = Self {
            root:  root_item.clone(),
            queue: VecDeque::new(),
        };
        new_queue.queue.push_front(root_item);
        return new_queue;
    }

    // Destructure the queue to get the root item out once everything's been expanded
    pub fn take_root(self) -> ParseItemReference<'a>
    {
        return self.root;
    }

    // Get the next item from the queue and expand it, potentially adding more items to the front of the queue
    //  - returns true if anything was expanded
    pub fn expand_next(&mut self, context: &mut Context) -> bool
    {
        match self.queue.pop_front()
        {
            Some(mut item) =>
            {
                // Keep a copy around in case we need to add this item back onto the queue
                let item_clone = Rc::clone(&item);

                // Destructure Rc to get the actual reference
                // note: unsafe seemingly needed to keep references to items and child items in the queue at the same time
                //       :(
                let item_ref = unsafe { Rc::get_mut_unchecked(&mut item) };

                match item_ref
                {
                    ParseItem::UnparsedNode(expression) =>
                    {
                        // Turn an unexpanded expression into
                        //  - a partially-expanded node, then place it back on the queue after all its dependencies
                        //  - a fully-expanded node, then leave it off the queue so its parent node can be completed

                        match parse_node::parse_expression(expression, context)
                        {
                            parse_node::ParseExpressionResult::Complete(node) =>
                            {
                                // The expression was fully expanded into a node
                                //  ie. for atomic nodes, etc.
                                *item_ref = ParseItem::CompleteNode(node);
                            }
                            parse_node::ParseExpressionResult::Partial(
                                partial_node,
                                child_items,
                            ) =>
                            {
                                // The expression was partially expanded, so we need to
                                //  - Put the original item back onto the queue
                                //  - Put all child items onto the queue in front of the original item

                                // Add the original item to the front of the queue
                                //  note: we use the clone from before, which points to the same location
                                //        that will be updated with the new PartialNode
                                self.queue.push_front(item_clone);

                                // Handle each child expression
                                for item in child_items
                                {
                                    self.queue.push_front(item);
                                }

                                // Update the original item to be a partial node
                                *item_ref = ParseItem::PartialNode(partial_node);
                            }
                            parse_node::ParseExpressionResult::Error =>
                            {
                                context.add_error(
                                    errors::FAILED_NODE,
                                    "Failed to parse node from expression",
                                    expression,
                                );
                            }
                        }
                    }
                    ParseItem::UnparsedType(expression) =>
                    {
                        // Turn an unexpanded expression into
                        //  - a partially-expanded type, then place it back on the queue after all its dependencies
                        //  - a fully-expanded type, then leave it off the queue so its parent item can be completed

                        match parse_type::parse_expression(expression, context)
                        {
                            parse_type::ParseExpressionResult::Complete(t) =>
                            {
                                // The expression was fully expanded into a type
                                //  ie. for primitive types (int, etc.)
                                *item_ref = ParseItem::CompleteType(t);
                            }
                            parse_type::ParseExpressionResult::Partial(
                                partial_type,
                                child_items,
                            ) =>
                            {
                                // The expression was partially expanded, so we need to
                                //  - Put the original item back onto the queue
                                //  - Put all child items onto the queue in front of the original item

                                // Add the original item to the front of the queue
                                //  note: we use the clone from before, which points to the same location
                                //        that will be updated with the new PartialType
                                self.queue.push_front(item_clone);

                                // Handle each child expression
                                for item in child_items
                                {
                                    self.queue.push_front(item);
                                }

                                // Update the original item to be a partial type
                                *item_ref = ParseItem::PartialType(partial_type);
                            }
                            parse_type::ParseExpressionResult::Error =>
                            {
                                context.add_error(
                                    errors::FAILED_TYPE,
                                    "Failed to parse type from expression",
                                    expression,
                                );
                            }
                        }
                    }

                    ParseItem::PartialNode(data) =>
                    {
                        // The item was waiting for its dependencies to be expanded,
                        // so now we can hopefully turn it into a completed node

                        // First, we extract the original partial data to use in the build function
                        let mut node_data = PartialNodeData::Empty;
                        std::mem::swap(&mut node_data, data);

                        // Call the build function to hopefully produce a new complete node
                        match parse_node::construct(node_data, context)
                        {
                            Some(node) =>
                            {
                                // Write to the original location with the new completed node
                                *item_ref = ParseItem::CompleteNode(node);
                            }
                            None =>
                            {
                                // The build function wasn't able to produce a completed node
                                context.add_error(
                                    errors::INTERNAL,
                                    "Failed to construct node",
                                    item_ref,
                                );
                            }
                        }
                    }
                    ParseItem::PartialType(data) =>
                    {
                        // The item was waiting for its dependencies to be expanded,
                        // so now we can hopefully turn it into a completed type

                        // First, we extract the original partial data to use in the build function
                        let mut type_data = PartialTypeData::Empty;
                        std::mem::swap(&mut type_data, data);

                        // Call the build function to hopefully produce a new complete node
                        match parse_type::construct(type_data, context)
                        {
                            Some(node) =>
                            {
                                // Write to the original location with the new completed node
                                *item_ref = ParseItem::CompleteType(node);
                            }
                            None =>
                            {
                                // The build function wasn't able to produce a completed node
                                context.add_error(
                                    errors::INTERNAL,
                                    "Failed to construct type",
                                    item_ref,
                                );
                            }
                        }
                    }

                    ParseItem::CompleteNode(_) | ParseItem::CompleteType(_) =>
                    {
                        // The item is fully expanded, so it can be left as-is
                        //  note: should completed items ever be placed on the parse queue???
                    }

                    item =>
                    {
                        // Some invalid ParseItem was found
                        context.add_error(
                            errors::INTERNAL,
                            "Trying to expand invalid ParseItem",
                            item,
                        );
                    }
                }

                true
            }
            None => false,
        }
    }
}

/* -------------------------------------------------------------------------- */
/*                                   Display                                  */
/* -------------------------------------------------------------------------- */

use std::fmt;
impl fmt::Display for ParseItem<'_>
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        match self
        {
            ParseItem::Empty => write!(f, "(empty)"),
            ParseItem::UnparsedNode(e) => write!(f, "(node-expr {})", e),
            ParseItem::PartialNode(data) => write!(f, "(node-partial {:?})", data),
            ParseItem::CompleteNode(n) => write!(f, "(node {})", n),
            ParseItem::UnparsedType(e) => write!(f, "(type-expr {})", e),
            ParseItem::PartialType(data) => write!(f, "(type-partial {:?})", data),
            ParseItem::CompleteType(n) => write!(f, "(type {})", n),
        }
    }
}
impl fmt::Debug for ParseItem<'_>
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        return fmt::Display::fmt(self, f);
    }
}

impl fmt::Display for PartialNodeData<'_>
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        return fmt::Debug::fmt(self, f);
    }
}
impl fmt::Display for PartialTypeData<'_>
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        return fmt::Debug::fmt(self, f);
    }
}
