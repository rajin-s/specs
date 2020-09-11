use super::*;

#[derive(Debug)]
pub struct Binding
{
    name:      String,
    binding:   OtherNode,
    node_type: Indirect<Type>,
    source:    Source,
}
impl Binding
{
    pub fn new(name: String, binding: Node, source: Source) -> Self
    {
        return Self {
            name,
            binding: OtherNode::new(binding),
            node_type: basic_types::indirect::void(),
            source,
        };
    }

    get!(get_name     -> name : &String);
    get!(get_name_mut -> name : &mut String);

    get!(get_type    -> node_type.clone() : Indirect<Type>);
    get!(borrow_type -> node_type.borrow() : Ref<Type>);

    get!(get_source -> source.clone() : Source);

    get_children! {
        get_binding, get_binding_mut -> binding
    }
}

impl_recur!{ Binding [binding] }

/* -------------------------------------------------------------------------- */
/*                                   Display                                  */
/* -------------------------------------------------------------------------- */

impl std::fmt::Display for Binding
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result
    {
        write!(
            f,
            "(let [{} {}] = {})",
            self.get_name(),
            self.get_binding().borrow_type(),
            self.get_binding()
        )
    }
}
