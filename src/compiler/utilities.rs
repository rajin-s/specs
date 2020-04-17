use crate::language::nodes::*;

pub struct TempNameGenerator
{
    name:   String,
    number: usize,
}
impl TempNameGenerator
{
    pub fn new(name: &str) -> Self
    {
        return Self {
            name:   name.to_owned(),
            number: 0,
        };
    }

    pub fn next(&mut self) -> String
    {
        self.number += 1;
        format!("_{}_{}", self.name, self.number)
    }
}

pub fn print_types(node: &Node)
{
    let mut _params = ();

    println!("Types:");
    print_type(node, &mut _params);
    println!("");

    fn print_type(node: &Node, _params: &mut ())
    {
        println!("\t{:8} : {}", format!("{}", node.get_type()), node);
        node.recur_parse(print_type, _params);
    }

}
