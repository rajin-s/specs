use super::*;

/* -------------------------------------------------------------------------- */
/*                                   C Nodes                                  */
/* -------------------------------------------------------------------------- */

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum CNodeType
{
    Atomic,
    InfixOperator,
    PrefixOperator,
    Bare,
    Wrapped,
    Block,
    BlockTransparent,
    Conditional,
    Call,
    Function,
    Struct,
    Comment,
    Nothing,
}

#[derive(Debug)]
pub struct CNode
{
    cnode_type: CNodeType,
    text:       String,

    node_type: Indirect<Type>,
    source:    Source,
}
impl CNode
{
    pub fn new(
        cnode_type: CNodeType,
        text: String,
        node_type: Indirect<Type>,
        source: Source,
    ) -> Self
    {
        Self {
            cnode_type,
            text,
            node_type,
            source,
        }
    }

    get!(get_node_type -> cnode_type : CNodeType);
    get!(get_text -> text : &String);

    get!(get_type -> node_type.clone() : Indirect<Type>);
    get!(borrow_type -> node_type.borrow() : Ref<Type>);

    get!(get_source -> source.clone() : Source);
}

impl_recur!{ CNode [] }

simple_fmt_display! {
    CNode : "{}", get_text()
}
