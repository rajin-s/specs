use crate::language::nodes::*;
use crate::language::runtime;

mod format_node;

pub fn get_c_string(root: &Node) -> String
{
    let mut result = String::new();

    result.push_str("#include \"specs_runtime.h\"");

    fn write_header(result: &mut String, header: &str)
    {
        result.push_str("\n\n/* ");
        result.push_str(header);
        result.push_str(" */\n");
    }
    fn write(result: &mut String, content: String)
    {
        *result = format!("{}{}\n", result, content);
    }

    let type_declarations = format_node::get_types(root, true);
    if !type_declarations.is_empty()
    {
        write_header(&mut result, "Type Declarations");
        write(&mut result, type_declarations);
    }

    let type_definitions = format_node::get_types(root, false);
    if !type_definitions.is_empty()
    {
        write_header(&mut result, "Type Definitions");
        write(&mut result, type_definitions);
    }

    let function_declarations = format_node::get_functions(root, true);
    if !function_declarations.is_empty()
    {
        write_header(&mut result, "Function Declarations");
        write(&mut result, function_declarations);
    }
    
    let function_definitions = format_node::get_functions(root, false);
    if !function_definitions.is_empty()
    {
        write_header(&mut result, "Function Definitions");
        write(&mut result, function_definitions);
    }
    
    write_header(&mut result, "Program Body");

    write(&mut result, format!("int {}()", runtime::names::MAIN_FUNCTION));
    write(&mut result, format_node::get_program_body(root));

    return result;
}