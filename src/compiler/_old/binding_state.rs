use crate::compiler::internal::*;
use std::collections::HashMap;

// A utility state component representing a map of binding/definition names to some values (types, new names, etc.)
pub struct BindingState<TBinding: Clone, TData: Default>
{
    definitions: Indirect<HashMap<String, TBinding>>,
    bindings:    Indirect<HashMap<String, TBinding>>,

    data: TData,

    inherit_bindings: bool,

    parent: Option<Indirect<Self>>,
}

impl<TBinding: Clone, TData: Default> PassState for BindingState<TBinding, TData>
{
    fn empty() -> Self
    {
        Self {
            definitions: Indirect::new(HashMap::new()),
            bindings:    Indirect::new(HashMap::new()),

            data: Default::default(),

            inherit_bindings: false,
            parent:           None,
        }
    }
}

impl<TBinding: Clone, TData: Default> BindingState<TBinding, TData>
{
    // Create a new state as a child of the current state, potentially including/excluding bindings
    //  - Immediately introduce local definitions
    pub fn extend_with_definitions(
        parent: Indirect<Self>,
        inherit_bindings: bool,
        new_definitions: HashMap<String, TBinding>,
        new_data: TData,
    ) -> Self
    {
        Self {
            definitions: Indirect::new(new_definitions),
            bindings: Indirect::new(HashMap::new()),
            parent: Some(parent),
            inherit_bindings,
            data: new_data,
        }
    }

    // Create a new state as a child of the current state, potentially including/excluding bindings
    //  - Immediately introduce local bindings
    pub fn extend_with_bindings(
        parent: Indirect<Self>,
        inherit_bindings: bool,
        new_bindings: HashMap<String, TBinding>,
        new_data: TData,
    ) -> Self
    {
        Self {
            definitions: Indirect::new(HashMap::new()),
            bindings: Indirect::new(new_bindings),
            parent: Some(parent),
            inherit_bindings,
            data: new_data,
        }
    }

    // Add a new bindings
    pub fn add_binding(&mut self, name: String, value: TBinding) -> bool
    {
        match self.bindings.borrow_mut().insert(name, value)
        {
            Some(_) => true,
            _ => false,
        }
    }

    // Add a single definitions
    pub fn add_definition(&mut self, name: String, value: TBinding) -> bool
    {
        match self.definitions.borrow_mut().insert(name, value)
        {
            Some(_) => true,
            _ => false,
        }
    }

    // Add multiple definitions
    pub fn add_definitions(&mut self, definitions: HashMap<String, TBinding>)
    {
        for (name, value) in definitions.into_iter()
        {
            self.definitions.borrow_mut().insert(name, value);
        }
    }

    // Internal type lookup function that respects binding inheritance properties
    fn lookup(&self, name: &String, check_bindings: bool) -> Option<TBinding>
    {
        // Check local bindings first
        if check_bindings
        {
            if let Some(value) = self.bindings.borrow().get(name)
            {
                return Some(value.clone());
            }
        }
        // Check local definitions second
        if let Some(value) = self.definitions.borrow().get(name)
        {
            return Some(value.clone());
        }
        // Check parent scope last with a (hopefully) tail-recursive call
        //  note: propagates check_bindings=false
        if let Some(parent) = &self.parent
        {
            return parent
                .borrow()
                .lookup(name, check_bindings && self.inherit_bindings);
        }
        return None;
    }

    // Get a symbol from the current scope, or look it up in the parent scope
    pub fn get(&self, name: &String) -> Option<TBinding>
    {
        self.lookup(name, true)
    }

    // Get a symbol from the current scope's definitions, or look it up in the parent scope
    pub fn get_definition(&self, name: &String) -> Option<TBinding>
    {
        self.lookup(name, false)
    }

    // Get a reference to the state's data
    pub fn get_data(&self) -> &TData
    {
        return &self.data;
    }
    pub fn get_data_mut(&mut self) -> &mut TData
    {
        return &mut self.data;
    }

    // Get a map of bindings from a function's arguments
    pub fn get_bindings_from_function<TMap>(
        function: &node::definition::Function,
        map: TMap,
    ) -> HashMap<String, TBinding>
    where
        TMap: Fn(&node::definition::Argument) -> TBinding,
    {
        function
            .get_arguments()
            .iter()
            .map(|arg| (arg.get_name().clone(), map(arg)))
            .collect()
    }

    // Get a map of definitions from a sequence of nodes
    pub fn get_definitions_from_nodes<TMapFunction, TMapClass>(
        nodes: &Vec<Node>,
        map_function: TMapFunction,
        map_class: TMapClass,
    ) -> HashMap<String, TBinding>
    where
        TMapFunction: Fn(&node::definition::Function) -> TBinding,
        TMapClass: Fn(&node::definition::Class) -> TBinding,
    {
        let mut map = HashMap::new();

        for node in nodes
        {
            match node
            {
                Node::Function(function) =>
                {
                    map.insert(function.get_name().clone(), map_function(function));
                }
                Node::Class(class) =>
                {
                    map.insert(class.get_name().clone(), map_class(class));
                }
                _ =>
                {}
            }
        }

        map
    }
}
