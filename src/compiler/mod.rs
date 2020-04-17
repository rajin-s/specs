use crate::language::nodes::*;

mod type_system;
mod checks;
mod passes;
mod output;

mod utilities;

#[derive(Clone, Copy, Debug)]
pub struct CompilerOptions
{
    pub show_debug_output: bool,
}
impl CompilerOptions
{
    pub fn new() -> Self
    {
        return Self {
            show_debug_output: false,
        };
    }
}

pub struct Compiler
{
    options:   CompilerOptions,
    root_node: Node,
}
impl Compiler
{
    pub fn compile(&mut self) -> Option<String>
    {
        use checks::*;
        use passes::*;

        // Do checks

        if let Some(errors) = validate_nodes::apply(&self.root_node)
        {
            for error in errors.iter()
            {
                println!("Structural error: {}", error);
            }
            return None;
        }

        // Build type information
        type_system::infer_types::apply(&mut self.root_node);

        if self.options.show_debug_output
        {
            utilities::print_types(&self.root_node);
        }

        // Check types
        if let Some(errors) = type_system::check_types::apply(&self.root_node)
        {
            for error in errors.iter()
            {
                println!("Type error: {}", error);
            }

            return None;
        }
        // Do passes

        
        remove_single_sequences::apply(&mut self.root_node);
        extract_conditionals::apply(&mut self.root_node);
        extract_sequences::apply(&mut self.root_node);
        
        insert_returns::apply(&mut self.root_node);
        make_definition_names_unique::apply(&mut self.root_node);

        // Generate output
        let output = output::get_c_string(&self.root_node);
        return Some(output);
    }

    pub fn new(options: CompilerOptions, node: Node) -> Self
    {
        return Self {
            options:   options,
            root_node: node,
        };
    }
}
