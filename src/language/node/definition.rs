use super::*;
use crate::language::types::Type;

/* -------------------------------------------------------------------------- */
/*                              Function Argument                             */
/* -------------------------------------------------------------------------- */

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
/*                             Function Definition                            */
/* -------------------------------------------------------------------------- */

#[derive(Debug)]
pub struct Function
{
    name:        String,
    arguments:   Vec<Argument>,
    return_type: Indirect<Type>,
    body:        OtherNode,

    node_type: Indirect<Type>,
}
impl Function
{
    pub fn new(name: String, arguments: Vec<Argument>, return_type: Type, body: Node) -> Self
    {
        return Self {
            name,
            arguments,
            return_type: Indirect::new(return_type),
            body: OtherNode::new(body),
            node_type: basic_types::indirect::unknown(),
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

    get_children! {
        get_body,
        borrow_body,
        borrow_body_mut -> body
    }
}

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
        write!(f, "> -> {} << {} >>", self.return_type, self.get_body())
    }
}
