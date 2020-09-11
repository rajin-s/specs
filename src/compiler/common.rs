pub use crate::errors::compile_error::*;
pub use crate::language::node::*;
pub use crate::utilities::*;

use std::collections::HashMap;

///
/// Helper struct for generating unique temporary names
///
pub struct TempNameGenerator
{
    name:   String,
    number: usize,
}

impl TempNameGenerator
{
    ///
    /// Create a new name generator with the given prefix
    ///
    pub fn new(name: &str) -> Self
    {
        return Self {
            name:   name.to_owned(),
            number: 0,
        };
    }

    ///
    /// Get the next unique name from the generator
    ///
    pub fn next(&mut self) -> String
    {
        self.number += 1;
        format!("_{}_{}", self.name, self.number)
    }
}

///
/// Map of local binings, and potentially a reference to a parent map
/// 
/// We use an unsafe pointer because we can't thread lifetimes through generic recursive transformation handlers...
/// 
struct BindingMap<TBinding>
{
    pub local:  HashMap<String, TBinding>,
    pub parent: Option<*const Self>,
}

impl<TBinding> BindingMap<TBinding>
{
    pub fn new_root() -> Self
    {
        BindingMap {
            local:  HashMap::new(),
            parent: None,
        }
    }

    pub fn new_child(parent: &Self) -> Self
    {
        BindingMap {
            local:  HashMap::new(),
            parent: Some(parent as *const Self),
        }
    }

    pub fn get_parent(&self) -> Option<&Self>
    {
        match self.parent
        {
            Some(ptr) => Some(unsafe { &*ptr }),
            None => None,
        }
    }
}

///
/// A general-purpose recursive state component to map names to some sort of data
///
pub struct BindingState<TBinding>
{
    bindings:    BindingMap<TBinding>,
    definitions: BindingMap<TBinding>,
}

impl<TBinding: Clone> BindingState<TBinding>
{
    ///
    /// Create am empty BindingState with no parent
    ///
    pub fn root() -> BindingState<TBinding>
    {
        BindingState {
            bindings:    BindingMap::<TBinding>::new_root(),
            definitions: BindingMap::<TBinding>::new_root(),
        }
    }

    ///
    /// Create an empty binding state with a parent, specifying if bindings an/or definitions from
    ///     the parent should be visible in the new state as well
    ///
    pub fn empty(
        parent: &Self,
        inherit_definitions: bool,
        inherit_bindings: bool,
    ) -> BindingState<TBinding>
    {
        let bindings = match inherit_bindings
        {
            true => BindingMap::new_child(&parent.bindings),
            false => BindingMap::new_root(),
        };
        let definitions = match inherit_definitions
        {
            true => BindingMap::new_child(&parent.definitions),
            false => BindingMap::new_root(),
        };

        BindingState {
            bindings,
            definitions,
        }
    }

    ///
    /// Add definition bindings from `Function` nodes, using the given `Function` to `TBinding` map
    ///
    pub fn add_definitions_from_functions<TGetBinding>(
        &mut self,
        nodes: &Vec<Node>,
        get_binding: TGetBinding,
    ) where
        TGetBinding: Fn(&Function) -> TBinding,
    {
        self.definitions
            .local
            .extend(nodes.iter().filter_map(|node| match node
            {
                Node::Function(function) =>
                {
                    Some((function.get_name().clone(), get_binding(function)))
                }
                _ => None,
            }));
    }

    ///
    /// Add bindings from a function's arguments, using the given `Argument` to `TBinding` map
    ///
    pub fn add_bindings_from_arguments<TGetBinding>(
        &mut self,
        arguments: &Vec<Argument>,
        get_binding: TGetBinding,
    ) where
        TGetBinding: Fn(&Argument) -> TBinding,
    {
        self.bindings.local.extend(
            arguments
                .iter()
                .map(|argument| (argument.get_name().clone(), get_binding(argument))),
        );
    }

    ///
    /// Add a binding to the given name
    ///
    pub fn add_binding(&mut self, name: &String, binding: TBinding)
    {
        self.bindings.local.insert(name.clone(), binding);
    }

