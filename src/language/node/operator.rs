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
    source:    Source,
}
impl Call
{
    pub fn new(operator: Node, operands: Vec<Node>, source: Source) -> Self
    {
        return Self {
            operator: OtherNode::new(operator),
            operands,
            node_type: basic_types::indirect::unknown(),
            source,
        };
    }

    get!(get_operator     -> operator.as_ref() : &Node);
    get!(get_operator_mut -> operator.as_mut() : &mut Node);

    get!(get_operands     -> operands : &Vec<Node>);
    get!(get_operands_mut -> operands : &mut Vec<Node>);

    get!(get_type    -> node_type.clone() : Indirect<Type>);
    get!(borrow_type -> node_type.borrow() : Ref<Type>);
    set!(set_type    -> node_type : Indirect<Type>);

    pub fn get_all_mut(&mut self) -> (&mut Node, &mut Vec<Node>)
    {
        (self.operator.as_mut(), &mut self.operands)
    }

    get!(get_source -> source.clone() : Source);
}

impl Recur<Node> for Call
{
    fn get_children(&self) -> Vec<&Node>
    {
        let mut result = vec![self.operator.as_ref()];
        result.append(&mut self.operands.iter().collect());
    
        result
    }
    fn get_children_mut(&mut self) -> Vec<&mut Node>
    {
        let mut result = vec![self.operator.as_mut()];
        result.append(&mut self.operands.iter_mut().collect());
    
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
    source:    Source,
}
impl Reference
{
    pub fn new(mode: ReferenceMode, target: Node, source: Source) -> Self
    {
        return Self {
            mode,
            target: OtherNode::new(target),
            node_type: basic_types::indirect::unknown(),
            source,
        };
    }

    get!(get_mode -> mode : ReferenceMode);

    get!(get_type    -> node_type.clone() : Indirect<Type>);
    get!(borrow_type -> node_type.borrow() : Ref<Type>);
    set!(set_type    -> node_type : Indirect<Type>);

    get!(get_source -> source.clone() : Source);

    get_children! {
        get_target, get_target_mut -> target
    }
}

impl_recur!{ Reference [target] }

/* -------------------------------------------------------------------------- */
/*                                 Dereference                                */
/* -------------------------------------------------------------------------- */

#[derive(Debug)]
pub struct Dereference
{
    target:    OtherNode,
    node_type: Indirect<Type>,
    source:    Source,
}
impl Dereference
{
    pub fn new(target: Node, source: Source) -> Self
    {
        return Self {
            target: OtherNode::new(target),
            node_type: basic_types::indirect::unknown(),
            source,
        };
    }

    get!(get_type    -> node_type.clone() : Indirect<Type>);
    get!(borrow_type -> node_type.borrow() : Ref<Type>);
    set!(set_type    -> node_type : Indirect<Type>);

    get!(get_source -> source.clone() : Source);

    get_children! {
        get_target, get_target_mut -> target
    }
}

impl_recur!{ Dereference [target] }

/* -------------------------------------------------------------------------- */
/*                                   Assign                                   */
/* -------------------------------------------------------------------------- */

#[derive(Debug)]
pub struct Assign
{
    lhs:       OtherNode,
    rhs:       OtherNode,
    node_type: Indirect<Type>,
    source:    Source,
}
impl Assign
{
    pub fn new(lhs: Node, rhs: Node, source: Source) -> Self
    {
        return Self {
            lhs: OtherNode::new(lhs),
            rhs: OtherNode::new(rhs),
            node_type: basic_types::indirect::void(),
            source,
        };
    }

    get!(get_type    -> node_type.clone() : Indirect<Type>);
    get!(borrow_type -> node_type.borrow() : Ref<Type>);

    get!(get_source -> source.clone() : Source);

    get_children! {
        get_lhs, get_lhs_mut -> lhs,
        get_rhs, get_rhs_mut -> rhs,
    }
}

impl_recur!{ Assign [lhs, rhs] }

/* -------------------------------------------------------------------------- */
/*                                   Access                                   */
/* -------------------------------------------------------------------------- */

#[derive(Debug)]
pub struct Access
{
    target:        OtherNode,
    property_name: String,
    node_type:     Indirect<Type>,
    source:        Source,
}
impl Access
{
    pub fn new(target: Node, property_name: String, source: Source) -> Self
    {
        return Self {
            target: OtherNode::new(target),
            property_name,
            node_type: basic_types::indirect::unknown(),
            source,
        };
    }

    get!(get_property     -> property_name : &String);
    get!(get_property_mut -> property_name : &mut String);

    get!(get_type    -> node_type.clone() : Indirect<Type>);
    get!(borrow_type -> node_type.borrow() : Ref<Type>);
    set!(set_type    -> node_type : Indirect<Type>);

    get!(get_source -> source.clone() : Source);

    get_children! {
        get_target, get_target_mut -> target
    }
}

impl_recur!{ Access [target] }

/* -------------------------------------------------------------------------- */
/*                                   Display                                  */
/* -------------------------------------------------------------------------- */

impl std::fmt::Display for Call
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result
    {
        let _ = write!(f, "({} ~", self.get_operator());
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
    Assign : "({} <- {})",
        get_lhs(),
        get_rhs(),
}
simple_fmt_display! {
    Access : "({} . {})",
        get_property(),
        get_target(),
}
