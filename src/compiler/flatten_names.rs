use super::common::*;

///
/// ## FlattenNames Pass
///
/// - Makes all definition names unique (such that they can appear as top-level definitions in C)
///     - ex. nested `fn Foo { fn Bar }` to `fn Foo { fn Foo/Bar }`
///     - Generates unique names for anonymous sequence scopes
/// - Updates variable references to renamed definitions (respecting shadowing / scope binding rules)
/// - Leaves definition nodes in-place for `FlattenDefinitions` to extract
///
pub struct FlattenNames
{
    anonymous_scope_names: TempNameGenerator,
}

impl FlattenNames
{
    pub fn new() -> FlattenNames
    {
        FlattenNames {
            anonymous_scope_names: TempNameGenerator::new("scope"),
        }
    }
}

pub enum ParentScope
{
    None,
    Root,
    Function(String),
    Sequence(String),
}

impl ParentScope
{
    pub fn get_name(&self) -> &str
    {
        match self
        {
            ParentScope::None | ParentScope::Root => "",
            ParentScope::Function(name) | ParentScope::Sequence(name) => name.as_str(),
        }
    }

    pub fn get_child_name(&self, child_name: &String) -> String
    {
        match self
        {
            ParentScope::None | ParentScope::Root => child_name.clone(),
            ParentScope::Function(name) | ParentScope::Sequence(name) =>
            {
                format!("{}/{}", name, child_name)
            }
        }
    }
}

pub type State = (ParentScope, BindingState<Option<String>>);

impl RecurTransform<Node, State, Error> for FlattenNames
{
    fn get_root_state(&mut self, node: &Node) -> State
    {
        let mut binding_state = BindingState::root();

        if let Node::Sequence(sequence) = node
        {
            // Don't give new names to root-level functions

            binding_state.add_definitions_from_functions(sequence.get_nodes(), |_function| None);
        }

        (ParentScope::None, binding_state)
    }

    fn get_child_states(&mut self, state: &State, node: &Node) -> Vec<ChildState<State>>
    {
        let (parent_scope, binding_state) = state;

        match node
        {
            Node::Sequence(sequence) if !sequence.is_transparent() =>
            {
                let new_parent_scope = match parent_scope
                {
                    ParentScope::None => ParentScope::Root,
                    ParentScope::Function(function_name) =>
                    {
                        // Don't give new names to sequences that are function bodies

                        ParentScope::Sequence(function_name.clone())
                    }
                    ParentScope::Root | ParentScope::Sequence(_) =>
                    {
                        let anonymous_name = self.anonymous_scope_names.next();
                        ParentScope::Sequence(parent_scope.get_child_name(&anonymous_name))
                    }
                };

                // Get new names for definitions in this sequence

                let mut new_binding_state = BindingState::empty(binding_state, true, true);
                new_binding_state
                    .add_definitions_from_functions(sequence.get_nodes(), |function| {
                        Some(new_parent_scope.get_child_name(function.get_name()))
                    });

                let new_state = (new_parent_scope, new_binding_state);
                vec![ChildState::New(new_state)]
            }
            Node::Function(function) =>
            {
                // Get the new name of this function

                let scope_name = parent_scope.get_child_name(function.get_name());
                let new_parent_scope = ParentScope::Function(scope_name);

                // Allow function arguments to shadow definition names from the parent scope

                let mut new_binding_state = BindingState::empty(binding_state, true, false);
                new_binding_state
                    .add_bindings_from_arguments(function.get_arguments(), |_argument| None);

                let new_state = (new_parent_scope, new_binding_state);
                vec![ChildState::New(new_state)]
            }
            _ => vec![ChildState::Inherit],
        }
    }

    ///
    /// Rename definitions and variable references to definitions
    ///
    fn exit(&mut self, node: &mut Node, state: &mut State) -> ResultLog<(), Error>
    {
        let (scope, binding_state) = state;

        match node
        {
            Node::Variable(variable) =>
            {
                // Rename any variables if a binding exists

                match binding_state.lookup(variable.get_name())
                {
                    Some(Some(new_name)) => *variable.get_name_mut() = new_name.clone(),
                    _ => (),
                }
            }
            Node::Binding(binding) =>
            {
                // Shadow previous names so they are no longer changed after this variable is bound

                binding_state.add_binding(binding.get_name(), None);
            }
            Node::Function(function) =>
            {
                // Rename the function

                *function.get_name_mut() = scope.get_child_name(function.get_name());
            }

            _ => (),
        }

        ResultLog::Ok(())
    }
}
