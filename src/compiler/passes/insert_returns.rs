use crate::language::nodes::*;
use std::mem::swap;

pub fn apply(root_node: &mut Node)
{
    let mut root_is_return_context = true;
    root_node.recur_transformation(convert_leaves, &mut root_is_return_context);
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
        Node::Sequence(_) =>
        {
            node.recur_transformation(convert_leaves, is_return_context);
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
        
        node =>
        {
            let mut is_return_context = false;
            node.recur_transformation(convert_leaves, &mut is_return_context);
        }
    }
}
