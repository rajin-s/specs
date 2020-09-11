use crate::compiler::internal::*;

/* Pass: c_output::convert_names
    - Converts all names to C-safe versions
*/

// Compiler pass instance
pub struct Pass {}
impl Pass
{
    pub fn new() -> Self
    {
        Self {}
    }
}

// Pass state
//  - Generated when descending the AST
//  - Potentially modified while ascending the AST (in execution order)
pub struct State {}
impl PassState for State
{
    fn empty() -> Self
    {
        return State {};
    }
}

/* -------------------------------------------------------------------------- */
/*                                    Pass                                    */
/* -------------------------------------------------------------------------- */

fn sanitize(name: String) -> String
{
    let mut result = String::new();
    result.reserve_exact(name.len());

    for c in name.chars()
    {
        match c
        {
            '/' =>
            {
                result.push_str("__");
            }
            '-' =>
            {
                result.push_str("_");
            }

            c => result.push(c),
        }
    }

    return result;
}

impl CompilerPass<State> for Pass
{
    // Modify a node given some state
    //  - All child nodes have already been transformed at this point
    fn transform(
        &mut self,
        node: &mut Node,
        _state: Indirect<State>,
        _messages: &mut PassMessageContext,
    )
    {
        fn replace_name(name: &mut String)
        {
            let mut temp = String::new();
            std::mem::swap(&mut temp, name);
            temp = sanitize(temp);
            std::mem::swap(&mut temp, name);
        }

        match node
        {
            Node::Variable(variable) =>
            {
                replace_name(variable.get_name_mut());
            }
            Node::Access(access) =>
            {
                replace_name(access.get_property_mut());
            }
            Node::Binding(binding) =>
            {
                replace_name(binding.get_name_mut());
            }
            Node::Function(function) =>
            {
                replace_name(function.get_name_mut());
            }
            Node::Class(class) =>
            {
                replace_name(class.get_name_mut());
            }

            _ =>
            {}
        }
    }

    // Get the state to use for a node's children based on the current state
    //  - Child nodes have not yet been visited
    fn get_child_states(
        &mut self,
        _node: &Node,
        parent: Indirect<State>,
        _messages: &mut PassMessageContext,
    ) -> Vec<Indirect<State>>
    {
        vec![parent.clone()]
    }

    // Get the name of the pass (for debugging)
    fn get_name(&self) -> String
    {
        "ConvertNames".to_owned()
    }
}

/* -------------------------------------------------------------------------- */
/*                                    State                                   */
/* -------------------------------------------------------------------------- */

impl State {}
