pub mod atomic;
pub mod binding;
pub mod control;
pub mod definition;
pub mod operator;

pub mod primitive;

pub mod all
{
    pub use super::atomic::*;
    pub use super::binding::*;
    pub use super::control::*;
    pub use super::definition::*;

    pub use super::primitive::*;

    pub use super::*;
}

use super::types::*;
use crate::utilities::Indirect;

/* -------------------------------------------------------------------------- */
/*                                    Node                                    */
/* -------------------------------------------------------------------------- */

// Build the Node enum type and associated boilerplate
macro_rules! nodes {
    { $( $name:ident : $data:path, )* } => {
        #[derive(Debug)]
        pub enum Node
        {
            // Generate Node::* variants
            $( $name($data) ),*
        }

        impl Node
        {
            // Alternative syntax to call to_node() on data
            pub fn from<T: ToNode>(data: T) -> Node
            {
                data.to_node()
            }

            // Pass get_children to data
            pub fn get_children(&self) -> Vec<OtherNode>
            {
                match self
                {
                    $(
                        Node::$name(data) => data.get_children(),
                    )*
                }
            }

            // Pass get_type to data
            pub fn get_type(&self) -> Indirect<Type>
            {
                match self
                {
                    $(
                        Node::$name(data) => data.get_type(),
                    )*
                }
            }
            // Pass borrow_type to data
            pub fn borrow_type(&self) -> Ref<Type>
            {
                match self
                {
                    $(
                        Node::$name(data) => data.borrow_type(),
                    )*
                }
            }
        }
        
        $(
            impl ToNode for $data
            {
                fn to_node(self) -> Node
                {
                    Node::$name(self)
                }
            }
        )*

        impl std::fmt::Display for Node
        {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result
            {
                match self
                {
                    // Forward all format calls to data
                    $(
                        Node::$name(data) => write!(f, "{}", data),
                    )*
                }
            }
        }
    };
}

// Specify Node variants and associated data
nodes! {
    Nothing           : atomic::Nothing,
    Integer           : atomic::Integer,
    Boolean           : atomic::Boolean,
    Variable          : atomic::Variable,
    PrimitiveOperator : atomic::PrimitiveOperator,

    Call        : operator::Call,
    Reference   : operator::Reference,
    Dereference : operator::Dereference,
    Assign      : operator::Assign,
    Access      : operator::Access,

    Binding : binding::Binding,

    Sequence    : control::Sequence,
    Conditional : control::Conditional,

    Function : definition::Function,
}

impl Node
{
    pub fn is_definition(&self) -> bool
    {
        match self
        {
            Node::Function(_) => true,
            _ => false,
        }
    }
}

pub type OtherNode = Indirect<Node>;
pub type OtherNodes = Vec<Indirect<Node>>;
pub use std::cell::{Ref, RefMut};

/* -------------------------------------------------------------------------- */
/*                                   Traits                                   */
/* -------------------------------------------------------------------------- */

pub trait ToNode
{
    fn to_node(self) -> Node;
}