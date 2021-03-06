use crate::compiler::internal::*;

use crate::language::types::function::FunctionType;
use crate::language::types::ToType;

/* Pass: type_system::build_definition_types
    - Creates types for definition nodes based on parsed information
*/

pub struct Pass {}
impl Pass
{
    pub fn new() -> Self
    {
        Self {}
    }
}
impl CompilerPass<PassStateEmpty> for Pass
{
    // Build types for definitions
    fn transform(
        &mut self,
        node: &mut Node,
        _state: Indirect<PassStateEmpty>,
        _messages: &mut PassMessageContext,
    )
    {
        match node
        {
            Node::Function(function) =>
            {
                // Create a new function type from the definitions arguments and return type
                let function_type = {
                    let arguments = function
                        .get_arguments()
                        .iter()
                        .map(|arg| arg.get_type())
                        .collect();
                    FunctionType::from(arguments, function.get_return_type()).to_type()
                };
                function.set_type(Indirect::new(function_type));
            }
            _ =>
            {}
        }
    }

    // Pass doesn't need any state
    fn get_child_states(
        &mut self,
        _node: &Node,
        parent: Indirect<PassStateEmpty>,
        _messages: &mut PassMessageContext,
    ) -> Vec<Indirect<PassStateEmpty>>
    {
        vec![parent.clone()]
    }

    fn get_name(&self) -> String
    {
        "BuildDefinitionTypes".to_owned()
    }
}
