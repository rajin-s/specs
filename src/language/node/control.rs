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
    source:    Source,
}
impl Sequence
{
    pub fn new(mode: SequenceMode, nodes: Vec<Node>, source: Source) -> Self
    {
        return Self {
            mode,
            nodes,
            node_type: basic_types::indirect::unknown(),
            source,
        };
    }

    get!(get_mode -> mode : SequenceMode);
    set!(set_mode -> mode : SequenceMode);

    get!(get_nodes     -> nodes : &Vec<Node>);
    get!(get_nodes_mut -> nodes : &mut Vec<Node>);

    get!(get_type    -> node_type.clone() : Indirect<Type>);
    get!(borrow_type -> node_type.borrow() : Ref<Type>);
    set!(set_type    -> node_type : Indirect<Type>);

    pub fn is_transparent(&self) -> bool
    {
        self.mode == SequenceMode::Transparent
    }

    pub fn get_result_index(&self) -> Option<usize>
    {
        // The last returning node is the result of the sequence
        for (i, node) in self.nodes.iter().enumerate().rev()
        {
            match node
            {
                Node::Binding(_)
                | Node::Function(_)
                | Node::Class(_)
                | Node::Comment(_) =>
                {}
                _ =>
                {
                    return Some(i);
                }
            }
        }

        return None;
    }

    pub fn get_result_node(&self) -> Option<&Node>
    {
        match self.get_result_index()
        {
            Some(i) => Some(&self.get_nodes()[i]),
            None => None,
        }
    }
    pub fn get_result_node_mut(&mut self) -> Option<&mut Node>
    {
        match self.get_result_index()
        {
            Some(i) => Some(&mut self.get_nodes_mut()[i]),
            None => None,
        }
    }

    get!(get_source -> source.clone() : Source);
}

impl Recur<Node> for Sequence
{
    fn get_children(&self) -> Vec<&Node>
    {
        self.nodes.iter().collect()
    }
    fn get_children_mut(&mut self) -> Vec<&mut Node>
    {
        self.nodes.iter_mut().collect()
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
    source:    Source,
}
impl Conditional
{
    pub fn new(condition: Node, then_node: Node, else_node: Node, source: Source) -> Self
    {
        return Self {
            condition: OtherNode::new(condition),
            then_node: OtherNode::new(then_node),
            else_node: OtherNode::new(else_node),
            node_type: basic_types::indirect::unknown(),
            source,
        };
    }

    pub fn has_else(&self) -> bool
    {
        match self.get_else()
        {
            Node::Nothing(_) => false,
            _ => true,
        }
    }
    get_children! {
        get_condition, get_condition_mut -> condition,
        get_then,      get_then_mut      -> then_node,
        get_else,      get_else_mut      -> else_node,
    }

    get!(get_type    -> node_type.clone() : Indirect<Type>);
    get!(borrow_type -> node_type.borrow() : Ref<Type>);
    set!(set_type    -> node_type : Indirect<Type>);

    get!(get_source -> source.clone() : Source);
}

impl_recur! { Conditional [condition, then_node, else_node] }

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
            SequenceMode::Transparent => ("{!", "!}"),
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
