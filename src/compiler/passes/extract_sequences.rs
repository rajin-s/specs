use super::utilities::TempNameGenerator;
use crate::language::nodes::*;
use std::mem::swap;

pub fn apply(root_node: &mut Node)
{
    let mut temp_names = TempNameGenerator::new("xseq");
    root_node.recur_transformation(extract_sequences, &mut temp_names);
}

// Helper functions
fn make_temp_variable(name: &String, node_type: &Type) -> Node
{
    return Node::from(VariableNodeData::new_typed(name.clone(), node_type.clone()));
}

fn extract_sequences(node: &mut Node, temp_names: &mut TempNameGenerator)
{
    // First make sure all child nodes are handled
    node.recur_transformation(extract_sequences, temp_names);

    match node
    {
        Node::Nothing
        | Node::Integer(_)
        | Node::Boolean(_)
        | Node::Variable(_)
        | Node::PrimitiveOperator(_) =>
        {}

        Node::Call(data) =>
        {
            /*
                Operand
                (F {... a} {... b})
                =>
                {
                    let temp1 = Nothing
                    {... (temp1 = a)}

                    let temp2 = Nothing
                    {... (temp2 = a)}

                    (F temp1 temp2)
                }

                Operator
                ({... F} a b c)
                =>
                {
                    let temp = Nothing
                    {... (temp = F)}

                    # handle any opereand bindings here

                    (temp a b c)
                }
            */

            let mut operand_temp_nodes: Vec<Node> = Vec::new();
            for operand in data.get_operands_mut().iter_mut()
            {
                if let Node::Sequence(_) = operand
                {
                    let temp_name = temp_names.next();
                    let operand_type = operand.get_type().clone();

                    // Extract the target node, replacing it with a temporary variable
                    let mut original_operand = make_temp_variable(&temp_name, &operand_type);
                    swap(&mut original_operand, operand);

                    // original_target -> (if ...)
                    // target_node     -> [var temp]

                    if let Node::Sequence(mut sequence_data) = original_operand
                    {
                        let mut temp = Node::Nothing;

                        // Convert final node into an assignment
                        swap(&mut temp, sequence_data.get_final_node_mut().unwrap());
                        temp = Node::from(AssignmentNodeData::new(
                            make_temp_variable(&temp_name, &operand_type),
                            temp,
                        ));

                        // The new RHS could be a sequence
                        extract_sequences(&mut temp, temp_names);

                        swap(&mut temp, sequence_data.get_final_node_mut().unwrap());

                        // let temp = Nothing
                        let bind_temp = Node::from(BindingNodeData::new_empty(
                            temp_name.clone(),
                            operand_type.clone(),
                        ));

                        // {... (temp = a)}
                        let assign_temp = Node::from(sequence_data);

                        operand_temp_nodes.push(bind_temp);
                        operand_temp_nodes.push(assign_temp);
                    }
                    else
                    {
                        // If we called lift_sequence, we know the target is a conditional
                        unreachable!();
                    }
                }
            }

            if let Node::Sequence(_) = data.get_operator()
            {
                fn get_operator(node: &mut Node) -> &mut Node
                {
                    if let Node::Call(data) = node
                    {
                        return data.get_operator_mut();
                    }

                    unreachable!();
                }

                // Get the temp binding and assignment nodes for the operator
                let mut operator_temp_nodes =
                    lift_sequence_no_overwrite(node, get_operator, temp_names);

                // Handle any operand bindings after the operator
                operator_temp_nodes.append(&mut operand_temp_nodes);

                // Make the enclosing group
                make_sequence(node, operator_temp_nodes);
            }
            else if operand_temp_nodes.len() > 0
            {
                // Make the enclosing sequence
                make_sequence(node, operand_temp_nodes);
            }
        }

        Node::Reference(data) =>
        {
            /* Reference
                (ref {... a})
                =>
                { ???
                    let temp = Nothing
                    {... (temp = a)}
                    (ref temp)
                }
            */

            if let Node::Sequence(_) = data.get_target()
            {
                fn get_target(node: &mut Node) -> &mut Node
                {
                    if let Node::Reference(data) = node
                    {
                        return data.get_target_mut();
                    }

                    unreachable!();
                }

                lift_sequence(node, get_target, temp_names);
            }
        }
        Node::Dereference(data) =>
        {
            /* Dereference
            (deref {... a})
            =>
            {
                let temp = Nothing
                {... temp = a}
                (deref temp)
            }
            */

            if let Node::Sequence(_) = data.get_target()
            {
                fn get_target(node: &mut Node) -> &mut Node
                {
                    if let Node::Dereference(data) = node
                    {
                        return data.get_target_mut();
                    }

                    unreachable!();
                }

                lift_sequence(node, get_target, temp_names);
            }
        }

        Node::Binding(data) =>
        {
            /* Binding
                (let x = {... a})
                =>
                /
                    let temp = Nothing
                    {... (temp = a)}
                    let x = temp
                \
            */
            if let Node::Sequence(_) = data.get_binding()
            {
                fn get_binding(node: &mut Node) -> &mut Node
                {
                    if let Node::Binding(data) = node
                    {
                        return data.get_binding_mut();
                    }

                    unreachable!();
                }

                lift_sequence(node, get_binding, temp_names);
            }
        }
        Node::Assignment(data) =>
        {
            /*
                RHS
                x = {... a}
                =>
                {
                    let temp = Nothing
                    {... (temp = a)}
                    x = temp
                }
            */

            if let Node::Sequence(_) = data.get_rhs()
            {
                fn get_rhs(node: &mut Node) -> &mut Node
                {
                    if let Node::Assignment(data) = node
                    {
                        return data.get_rhs_mut();
                    }

                    unreachable!();
                }

                lift_sequence(node, get_rhs, temp_names);
            }
        }

        Node::Sequence(data) =>
        {
            /* Sequence
                {
                    ...
                    {... a}
                }
                =>
                let temp = Nothing
                {
                    ...
                    {... (temp = a)}
                }
            */

            // Don't worry about void sequences
            if !data.get_type().is_void()
            {
                if let Some(Node::Sequence(_)) = data.get_final_node()
                {
                    fn get_final_node(node: &mut Node) -> &mut Node
                    {
                        if let Node::Sequence(data) = node
                        {
                            return data.get_final_node_mut().unwrap();
                        }
                        unreachable!();
                    }
                    lift_sequence(node, get_final_node, temp_names);
                }
            }
        }
        Node::Conditional(data) =>
        {
            /* Condition
                if {... a} then b else c
                =>
                let temp = Nothing
                {... (temp = a)}
                if temp then b else c
            */
            if let Node::Sequence(_) = data.get_condition()
            {
                unimplemented!();
            }
        }

        Node::Function(_data) =>
        {
            // Functions don't need to change
        }
    }
}

