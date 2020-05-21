use crate::language::nodes::*;
use std::mem::swap;

pub fn apply(root_node: &mut Node)
{
    let mut root_is_return_context = true;
    convert_leaves(root_node, &mut root_is_return_context);
}

fn convert_leaves(node: &mut Node, is_return_context: &mut bool)
{
    match node
    {
        Node::Integer(_)
        | Node::Boolean(_)
        | Node::Variable(_)
        | Node::Call(_)
        | Node::Reference(_)
        | Node::Dereference(_)
            if *is_return_context =>
        {
            // If we are in a returning context, replace returnable nodes with a return statement
            let mut temp = Node::Nothing;
            swap(&mut temp, node);

            temp = Node::from(CallNodeData::new(
                Node::from(VariableNodeData::new(String::from("return"))),
                vec![temp],
            ));

            swap(&mut temp, node);
        }

        Node::Call(_) =>
        {
            let mut is_return_context = false;
            node.recur_transformation(convert_leaves, &mut is_return_context);
        }

        Node::Reference(_) =>
        {
            let mut is_return_context = false;
            node.recur_transformation(convert_leaves, &mut is_return_context);
        }
        Node::Dereference(_) =>
        {
            let mut is_return_context = false;
            node.recur_transformation(convert_leaves, &mut is_return_context);
        }

        Node::Binding(_) =>
        {
            let mut is_return_context = false;
            node.recur_transformation(convert_leaves, &mut is_return_context);
        }
        Node::Assignment(_) =>
        {
            let mut is_return_context = false;
            node.recur_transformation(convert_leaves, &mut is_return_context);
        }
        Node::Sequence(data) =>
        {
            let mut body_nodes_return_context = false;

            if let Some(final_node_index) = data.get_final_node_index()
            {
                for (i, sequence_node) in data.get_nodes_mut().iter_mut().enumerate()
                {
                    if i == final_node_index
                    {
                        convert_leaves(sequence_node, is_return_context);
                    }
                    else
                    {
                        convert_leaves(sequence_node, &mut body_nodes_return_context);
                    }
                }
            }
            else
            {
                data.recur_transformation(convert_leaves, &mut body_nodes_return_context);
            }
        }
        Node::Conditional(data) =>
        {
            let mut condition_is_return_context = false;
            let mut is_return_context = *is_return_context;
            convert_leaves(data.get_condition_mut(), &mut condition_is_return_context);
            convert_leaves(data.get_then_mut(), &mut is_return_context);
            convert_leaves(data.get_else_mut(), &mut is_return_context);
        }
        Node::Function(data) =>
        {
            let mut body_is_return_context = true;
            convert_leaves(data.get_body_mut(), &mut body_is_return_context);
        }
        Node::Type(data) =>
        {
            for method in data.get_methods_mut().iter_mut()
            {
                let mut body_is_return_context = true;
                let function_data = method.get_function_data_mut();
                convert_leaves(function_data.get_body_mut(), &mut body_is_return_context);
            }
        }
        node =>
        {
            let mut is_return_context = false;
            node.recur_transformation(convert_leaves, &mut is_return_context);
        }
    }
}
