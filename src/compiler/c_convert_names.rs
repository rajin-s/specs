use super::common::*;

///
/// ## C Convert Names Pass
///
/// - Converts all names to C-safe versions
///
pub struct ConvertNames {}

impl ConvertNames
{
    pub fn new() -> ConvertNames
    {
        ConvertNames {}
    }

    fn convert_name(&self, name: String) -> String
    {
        let mut result = String::new();

        for c in name.chars()
        {
            match c
            {
                '/' => result.push_str("__"),
                '-' => result.push_str("_"),
                c => result.push(c),
            }
        }

        result
    }
}

pub struct PassState {}

impl PassState
{
    pub fn new() -> PassState
    {
        PassState {}
    }
}

impl RecurTransform<Node, PassState, Error> for ConvertNames
{
    fn get_root_state(&mut self, _node: &Node) -> PassState
    {
        PassState::new()
    }

    fn exit(&mut self, node: &mut Node, _state: &mut PassState) -> ResultLog<(), Error>
    {
        match node
        {
            Node::Variable(variable) =>
            {
                let name = variable.get_name_mut();
                *name = self.convert_name(std::mem::take(name));
            }
            Node::Access(access) =>
            {
                let name = access.get_property_mut();
                *name = self.convert_name(std::mem::take(name));
            }
            Node::Binding(binding) =>
            {
                let name = binding.get_name_mut();
                *name = self.convert_name(std::mem::take(name));
            }
            Node::Function(function) =>
            {
                let name = function.get_name_mut();
                *name = self.convert_name(std::mem::take(name));
            }
            _ => (),
        }
        ResultLog::Ok(())
    }
}
