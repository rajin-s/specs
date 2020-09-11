use super::*;
use crate::language::types::Type;
use crate::language::{MemberScope, Visibility};

/* -------------------------------------------------------------------------- */
/*                             Function Definition                            */
/* -------------------------------------------------------------------------- */

#[derive(Debug)]
pub struct Function
{
    name:        String,
    arguments:   Vec<Argument>,
    return_type: Indirect<Type>,
    body:        OtherNode,
    node_type:   Indirect<Type>,
    source:      Source,
}
impl Function
{
    pub fn new(
        name: String,
        arguments: Vec<Argument>,
        return_type: Type,
        body: Node,
        source: Source,
    ) -> Self
    {
        let return_indirect = Indirect::new(return_type);
        let argument_types = arguments
            .iter()
            .map(|argument| argument.get_type())
            .collect();

        let function_type = FunctionType::from(argument_types, return_indirect.clone());

        return Self {
            name,
            arguments,
            return_type: return_indirect,
            body: OtherNode::new(body),
            node_type: Indirect::new(function_type.to_type()),
            source,
        };
    }

    get!(get_name     -> name : &String);
    get!(get_name_mut -> name : &mut String);

    get!(get_return_type -> return_type.clone() : Indirect<Type>);
    get!(get_arguments     -> arguments : &Vec<Argument>);
    get!(get_arguments_mut -> arguments : &mut Vec<Argument>);

    get!(get_type    -> node_type.clone() : Indirect<Type>);
    get!(borrow_type -> node_type.borrow() : Ref<Type>);
    set!(set_type    -> node_type : Indirect<Type>);

    get!(get_source -> source.clone() : Source);

    get_children! {
        get_body, get_body_mut -> body
    }
}

impl_recur! { Function [body] }

#[derive(Debug)]
pub struct Argument
{
    name:          String,
    argument_type: Indirect<Type>,
}
impl Argument
{
    get!(get_name -> name : &String);
    get!(get_type -> argument_type.clone() : Indirect<Type>);

    pub fn new(name: String, argument_type: Type) -> Self
    {
        return Self {
            name,
            argument_type: Indirect::new(argument_type),
        };
    }
}

/* -------------------------------------------------------------------------- */
/*                                    Type                                    */
/* -------------------------------------------------------------------------- */

#[derive(Debug)]
pub struct Class
{
    name:      String,
    node_type: Indirect<Type>,
    source:    Source,
}
impl Class
{
    get!(get_name     -> name : &String);
    get!(get_name_mut -> name : &mut String);

    get!(get_type    -> node_type.clone() : Indirect<Type>);
    get!(borrow_type -> node_type.borrow() : Ref<Type>);
    set!(set_type    -> node_type : Indirect<Type>);

    get!(get_source -> source.clone() : Source);
}

impl_recur! { Class [] }

#[derive(Debug)]
pub struct Member
{
    name:             String,
    scope:            MemberScope,
    read_visibility:  Visibility,
    write_visibility: Visibility,
    source:           Source,
}
impl Member {}

#[derive(Debug)]
pub struct Method
{
    function:   Function,
    scope:      MemberScope,
    visibility: Visibility,
    source:     Source,
}
impl Method {}

/*

    TODO: Finish implementing Class node, etc.

*/

/* -------------------------------------------------------------------------- */
/*                                   Display                                  */
/* -------------------------------------------------------------------------- */
simple_fmt_display! {
    Argument : "[{} {}]",
        name,
        argument_type
}

impl std::fmt::Display for Function
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result
    {
        let _ = write!(f, "(fn {} <", self.name);
        for (i, argument) in self.arguments.iter().enumerate()
        {
            match i
            {
                0 =>
                {
                    let _ = write!(f, "{}", argument);
                }
                _ =>
                {
                    let _ = write!(f, " {}", argument);
                }
            }
        }
        write!(f, "> -> {} {})", self.return_type, self.get_body())
    }
}

impl std::fmt::Display for Class
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result
    {
        write!(f, "[Class]")
    }
}
