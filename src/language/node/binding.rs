use super::*;

#[derive(Debug)]
pub struct Binding
{
    name:    String,
    binding: OtherNode,
    node_type: Indirect<Type>,
}
impl Binding
{
    pub fn new(name: String, binding: Node) -> Self
    {
        return Self {
            name,
            binding: OtherNode::new(binding),
            node_type: basic_types::indirect::void(),
        };
    }

    get!(get_name     -> name : &String);
    get!(get_name_mut -> name : &mut String);

    get!(get_type    -> node_type.clone() : Indirect<Type>);
    get!(borrow_type -> node_type.borrow() : Ref<Type>);

    get_children! {
        get_binding,
        borrow_binding,
        borrow_binding_mut -> binding
    }
}

/* -------------------------------------------------------------------------- */
/*                                   Display                                  */
/* -------------------------------------------------------------------------- */

simple_fmt_display! {
    Binding : "(let {} = {})",
        get_name(),
        get_binding(),
}
