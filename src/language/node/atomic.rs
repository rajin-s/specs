use super::primitive;
use super::*;

/* -------------------------------------------------------------------------- */
/*                                Data Literals                               */
/* -------------------------------------------------------------------------- */

#[derive(Debug)]
pub struct Nothing
{
    node_type: Indirect<Type>,
}
impl Nothing
{
    pub fn new() -> Self
    {
        Self {
            node_type: basic_types::indirect::void(),
        }
    }

    get!(get_type -> node_type.clone() : Indirect<Type>);
    get!(borrow_type -> node_type.borrow() : Ref<Type>);

    get_children! {}
}

#[derive(Debug)]
pub struct Integer
{
    value:     i64,
    node_type: Indirect<Type>,
}
impl Integer
{
    pub fn new(value: i64) -> Self
    {
        Self {
            value,
            node_type: basic_types::indirect::integer(),
        }
    }

    get!(get_value -> value : i64);
    get!(get_type -> node_type.clone() : Indirect<Type>);
    get!(borrow_type -> node_type.borrow() : Ref<Type>);

    get_children! {}
}

#[derive(Debug)]
pub struct Boolean
{
    value:     bool,
    node_type: Indirect<Type>,
}
impl Boolean
{
    pub fn new(value: bool) -> Self
    {
        Self {
            value,
            node_type: basic_types::indirect::boolean(),
        }
    }

    get!(get_value -> value : bool);
    get!(get_type -> node_type.clone() : Indirect<Type>);
    get!(borrow_type -> node_type.borrow() : Ref<Type>);

    get_children! {}
}

/* -------------------------------------------------------------------------- */
/*                                  Variables                                 */
/* -------------------------------------------------------------------------- */

#[derive(Debug)]
pub struct Variable
{
    name:      String,
    node_type: Indirect<Type>,
}
impl Variable
{
    pub fn new(name: String) -> Self
    {
        return Self {
            name,
            node_type: basic_types::indirect::unknown(),
        };
    }
    pub fn new_typed(name: String, node_type: Type) -> Self
    {
        return Self {
            name,
            node_type: Indirect::new(node_type),
        };
    }

    get!(get_name     -> name : &String);
    get!(get_name_mut -> name : &mut String);

    get!(get_type    -> node_type.clone() : Indirect<Type>);
    get!(borrow_type -> node_type.borrow() : Ref<Type>);
    set!(set_type    -> node_type : Indirect<Type>);

    get_children! {}
}

/* -------------------------------------------------------------------------- */
/*                             Primitive Operators                            */
/* -------------------------------------------------------------------------- */

#[derive(Debug)]
pub struct PrimitiveOperator
{
    operator:  primitive::Operator,
    node_type: Indirect<Type>,
}
impl PrimitiveOperator
{
    pub fn new(operator: primitive::Operator) -> Self
    {
        return Self {
            operator,
            node_type: basic_types::indirect::unknown(),
        };
    }

    get!(get_operator     -> operator : primitive::Operator);
    get!(get_operator_mut -> operator : &mut primitive::Operator);

    get!(get_type    -> node_type.clone() : Indirect<Type>);
    get!(borrow_type -> node_type.borrow() : Ref<Type>);
    set!(set_type    -> node_type : Indirect<Type>);

    get_children! {}
}

/* -------------------------------------------------------------------------- */
/*                                   Display                                  */
/* -------------------------------------------------------------------------- */

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
