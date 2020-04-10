mod simplify_bindings;
mod write_c;
mod utilities;

use crate::language::nodes::*;

pub struct Compiler
{
    root_node: Node,
}
impl Compiler
{
    pub fn compile_c(&mut self) -> String
    {
        simplify_bindings::apply(&mut self.root_node);

        return write_c::apply(&self.root_node);
    }

    pub fn new(node: Node) -> Self
    {
        return Self { root_node: node };
    }
}
