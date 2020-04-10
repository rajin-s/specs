use crate::language::nodes::*;
use std::collections::HashMap;

type SymbolTable = HashMap<String, Type>;
type LocalSymbolTable = HashMap<String, Option<Type>>;

struct Params<'a>
{
    symbols:       &'a mut SymbolTable,
    local_symbols: &'a mut LocalSymbolTable,
}
impl<'a> Params<'a>
{
    pub fn new(symbols: &'a mut SymbolTable, local_symbols: &'a mut LocalSymbolTable) -> Self
    {
        return Self {
            symbols:       symbols,
            local_symbols: local_symbols,
        };
    }

    pub fn get_symbols(&self) -> &SymbolTable
    {
        return self.symbols;
    }
    pub fn get_local_symbols(&self) -> &LocalSymbolTable
    {
        return self.local_symbols;
    }

    pub fn get_symbols_mut(&mut self) -> &mut SymbolTable
    {
        return self.symbols;
    }
    pub fn get_local_symbols_mut(&mut self) -> &mut LocalSymbolTable
    {
        return self.local_symbols;
    }
    pub fn get_mut(&mut self) -> (&mut SymbolTable, &mut LocalSymbolTable)
    {
        return (self.symbols, self.local_symbols);
    }
}

pub fn apply(root: &mut Node)
{
    let mut symbols = SymbolTable::new();
    let mut top_level_symbols = LocalSymbolTable::new();

    let mut params = Params::new(&mut symbols, &mut top_level_symbols);
    infer_type(root, &mut params);
}

fn infer_type(node: &mut Node, params: &mut Params)
{
    match node
    {
        Node::Nothing | Node::Integer(_) =>
        {}
        Node::Variable(data) =>
        {
            let symbols = params.get_symbols();

            match symbols.get(data.get_name())
            {
                // Look up types for variables from the current symbol table
                Some(symbol_type) =>
                {
                    data.set_type(symbol_type.clone());
                }
                None =>
                {}
            }
        }

        Node::Call(data) =>
        {
            // Infer types of operator and operands first
            data.recur_transformation(infer_type, params);

            // Then infer the type of the result
            let operator_type = data.get_operator().get_type();
            if let DataType::Callable(operator_type_data) = operator_type.get_data_type()
            {
                let return_type = operator_type_data.get_return_type().clone();
                data.set_type(return_type);
            }
        }
        Node::PrimitiveOperator(data) => match data.get_operator()
        {
            // + : (int int) -> int
            PrimitiveOperator::Add =>
            {
                let integer_type = Type::new(DataType::Integer);
                let operator_type = CallableTypeData::new(
                    vec![integer_type.clone(), integer_type.clone()],
                    integer_type.clone(),
                )
                .to_type();

                data.set_type(operator_type);
            }
        },

        Node::Binding(data) =>
        {
            // Infer the type of the binding
            data.recur_transformation(infer_type, params);

            let name = data.get_name().clone();
            let binding_type = data.get_binding().get_type().clone();

            // Keep track of the binding type
            // note: the binding might change during compilation
            data.set_binding_type(binding_type.clone());

            // Add the binding type to the current symbol table
            let (symbols, local_symbols) = params.get_mut();

            let previous_entry = symbols.insert(name.clone(), binding_type);

            // Save the original type of the symbol if needed
            if !local_symbols.contains_key(&name)
            {
                local_symbols.insert(name.clone(), previous_entry);
            }
        }
        Node::Assignment(_) =>
        {
            node.recur_transformation(infer_type, params);
        }
        Node::Sequence(data) =>
        {
            if data.is_transparent()
            {
                // Transparent sequences don't introduce a new scope
                // note: transparent sequences are only used internally,
                //       so this doesn't matter for type-checking user programs
                data.recur_transformation(infer_type, params);
            }
            else
            {
                // Save original types if bindings shadow other bindings
                let symbols = params.get_symbols_mut();
                let mut new_local_symbols = LocalSymbolTable::new();

                let mut new_params: Params = Params::new(symbols, &mut new_local_symbols);

                // Infer the type of each child node
                data.recur_transformation(infer_type, &mut new_params);

                // Restore symbol types to what they were before this scope
                for (name, original_entry) in new_local_symbols
                {
                    if let Some(original_type) = original_entry
                    {
                        // The symbol was previously bound to something
                        symbols.insert(name, original_type);
                    }
                    else
                    {
                        // The symbol was not previously bound to anything
                        symbols.remove(&name);
                    }
                }
            }
        }
    }
}
