use super::errors::*;
use crate::language::nodes::*;

pub fn apply(root: &Node) -> TypeErrorList
{
    let mut errors = TypeErrorList::new();
    root.parse_recursive(check_type, &mut errors);

    return errors;
}

fn check_type(node: &Node, errors: &mut TypeErrorList)
{
    let mut new_errors: Vec<TypeError> = Vec::new();

    match node
    {
        Node::Variable(data) =>
        {
            // Ensure that all variables have types
            if data.get_type().is_unknown()
            {
                new_errors.push(TypeError::UnknownSymbolType(data.get_name().clone()));
            }
        }
        Node::Call(data) =>
        {
            // Ensure that all calls have
            //  - callable operator types
            //  - correct argument types
            let operator_type = data.get_operator().get_type();

            if let DataType::Callable(callable_data) = operator_type.get_data_type()
            {
                let operands = data.get_operands();
                let argument_types = callable_data.get_argument_types();

                // Make sure the number of arguments is correct
                if argument_types.len() == operands.len()
                {
                    // Make sure each argument type is correct
                    for (i, argument_type) in argument_types.iter().enumerate()
                    {
                        if argument_type != operands[i].get_type()
                        {
                            new_errors.push(TypeError::InvalidArgumentType(
                                i,
                                argument_type.clone(),
                                operands[i].get_type().clone(),
                            ));
                        }
                    }
                }
                else
                {
                    new_errors.push(TypeError::InvalidArgumentCount(
                        argument_types.len(),
                        operands.len(),
                    ));
                }
            }
            else
            {
                new_errors.push(TypeError::InvalidOperatorType(operator_type.clone()));
            }
        }
        Node::Binding(data) =>
        {
            // Ensure that all variable bindings have types
            if data.get_binding().get_type().is_unknown()
            {
                new_errors.push(TypeError::InvalidBindingType(
                    data.get_name().clone(),
                    data.get_binding().get_type().clone(),
                ));
            }
        }
        _ =>
        {}
    }

    errors.append(&mut new_errors);
}
