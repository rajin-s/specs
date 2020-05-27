use crate::compiler::internal::*;

pub struct Pass {}
impl Pass
{
    pub fn new() -> Self
    {
        Self {}
    }
}
impl CompilerPass<State> for Pass
{
    // No transformation needed
    fn transform(
        &self,
        _node: &mut Node,
        _state: Indirect<State>,
        _messages: &mut PassMessageContext,
    )
    {
    }

    // Print the type of each node as we descent the AST
    fn get_state(
        &self,
        node: &Node,
        parent: Indirect<State>,
        _messages: &mut PassMessageContext,
    ) -> Indirect<State>
    {
        let depth = parent.borrow().depth;

        for _ in 0..depth
        {
            print!("\t\t");
        }

        println!("{} => {}", node.get_type(), node);

        // Child nodes are at depth+1
        Indirect::new(State::new(depth + 1))
    }

    fn get_name(&self) -> String
    {
        "PrintTypes".to_owned()
    }
}

// Track node depth for indentation
pub struct State
{
    pub depth: usize,
}
impl State
{
    pub fn new(depth: usize) -> Self
    {
        Self { depth }
    }
}
impl PassState for State
{
    fn empty() -> Self
    {
        Self { depth: 0 }
    }
}
