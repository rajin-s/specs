use crate::language::nodes::*;
use crate::language::symbols;

pub fn apply(node: &Node) -> String
{
    return format!(
        "#include \"specs_runtime.h\"\nint _USER_MAIN() \n{{\n{}\n}}",
        get_c(node, 1)
    );
}

fn get_c(node: &Node, indent: usize) -> String
{
    match node
    {
        Node::Nothing => format!("/* invalid nothing node */"),
        Node::Variable(data) =>
        {
            let c_name = symbols::convert_to_c_safe(data.get_name());
            c_name
        }
        Node::Integer(data) => format!("{}", data.get_value()),

        Node::Call(data) => match data.get_operator()
        {
            Node::PrimitiveOperator(operator_data) => match operator_data.get_operator()
            {
                PrimitiveOperator::Add =>
                {
                    let operands = data.get_operands();
                    let a_string = get_c(&operands[0], indent);
                    let b_string = get_c(&operands[1], indent);

                    format!("({} + {})", a_string, b_string)
                }
            },
            _ => format!("/* invalid operator node: {} */", node),
        },
        Node::PrimitiveOperator(data) => format!(
            "/* unhandled primitive operator {:?} */",
            data.get_operator()
        ),

        Node::Binding(data) =>
        {
            let type_string = get_c_type(data.get_binding_type());
            let c_name = symbols::convert_to_c_safe(data.get_name());

            if let Node::Nothing = data.get_binding()
            {
                format!("{} {}", type_string, c_name)
            }
            else
            {
                let binding_string = get_c(data.get_binding(), indent);
                format!("{} {} = {}", type_string, c_name, binding_string)
            }
        }
        Node::Assignment(data) =>
        {
            let lhs_string = get_c(data.get_lhs(), indent);
            let rhs_string = get_c(data.get_rhs(), indent);

            format!("{} = {}", lhs_string, rhs_string)
        }
        Node::Sequence(data) =>
        {
            let mut result = String::new();

            for node in data.get_nodes().iter()
            {
                let node_string = get_c(node, indent);
                result = format!("{}{};", result, node_string);
            }

            if data.is_transparent()
            {
                result
            }
            else
            {
                format!("{{ {} }}", result)
            }
        }
    }
}

fn get_c_type(original_type: &Type) -> String
{
    match original_type.get_data_type()
    {
        DataType::Integer => format!("int"),
        DataType::Void => format!("void"),
        _ => format!("???"),
    }
}
