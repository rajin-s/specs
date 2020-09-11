use crate::compiler::binding_state::BindingState;
use crate::compiler::internal::*;
use crate::compiler::utilities::TempNameGenerator;

use std::collections::HashMap;

/* Pass: flatten::definition_names
    - Makes all definition names unique (such that they can appear as top-level definitions in C)
        - ex. nested fn Foo { fn Bar } => fn Foo { fn Foo/Bar }
    - Updates variable references to renamed definitions (respecting shadowing / scope binding rules)
    - Leaves definition nodes in-place for flatten::definitions to extract
*/

// Compiler pass instance
pub struct Pass
{
    anonymous_names: TempNameGenerator,
}
impl Pass
{
    pub fn new() -> Self
    {
        Self {
            anonymous_names: TempNameGenerator::new("anonymous_scope"),
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum StateContext
{
    None,
    Sequence,
    Function,
    FunctionBody,
    Class,
}

#[derive(Clone)]
pub struct StateData
{
    context:    StateContext,
    scope_name: String,
}

impl Default for StateData
{
    fn default() -> Self
    {
        Self {
            context:    StateContext::None,
            scope_name: String::new(),
        }
    }
}
impl StateData
{
    pub fn new(context: StateContext, scope_name: String) -> Self
    {
        Self {
            context,
            scope_name,
        }
    }
}

type State = BindingState<String, StateData>;

/* -------------------------------------------------------------------------- */
/*                                    Pass                                    */
/* -------------------------------------------------------------------------- */

impl CompilerPass<State> for Pass
{
    // Modify a node given some state
    //  - All child nodes have already been transformed at this point
    fn transform(
        &mut self,
        node: &mut Node,
        state: Indirect<State>,
        _messages: &mut PassMessageContext,
    )
    {
        match node
        {
            Node::Variable(variable) =>
            {
                // Rename variables referencing definitions
                if let Some(new_name) = state.borrow().get(variable.get_name())
                {
                    if !new_name.is_empty()
                    {
                        *variable.get_name_mut() = new_name;
                    }
                }
            }

            Node::Binding(binding) =>
            {
                // Potentially shadow definition names in the current scope
                state
                    .borrow_mut()
                    .add_binding(binding.get_name().clone(), String::new());
            }

            Node::Function(function) =>
            {
                // Rename the function
                if let Some(new_name) = state.borrow().get_definition(function.get_name())
                {
                    *function.get_name_mut() = new_name;
                }
            }

            Node::Class(class) =>
            {
                // TODO: Handle classes

                // Rename the class
                if let Some(new_name) = state.borrow().get_definition(class.get_name())
                {
                    *class.get_name_mut() = new_name;
                }
            }

            _ =>
            {}
        }
    }

    // Get the state to use for a node's children based on the current state
    //  - Child nodes have not yet been visited
    fn get_child_states(
        &mut self,
        node: &Node,
        parent: Indirect<State>,
        _messages: &mut PassMessageContext,
    ) -> Vec<Indirect<State>>
    {
        use definition::{Argument, Class, Function};

        let new_state = match node
        {
            Node::Function(function) =>
            {
                // Get the new name of the function scope
                let new_scope_name = {
                    let parent = parent.borrow();
                    let parent_data = parent.get_data();
                    let parent_scope_name = &parent_data.scope_name;

                    get_scoped_name(parent_scope_name, function.get_name())
                };

                // Argument bindings can shadow external definitions
                let argument_bindings =
                    State::get_bindings_from_function(function, |_arg: &Argument| String::new());

                // Create the new state and keep track of the current scope name
                let new_state = State::extend_with_bindings(
                    parent,
                    false,
                    argument_bindings,
                    StateData::new(StateContext::Function, new_scope_name),
                );

                Indirect::new(new_state)
            }
            Node::Sequence(sequence) =>
            {
                let (new_context, new_scope_name) = {
                    let parent = parent.borrow();
                    let parent_data = parent.get_data();
                    let parent_context = parent_data.context;
                    let parent_scope_name = &parent_data.scope_name;

                    match parent_context
                    {
                        StateContext::Function =>
                        {
                            // Don't add a new anonymous scope name if the sequence is the body of a function
                            (StateContext::FunctionBody, parent_scope_name.clone())
                        }
                        StateContext::None =>
                        {
                            // Don't add a new anonymous scope name if the sequence is the root node
                            (StateContext::Sequence, parent_scope_name.clone())
                        }
                        _ =>
                        {
                            // Create a new anonymous scope name
                            let anonymous_name = self.anonymous_names.next();
                            (StateContext::Sequence, get_scoped_name(&parent_scope_name, &anonymous_name))
                        }
                    }
                };

                // Get definition names inside the sequence
                let get_scoped_function_name =
                    |function: &Function| get_scoped_name(&new_scope_name, function.get_name());
                let get_scoped_class_name =
                    |class: &Class| get_scoped_name(&new_scope_name, class.get_name());

                let new_definitions = State::get_definitions_from_nodes(
                    sequence.get_nodes(),
                    get_scoped_function_name,
                    get_scoped_class_name,
                );

                // Create the new state and keep track of the current scope name
                let new_state = State::extend_with_definitions(
                    parent,
                    true,
                    new_definitions,
                    StateData::new(new_context, new_scope_name),
                );

                Indirect::new(new_state)
            }

            Node::Class(class) =>
            {
                // TODO: Handle classes

                // Get the new name of the function scope
                let new_scope_name = {
                    let parent = parent.borrow();
                    let parent_data = parent.get_data();
                    let parent_scope_name = &parent_data.scope_name;

                    get_scoped_name(parent_scope_name, class.get_name())
                };

                // Create the new state and keep track of the current scope name
                let new_state = State::extend_with_bindings(
                    parent,
                    false,
                    HashMap::new(),
                    StateData::new(StateContext::Class, new_scope_name),
                );

                Indirect::new(new_state)
            }

            _ => parent.clone(),
        };

        vec![new_state]
    }

    // Get the name of the pass (for debugging)
    fn get_name(&self) -> String
    {
        "FlattenDefinitionNames".to_owned()
    }
}

fn get_scoped_name(scope_name: &String, name: &String) -> String
{
    if scope_name.is_empty()
    {
        name.clone()
    }
    else
    {
        format!("{}/{}", scope_name, name)
    }
}
