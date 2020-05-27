use super::*;
use crate::language::ReferenceMode;

/* -------------------------------------------------------------------------- */
/*                                    Call                                    */
/* -------------------------------------------------------------------------- */

#[derive(Debug)]
pub struct Call
{
    operator:  OtherNode,
    operands:  OtherNodes,
    node_type: Indirect<Type>,
}
impl Call
{
    pub fn new(operator: Node, operands: Vec<Node>) -> Self
    {
        return Self {
            operator:  OtherNode::new(operator),
            operands:  operands.into_iter().map(Indirect::new).collect(),
            node_type: basic_types::indirect::unknown(),
        };
    }

    get!(get_operator -> operator.clone() : OtherNode);
    get!(get_operands -> operands : &Vec<OtherNode>);

    get!(get_type    -> node_type.clone() : Indirect<Type>);
    get!(borrow_type -> node_type.borrow() : Ref<Type>);
    set!(set_type    -> node_type : Indirect<Type>);

    pub fn get_children(&self) -> Vec<OtherNode>
    {
        let mut result = vec![self.operator.clone()];
        result.append(&mut self.operands.iter().map(Indirect::clone).collect());

        result
    }
}

/* -------------------------------------------------------------------------- */
/*                                  Reference                                 */
/* -------------------------------------------------------------------------- */

#[derive(Debug)]
pub struct Reference
{
    mode:      ReferenceMode,
    target:    OtherNode,
    node_type: Indirect<Type>,
}
impl Reference
{
    pub fn new(mode: ReferenceMode, target: Node) -> Self
    {
        return Self {
            mode,
            target: OtherNode::new(target),
            node_type: basic_types::indirect::unknown(),
        };
    }

    get!(get_mode -> mode : ReferenceMode);

    get!(get_type    -> node_type.clone() : Indirect<Type>);
    get!(borrow_type -> node_type.borrow() : Ref<Type>);
    set!(set_type    -> node_type : Indirect<Type>);

    get_children! {
        get_target,
        borrow_target,
        borrow_target_mut -> target
    }
}

/* -------------------------------------------------------------------------- */
/*                                 Dereference                                */
/* -------------------------------------------------------------------------- */

#[derive(Debug)]
pub struct Dereference
{
    target:    OtherNode,
    node_type: Indirect<Type>,
}
impl Dereference
{
    pub fn new(target: Node) -> Self
    {
        return Self {
            target:    OtherNode::new(target),
            node_type: basic_types::indirect::unknown(),
        };
    }

    get!(get_type    -> node_type.clone() : Indirect<Type>);
    get!(borrow_type -> node_type.borrow() : Ref<Type>);
    set!(set_type    -> node_type : Indirect<Type>);

    get_children! {
        get_target,
        borrow_target,
        borrow_target_mut -> target
    }
}

/* -------------------------------------------------------------------------- */
/*                                   Assign                                   */
/* -------------------------------------------------------------------------- */

#[derive(Debug)]
pub struct Assign
{
    lhs: OtherNode,
    rhs: OtherNode,
    node_type: Indirect<Type>,
}
impl Assign
{
    pub fn new(lhs: Node, rhs: Node) -> Self
    {
        return Self {
            lhs: OtherNode::new(lhs),
            rhs: OtherNode::new(rhs),
            node_type: basic_types::indirect::void(),
        };
    }

    get!(get_type    -> node_type.clone() : Indirect<Type>);
    get!(borrow_type -> node_type.borrow() : Ref<Type>);

    get_children! {
        get_lhs,
        borrow_lhs,
        borrow_lhs_mut -> lhs,

        get_rhs,
        borrow_rhs,
        borrow_rhs_mut -> rhs,
    }
}

/* -------------------------------------------------------------------------- */
/*                                   Access                                   */
/* -------------------------------------------------------------------------- */

#[derive(Debug)]
pub struct Access
{
    target:        OtherNode,
    property_name: String,
    node_type:     Indirect<Type>,
}
impl Access
{
    pub fn new(target: Node, property_name: String) -> Self
    {
        return Self {
            target: OtherNode::new(target),
            property_name,
            node_type: basic_types::indirect::unknown(),
        };
    }

    get!(get_property     -> property_name : &String);
    get!(get_property_mut -> property_name : &mut String);

    get!(get_type    -> node_type.clone() : Indirect<Type>);
    get!(borrow_type -> node_type.borrow() : Ref<Type>);
    set!(set_type    -> node_type : Indirect<Type>);

    get_children! {
        get_target,
        borrow_target,
        borrow_target_mut -> target
    }
}

/* -------------------------------------------------------------------------- */
/*                                   Display                                  */
/* -------------------------------------------------------------------------- */

impl std::fmt::Display for Call
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result
    {
        let _ = write!(f, "({}:", self.get_operator());
        for operand in &self.operands
        {
            let _ = write!(f, " {}", operand);
        }
        write!(f, ")")
    }
}

simple_fmt_display! {
    Reference : "({} {})",
        get_mode(),
        get_target(),
}
simple_fmt_display! {
    Dereference : "(deref {})",
        get_target()
}
simple_fmt_display! {
    Assign : "({} := {})",
        get_lhs(),
        get_rhs(),
}
simple_fmt_display! {
    Access : "({} . {})",
        get_property(),
        get_target(),
}
