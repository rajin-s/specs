use crate::language::nodes::*;
use std::collections::VecDeque;

pub fn foo(root: &mut Node)
{
    let mut q = VecDeque::new();
    q.push_front(root);

    while !q.is_empty()
    {
        let n = q.pop_front();
    }
}

fn foo_transform(node: &mut Node)
{
    if let Node::Variable(data) = node
    {
        let new_name = format!("FOO_{}", data.get_name());
        data.set_name(new_name);
    }
}

fn get_children<'a>(node: &'a mut Node) -> Vec<&'a mut Node>
{
    match node
    {
        Node::Nothing
        | Node::Integer(_)
        | Node::Boolean(_)
        | Node::Variable(_)
        | Node::PrimitiveOperator(_) => Vec::new(),

        Node::Call(data) =>
        {
            let (operator, operands, _) = data.get_all_mut();
            let mut operands = operands.iter_mut().collect();

            let mut result = vec![operator];
            result.append(&mut operands);

            result
        }

        Node::Reference(data) =>
        {
            vec![data.get_target_mut()]
        }
        Node::Dereference(data) =>
        {
            vec![data.get_target_mut()]
        }
        
        Node::Binding(data) =>
        {
            vec![data.get_binding_mut()]
        }
        Node::Assignment(data) =>
        {
            vec![data.get_lhs_mut(), data.get_rhs_mut()]
        }
        Node::Sequence(data) =>
        {
            data.get_nodes_mut().iter_mut().collect()
        }
        Node::Conditional(data) =>
        {
            vec![data.get_condition_mut(), data.get_then_mut(), data.get_else_mut()]
        }
        Node::Function(data) =>
        {
            vec![data.get_body_mut()]
        }
        Node::Type(data) =>
        {
            data.get_methods_mut().iter_mut().map(|x| x.get_function_data_mut().get_body_mut()).collect()
        }
        Node::Access(data) =>
        {
            vec![data.get_target_mut()]
        }
    }
}