    ///
    /// Check for any bindings associated with the given name, checking local bindings, local
    ///     definitions, then potentially recurring to parent bindings, etc...
    ///
    pub fn lookup(&self, name: &String) -> Option<TBinding>
    {
        fn lookup_recursive<TBinding: Clone>(
            name: &String,
            bindings: Option<&BindingMap<TBinding>>,
            definitions: Option<&BindingMap<TBinding>>,
        ) -> Option<TBinding>
        {
            if bindings.is_none() && definitions.is_none()
            {
                // If we aren't checking anything, we aren't finding anything (and can stop recurision)

                return None;
            }

            // Check local bindings

            let parent_bindings = match bindings
            {
                Some(bindings) => match bindings.local.get(name)
                {
                    Some(binding) => return Some(binding.clone()),
                    None => bindings.get_parent(),
                },
                None => None,
            };

            // Check local definitions

            let parent_definitions = match definitions
            {
                Some(definitions) => match definitions.local.get(name)
                {
                    Some(binding) => return Some(binding.clone()),
                    None => definitions.get_parent(),
                },
                None => None,
            };

            // Potentially check parent bindings and definitions

            lookup_recursive(name, parent_bindings, parent_definitions)
        }

        lookup_recursive(name, Some(&self.bindings), Some(&self.definitions))
    }
}

///
/// ## Wrap Pass
///
/// - Wraps the result of a node tree with the given wrap function
///
pub struct WrapPass<TWrap>
where
    TWrap: Fn(Node) -> Node,
{
    wrap: TWrap,
}

impl<TWrap> WrapPass<TWrap>
where
    TWrap: Fn(Node) -> Node,
{
    pub fn new(wrap: TWrap) -> WrapPass<TWrap>
    {
        WrapPass { wrap }
    }
}

///
/// Pass state for WrapPass
///
/// - Specify if the current node is in a result context
///
pub struct WrapPassState
{
    pub is_result_context: bool,
}

impl WrapPassState
{
    pub fn new(is_result_context: bool) -> WrapPassState
    {
        WrapPassState { is_result_context }
    }
}

impl<TWrap> RecurTransform<Node, WrapPassState, Error> for WrapPass<TWrap>
where
    TWrap: Fn(Node) -> Node,
{
    fn get_root_state(&mut self, _node: &Node) -> WrapPassState
    {
        WrapPassState::new(true)
    }

    fn get_child_states(
        &mut self,
        _state: &WrapPassState,
        node: &Node,
    ) -> Vec<ChildState<WrapPassState>>
    {
        match node
        {
            Node::Sequence(sequence) =>
            {
                match sequence.get_result_index()
                {
                    Some(index) =>
                    {
                        // Only the result node is potentially in a result context (wow :O)

                        sequence
                            .get_nodes()
                            .iter()
                            .enumerate()
                            .map(|(i, _node)| {
                                if i == index
                                {
                                    ChildState::Inherit
                                }
                                else
                                {
                                    ChildState::New(WrapPassState::new(false))
                                }
                            })
                            .collect()
                    }
                    None =>
                    {
                        // No child is in a result context (void sequence)

                        vec![ChildState::New(WrapPassState::new(false))]
                    }
                }
            }
            Node::Conditional(_) =>
            {
                // The condition is never in a result context, branches inherit from the parent

                vec![
                    ChildState::New(WrapPassState::new(false)),
                    ChildState::Inherit,
                    ChildState::Inherit,
                ]
            }
            _ =>
            {
                // Children of other nodes will never be in a result context

                vec![ChildState::New(WrapPassState::new(false))]
            }
        }
    }

    fn exit(&mut self, node: &mut Node, state: &mut WrapPassState) -> ResultLog<(), Error>
    {
        if !node.is_complex() && state.is_result_context
        {
            // We only want to wrap non-complex nodes in result contexts (otherwise we'd want to
            //  recur into the node's children)

            *node = (self.wrap)(node.extract_temp());
        }

        ResultLog::Ok(())
    }
}
