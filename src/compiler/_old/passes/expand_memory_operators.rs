use crate::language::nodes::*;
use crate::language::runtime;
use std::mem::swap;

pub fn apply(root_node: &mut Node)
{
    let mut params = ();

    root_node.recur_transformation(expand_create_operator, &mut params);
}

// Convert create operators to runtime function calls
fn expand_create_operator(node: &mut Node, _params: &mut ())
{
    match node
    {
        Node::Call(call_data) =>
        {
            if let Node::PrimitiveOperator(operator_data) = call_data.get_operator()
            {
                if operator_data.get_operator() == primitive::Operator::Create
                {
                    // Operator : create => _specs_allocate
                    {
                        let mut allocate_operator = Node::from(VariableNodeData::new(
                            String::from(runtime::names::ALLOCATE_FUNCTION),
                        ));
                        swap(&mut allocate_operator, call_data.get_operator_mut());
                    }

                    // Operand : T => sizeof(T)
                    let operand = &mut call_data.get_operands_mut()[0];
                    {
                        let size_operator = Node::from(VariableNodeData::new(String::from(
                            runtime::names::SIZE_OPERATOR,
                        )));

                        let mut temp = Node::Nothing;
                        swap(&mut temp, operand);
                        temp = Node::from(CallNodeData::new(size_operator, vec![temp]));
                        swap(&mut temp, operand);
                    }
                }
            }
        }
        _ =>
        {}
    }

    node.recur_transformation(expand_create_operator, _params);
}