fn lift_sequence(
    original_node: &mut Node,
    target_node_accessor: fn(&mut Node) -> &mut Node,
    temp_names: &mut TempNameGenerator,
)
{
    // Get the target node from the original node
    // note: this is needed because we can't borrow both at the same time as function arguments
    let target_node = target_node_accessor(original_node);

    let temp_name = temp_names.next();
    let target_type = target_node.get_type().clone();

    // Extract the target node, replacing it with a temporary variable
    let mut original_target = make_temp_variable(&temp_name, &target_type);
    swap(&mut original_target, target_node);

    // original_target -> (if ...)
    // target_node     -> [var temp]

    if let Node::Sequence(mut sequence_data) = original_target
    {
        let mut temp = Node::Nothing;

        // Convert final node into an assignment
        swap(&mut temp, sequence_data.get_final_node_mut().unwrap());
        temp = Node::from(AssignmentNodeData::new(
            make_temp_variable(&temp_name, &target_type),
            temp,
        ));

        // The new RHS could be a sequence
        extract_sequences(&mut temp, temp_names);

        swap(&mut temp, sequence_data.get_final_node_mut().unwrap());

        // let temp = Nothing
        let bind_temp = Node::from(BindingNodeData::new_empty(
            temp_name.clone(),
            target_type.clone(),
        ));

        // {... (temp = a)}
        let assign_temp = Node::from(sequence_data);

        // Extract the original node, which now uses temp in place of a conditional
        let mut use_temp = Node::Nothing;
        swap(&mut use_temp, original_node);

        // Create the new enclosing sequence
        let mut outer_sequence = Node::from(SequenceNodeData::new_transparent(vec![
            bind_temp,
            assign_temp,
            use_temp,
        ]));

        // Replace the original node with the new sequence
        swap(&mut outer_sequence, original_node);
    }
    else
    {
        // If we called lift_sequence, we know the target is a conditional
        unreachable!();
    }
}

fn lift_sequence_no_overwrite(
    original_node: &mut Node,
    target_node_accessor: fn(&mut Node) -> &mut Node,
    temp_names: &mut TempNameGenerator,
) -> Vec<Node>
{
    // Get the target node from the original node
    // note: this is needed because we can't borrow both at the same time as function arguments
    let target_node = target_node_accessor(original_node);

    let temp_name = temp_names.next();
    let target_type = target_node.get_type().clone();

    // Extract the target node, replacing it with a temporary variable
    let mut original_target = make_temp_variable(&temp_name, &target_type);
    swap(&mut original_target, target_node);

    // original_target -> (if ...)
    // target_node     -> [var temp]

    if let Node::Sequence(mut sequence_data) = original_target
    {
        let mut temp = Node::Nothing;

        // Convert final node into an assignment
        swap(&mut temp, sequence_data.get_final_node_mut().unwrap());
        temp = Node::from(AssignmentNodeData::new(
            make_temp_variable(&temp_name, &target_type),
            temp,
        ));

        // The new RHS could be a sequence
        extract_sequences(&mut temp, temp_names);

        swap(&mut temp, sequence_data.get_final_node_mut().unwrap());

        // let temp = Nothing
        let bind_temp = Node::from(BindingNodeData::new_empty(
            temp_name.clone(),
            target_type.clone(),
        ));

        // {... (temp = a)}
        let assign_temp = Node::from(sequence_data);

        return vec![bind_temp, assign_temp];
    }
    else
    {
        // If we called lift_sequence, we know the target is a conditional
        unreachable!();
    }
}

fn make_sequence(original_node: &mut Node, mut before: Vec<Node>)
{
    // Extract the original node, which now uses temp in place of a conditional
    let mut use_temp = Node::Nothing;
    swap(&mut use_temp, original_node);

    // Create the new enclosing sequence
    before.push(use_temp);
    let mut outer_sequence = Node::from(SequenceNodeData::new_transparent(before));

    // Replace the original node with the new sequence
    swap(&mut outer_sequence, original_node);
}
