use super::*;

/* -------------------------------------------------------------------------- */
/*                                  Sequences                                 */
/* -------------------------------------------------------------------------- */

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum SequenceMode
{
    Scope,
    Transparent,
}

#[derive(Debug)]
pub struct Sequence
{
    mode:      SequenceMode,
    nodes:     OtherNodes,
    node_type: Indirect<Type>,
}
impl Sequence
{
    pub fn new(mode: SequenceMode, nodes: Vec<Node>) -> Self
    {
        return Self {
            mode,
            nodes: nodes.into_iter().map(Indirect::new).collect(),
            node_type: basic_types::indirect::unknown(),
        };
    }

    get!(get_mode  -> mode : SequenceMode);
    get!(get_nodes -> nodes : &Vec<OtherNode>);

    get!(get_type    -> node_type.clone() : Indirect<Type>);
    get!(borrow_type -> node_type.borrow() : Ref<Type>);
    set!(set_type    -> node_type : Indirect<Type>);

    pub fn get_children(&self) -> Vec<OtherNode>
    {
        self.nodes.iter().map(Indirect::clone).collect()
    }
}
/* -------------------------------------------------------------------------- */
/*                                Conditionals                                */
/* -------------------------------------------------------------------------- */

#[derive(Debug)]
pub struct Conditional
{
    condition: OtherNode,
    then_node: OtherNode,
    else_node: OtherNode,

    node_type: Indirect<Type>,
}
impl Conditional
{
    pub fn new(condition: Node, then_node: Node, else_node: Node) -> Self
    {
        return Self {
            condition: OtherNode::new(condition),
            then_node: OtherNode::new(then_node),
            else_node: OtherNode::new(else_node),
            node_type: basic_types::indirect::unknown(),
        };
    }

    get_children! {
        get_condition,
        borrow_condition,
        borrow_condition_mut -> condition,

        get_then,
        borrow_then,
        borrow_then_mut -> then_node,

        get_else,
        borrow_else,
        borrow_else_mut -> else_node,
    }

    pub fn has_else(&self) -> bool
    {
        match *self.get_else().borrow()
        {
            Node::Nothing(_) => false,
            _ => true,
        }
    }

    get!(get_type    -> node_type.clone() : Indirect<Type>);
    get!(borrow_type -> node_type.borrow() : Ref<Type>);
    set!(set_type    -> node_type : Indirect<Type>);
}

/* -------------------------------------------------------------------------- */
/*                                   Display                                  */
/* -------------------------------------------------------------------------- */
impl std::fmt::Display for Sequence
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result
    {
        let (open, close) = match self.mode
        {
            SequenceMode::Scope => ("{", "}"),
            SequenceMode::Transparent => ("<<", ">>"),
        };

        let _ = write!(f, "{}", open);

        for (i, node) in self.nodes.iter().enumerate()
        {
            match i
            {
                0 =>
                {
                    let _ = write!(f, "{}", node);
                }
                _ =>
                {
                    let _ = write!(f, " {}", node);
                }
            }
        }

        write!(f, "{}", close)
    }
}
simple_fmt_display! {
    Conditional : "(if {} then {} else {})",
        get_condition(),
        get_then(),
        get_else(),
}
