#[macro_use]
pub mod macros;

pub mod atomic;
pub mod binding;
pub mod control;
pub mod definition;
pub mod internal;
pub mod operator;
pub mod primitive;

pub use atomic::*;
pub use binding::*;
pub use control::*;
pub use definition::*;
pub use internal::*;
pub use operator::*;
pub use primitive::*;

pub use super::types::*;
pub use super::ReferenceMode;

use crate::source::Source;
use crate::utilities::Indirect;
use crate::utilities::Recur;

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
        
        #[derive(Clone, Copy, PartialEq, Eq, Debug)]
        pub enum NodeKind
        {
            // Generate NodeKind::* variants
            $( $name ),*
        }

        impl Recur<Node> for Node
        {
            // Pass get_children to data
    
            fn get_children(&self) -> Vec<&Node>
            {
                match self
                {
                    $(
                        Node::$name(data) => data.get_children(),
                    )*
                }
            }
    
            fn get_children_mut(&mut self) -> Vec<&mut Node>
            {
                match self
                {
                    $(
                        Node::$name(data) => data.get_children_mut(),
                    )*
                }
            }
        }

        impl Node
        {
            // Alternative syntax to call to_node() on data

            pub fn from<T: ToNode>(data: T) -> Node
            {
                data.to_node()
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

            // Debugging helpers

            pub fn get_source(&self) -> Source
            {
                match self
                {
                    $(
                        Node::$name(data) => data.get_source(),
                    )*
                }
            }

            pub fn get_name(&self) -> String
            {
                let s = match self
                {
                    $(
                        Node::$name(_) => std::stringify!([$name]),
                    )*
                };
                return String::from(s);
            }
            
            pub fn get_kind(&self) -> NodeKind
            {
                match self
                {
                    $(
                        Node::$name(_) => NodeKind::$name,
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
    Nothing : atomic::Nothing,
    Comment : atomic::Comment,

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
    Class    : definition::Class,

    CNode : internal::CNode,
}

impl Node
{
    pub fn nothing(source: Source) -> Node
    {
        Node::Nothing(atomic::Nothing::new(source))
    }
    pub fn nothing_typed(node_type: Indirect<Type>, source: Source) -> Node
    {
        Node::Nothing(atomic::Nothing::new_typed(node_type, source))
    }

    /// 
    /// Check if a node is any kind of definition
    /// 
    pub fn is_definition(&self) -> bool
    {
        match self
        {
            Node::Function(_) => true,
            _ => false,
        }
    }

    /// 
    /// Check if a node is complex (ie can't be used as a function argument in C)
    /// 
    pub fn is_complex(&self) -> bool
    {
        match self
        {
            Node::Sequence(_) | Node::Conditional(_) | Node::Function(_) | Node::Class(_) => true,
            _ => false,
        }
    }

    ///
    /// Extract this node and replace it with some other node
    ///
    pub fn extract(&mut self, replacement: Node) -> Node
    {
        let mut temp = replacement;
        std::mem::swap(&mut temp, self);
        temp
    }
    
    ///
    /// Extract this node and replace it with a comment
    ///
    pub fn extract_comment(&mut self, message: String) -> Node
    {
        let mut temp = Comment::new(message, self.get_source()).to_node();
        std::mem::swap(&mut temp, self);
        temp
    }

    ///
    /// Extract this node and replace it with a temporary Nothing node
    ///
    pub fn extract_temp(&mut self) -> Node
    {
        let mut temp = Node::nothing(Source::empty());
        std::mem::swap(&mut temp, self);
        temp
    }

    ///
    /// Get the node's type and compare it against some other type
    /// 
    pub fn is_type(&self, t: &Type) -> bool
    {
        let t_indirect = self.get_type();
        let t_ref = t_indirect.borrow();

        &*t_ref == t
    }
}

pub type OtherNode = Box<Node>;
pub type OtherNodes = Vec<Node>;
use crate::utilities::Ref;

/* -------------------------------------------------------------------------- */
/*                                   Traits                                   */
/* -------------------------------------------------------------------------- */

pub trait ToNode
{
    fn to_node(self) -> Node;
}
