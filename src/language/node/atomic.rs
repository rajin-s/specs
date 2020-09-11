use super::primitive;
use super::*;

/* -------------------------------------------------------------------------- */
/*                                Data Literals                               */
/* -------------------------------------------------------------------------- */

#[derive(Debug)]
pub struct Nothing
{
    node_type: Indirect<Type>,
    source:    Source,
}
impl Nothing
{
    pub fn new(source: Source) -> Self
    {
        Self {
            node_type: basic_types::indirect::void(),
            source,
        }
    }

    pub fn new_typed(node_type: Indirect<Type>, source: Source) -> Self
    {
        Self { node_type, source }
    }

    get!(get_type -> node_type.clone() : Indirect<Type>);
    get!(borrow_type -> node_type.borrow() : Ref<Type>);

    get!(get_source -> source.clone() : Source);
}

impl_recur! { Nothing [] }

#[derive(Debug)]
pub struct Comment
{
    node_type: Indirect<Type>,
    content:   String,
    source:    Source,
}
impl Comment
{
    pub fn new(content: String, source: Source) -> Self
    {
        Self {
            node_type: basic_types::indirect::unknown(),
            content,
            source,
        }
    }

    get!(get_type -> node_type.clone() : Indirect<Type>);
    get!(borrow_type -> node_type.borrow() : Ref<Type>);

    get!(get_content -> content : &String);

    get!(get_source -> source.clone() : Source);
}

impl_recur! { Comment [] }

#[derive(Debug)]
pub struct Integer
{
    value:     i64,
    node_type: Indirect<Type>,
    source:    Source,
}
impl Integer
{
    pub fn new(value: i64, source: Source) -> Self
    {
        Self {
            value,
            node_type: basic_types::indirect::integer(),
            source,
        }
    }

    get!(get_value -> value : i64);
    get!(get_type -> node_type.clone() : Indirect<Type>);
    get!(borrow_type -> node_type.borrow() : Ref<Type>);

    get!(get_source -> source.clone() : Source);
}

impl_recur! { Integer [] }

#[derive(Debug)]
pub struct Boolean
{
    value:     bool,
    node_type: Indirect<Type>,
    source:    Source,
}
impl Boolean
{
    pub fn new(value: bool, source: Source) -> Self
    {
        Self {
            value,
            node_type: basic_types::indirect::boolean(),
            source,
        }
    }

    get!(get_value -> value : bool);
    get!(get_type -> node_type.clone() : Indirect<Type>);
    get!(borrow_type -> node_type.borrow() : Ref<Type>);

    get!(get_source -> source.clone() : Source);
}

impl_recur!{ Boolean [] }

/* -------------------------------------------------------------------------- */
/*                                  Variables                                 */
/* -------------------------------------------------------------------------- */

#[derive(Debug)]
pub struct Variable
{
    name:      String,
    node_type: Indirect<Type>,
    source:    Source,
}
impl Variable
{
    pub fn new(name: String, source: Source) -> Self
    {
        return Self {
            name,
            node_type: basic_types::indirect::unknown(),
            source,
        };
    }
    pub fn new_typed(name: String, node_type: Indirect<Type>, source: Source) -> Self
    {
        return Self {
            name,
            node_type,
            source,
        };
    }

    get!(get_name     -> name : &String);
    get!(get_name_mut -> name : &mut String);

    get!(get_type    -> node_type.clone() : Indirect<Type>);
    get!(borrow_type -> node_type.borrow() : Ref<Type>);
    set!(set_type    -> node_type : Indirect<Type>);

    get!(get_source -> source.clone() : Source);
}

impl_recur!{ Variable [] }

/* -------------------------------------------------------------------------- */
/*                             Primitive Operators                            */
/* -------------------------------------------------------------------------- */

#[derive(Debug)]
pub struct PrimitiveOperator
{
    operator:  primitive::Operator,
    node_type: Indirect<Type>,
    source:    Source,
}
impl PrimitiveOperator
{
    pub fn new(operator: primitive::Operator, source: Source) -> Self
    {
        return Self {
            operator,
            node_type: basic_types::indirect::unknown(),
            source,
        };
    }

    get!(get_value     -> operator : primitive::Operator);
    get!(get_value_mut -> operator : &mut primitive::Operator);

    get!(get_type    -> node_type.clone() : Indirect<Type>);
    get!(borrow_type -> node_type.borrow() : Ref<Type>);
    set!(set_type    -> node_type : Indirect<Type>);

    get!(get_source -> source.clone() : Source);
}

impl_recur!{ PrimitiveOperator [] }

/* -------------------------------------------------------------------------- */
/*                                   Display                                  */
/* -------------------------------------------------------------------------- */

simple_fmt_display! {
    Comment : "<<<{}>>>", content
}
simple_fmt_display! {
    Nothing : "[nothing]",
}
simple_fmt_display! {
    Integer : "[int {}]", value
}
simple_fmt_display! {
    Boolean : "[bool {}]", value
}
simple_fmt_display! {
    PrimitiveOperator : "[op {}]", operator
}
simple_fmt_display! {
    Variable : "[var {}]", name
}
