use crate::language::nodes::*;

mod checks;
mod output;
mod passes;
mod type_system;

mod utilities;

mod internal;

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
        fn do_pass(pass: fn(&mut Node), target: &mut Compiler, name: &str)
        {
            pass(&mut target.root_node);
            if target.options.show_debug_output
            {
                println!("{}\n{}\n", name, &target.root_node);
            }
        }

        do_pass(remove_single_sequences::apply, self, "Remove Single Sequences");
        do_pass(extract_conditionals::apply, self, "Extract Conditionals");
        do_pass(extract_sequences::apply, self, "Extract Sequences");
        do_pass(insert_returns::apply, self, "Insert Returns");
        do_pass(convert_instance_methods::apply, self, "Convert Instance Methods");
        do_pass(make_definition_names_unique::apply, self, "Make Definition Names Unique");

        do_pass(expand_memory_operators::apply, self, "Expand Memory Operators");

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
