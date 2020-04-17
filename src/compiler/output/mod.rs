use crate::language::nodes::*;

mod format_node;

pub fn get_c_string(root: &Node) -> String
{
    let mut result = String::new();

    let (declarations, definitions, program) = format_node::apply(root);

    result.push_str("#include \"specs_runtime.h\"");
    result.push('\n');

    result.push('\n');
    result.push_str("// Function Declarations");
    result.push('\n');
    {
        result = format!("{}{}", result, declarations);
    }
    result.push('\n');
    result.push_str("// Function Definitions");
    result.push('\n');
    {
        result = format!("{}{}", result, definitions);
    }
    result.push('\n');
    result.push_str("// User Program");
    result.push('\n');
    result.push_str("int _USER_MAIN()\n");
    {
        result = format!("{}{}", result, program);
    }

    return result;
}