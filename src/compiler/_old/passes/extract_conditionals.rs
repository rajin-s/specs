use super::utilities::TempNameGenerator;
use crate::language::nodes::*;
use std::mem::swap;

pub fn apply(root_node: &mut Node)
{
    let mut temp_names = TempNameGenerator::new("xcond");
    root_node.recur_transformation(extract_conditionals, &mut temp_names);
}

// Helper functions
fn make_temp_variable(name: &String, node_type: &Type) -> Node
{
    return Node::from(VariableNodeData::new_typed(name.clone(), node_type.clone()));
}

fn extract_conditionals(node: &mut Node, temp_names: &mut TempNameGenerator)
{
    // First make sure all child nodes are handled
    node.recur_transformation(extract_conditionals, temp_names);

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
                Operator
                ((if X then Y else Z) a b c)
                =>
                {
                    let temp1 = a
                    let temp2 = b
                    let temp3 = c

                    let temp4 = Nothing
                    (if X
                        then
                        temp4 = (Y temp1 temp2 temp3)
                        else
                        temp[4 = (Z temp1 temp2 temp3)
                    )
                    temp4
                }

                Operand
                (F (if X then Y else Z) (if A then B else C))
                =>
                {
                    let temp1 = Nothing
                    (if X then (temp1 = Y) else (temp1 = Z))
                    let temp2 = Nothing
                    (if A then (temp2 = B) else (temp2 = C))
                    (F temp1 temp2)
                }
            */

            if let Node::Conditional(_) = data.get_operator()
            {
                // Keep track of the result type
                let result_type = data.get_type().clone();

                let mut original_node = Node::Nothing;
                swap(&mut original_node, node);

                let mut original_operands: Vec<Node> = Vec::new();
                let mut operand_bindings: Vec<Node> = Vec::new();

                let mut original_condition = Node::Nothing;
                let mut original_operator_then = Node::Nothing;
                let mut original_operator_else = Node::Nothing;

                if let Node::Call(mut call_data) = original_node
                {
                    // Extract the original operands
                    swap(&mut original_operands, call_data.get_operands_mut());

                    for operand in original_operands.iter_mut()
                    {
                        let temp_name = temp_names.next();
                        let operand_type = operand.get_type().clone();

                        // Extract the original operand, replacing it with a temporary variable
                        let mut original_operand = make_temp_variable(&temp_name, &operand_type);
                        swap(&mut original_operand, operand);

                        // Bind the temporary variable to the original operand
                        let mut bind_temp =
                            Node::from(BindingNodeData::new(temp_name, original_operand));
                        // note: the original operand could've been a conditional
                        extract_conditionals(&mut bind_temp, temp_names);

                        operand_bindings.push(bind_temp);
                    }

                    if let Node::Conditional(conditional_data) = call_data.get_operator_mut()
                    {
                        // Extract the operator branches
                        swap(
                            &mut original_condition,
                            conditional_data.get_condition_mut(),
                        );
                        swap(&mut original_operator_then, conditional_data.get_then_mut());
                        swap(&mut original_operator_else, conditional_data.get_else_mut());

                        let condition_temp_name = temp_names.next();
                        let condition_type = original_condition.get_type().clone();

                        let result_temp_name = temp_names.next();

                        // Construct the new calls (note that all operands are now temporary variables)
                        let then_call = Node::from(AssignmentNodeData::new(
                            make_temp_variable(&result_temp_name, &result_type),
                            Node::from(CallNodeData::new(
                                original_operator_then,
                                original_operands.clone(),
                            )),
                        ));
                        let else_call = Node::from(AssignmentNodeData::new(
                            make_temp_variable(&result_temp_name, &result_type),
                            Node::from(CallNodeData::new(
                                original_operator_else,
                                original_operands,
                            )),
                        ));

                        // Create the new conditional with the simplified calls and add it after the bindings
                        let new_conditional = Node::from(ConditionalNodeData::new(
                            make_temp_variable(&condition_temp_name, &condition_type),
                            then_call,
                            else_call,
                        ));

                        // Bind the condition so it is executed first
                        let bind_condition = Node::from(BindingNodeData::new(
                            condition_temp_name,
                            original_condition,
                        ));

                        let bind_result_temp = Node::from(BindingNodeData::new_empty(
                            result_temp_name.clone(),
                            result_type.clone(),
                        ));
                        let return_result_temp =
                            Node::from(VariableNodeData::new_typed(result_temp_name, result_type));

                        // let condition = ...
                        // let arg = ...
                        // let temp = Nothing
                        // (if condition then (temp = (A arg)) else (temp = (B arg)))
                        // temp
                        let mut outer_sequence_nodes = vec![bind_condition];
                        outer_sequence_nodes.append(&mut operand_bindings);
                        outer_sequence_nodes.push(bind_result_temp);
                        outer_sequence_nodes.push(new_conditional);
                        outer_sequence_nodes.push(return_result_temp);

                        // Create the new outer sequence with operand bindings and the new conditional
                        let mut outer_sequence =
                            Node::from(SequenceNodeData::new(outer_sequence_nodes));

                        // Put the outer sequence in place of the original node
                        swap(&mut outer_sequence, node);
                    }
                    else
                    {
                        // We know the operator was a conditional
                        unreachable!();
                    }
                }
                else
                {
                    // We know the node was a call
                    unreachable!();
                }
            }
            else
            {
                let mut operand_temp_nodes: Vec<Node> = Vec::new();

                for operand in data.get_operands_mut().iter_mut()
                {
                    if let Node::Conditional(_) = operand
                    {
                        let temp_name = temp_names.next();
                        let operand_type = operand.get_type().clone();

                        // Extract the operand, replacing with a temporary variable
                        let mut original_operand = make_temp_variable(&temp_name, &operand_type);
                        let new_operand = operand;

                        swap(&mut original_operand, new_operand);

                        // original_operand -> (if ...)
                        // new_operand -> [var temp]

                        // Turn the conditional branches into assignments
                        if let Node::Conditional(mut conditional_data) = original_operand
                        {
                            let mut temp = Node::Nothing;

                            // Convert then into an assignment
                            {
                                swap(&mut temp, conditional_data.get_then_mut());
                                temp = Node::from(AssignmentNodeData::new(
                                    make_temp_variable(&temp_name, &operand_type),
                                    temp,
                                ));
                                // The RHS could be a conditional
                                extract_conditionals(&mut temp, temp_names);

                                swap(&mut temp, conditional_data.get_then_mut());
                            }

                            // Convert else into an assignment
                            {
                                swap(&mut temp, conditional_data.get_else_mut());
                                temp = Node::from(AssignmentNodeData::new(
                                    make_temp_variable(&temp_name, &operand_type),
                                    temp,
                                ));
                                // The RHS could be a conditional
                                extract_conditionals(&mut temp, temp_names);

                                swap(&mut temp, conditional_data.get_else_mut());
                            }

                            // Create the initial binding
                            let bind_temp = Node::from(BindingNodeData::new_empty(
                                temp_name.clone(),
                                operand_type.clone(),
                            ));

                            // Bind the temp variable, then assign it in a conditional
                            operand_temp_nodes.push(bind_temp);
                            operand_temp_nodes.push(Node::from(conditional_data));
                        }
                        else
                        {
                            // We know the operand was a conditional
                            unreachable!();
                        }
                    }
                }
                if operand_temp_nodes.len() > 0
                {
                    // Extract the original call node
                    let mut temp = Node::Nothing;
                    swap(&mut temp, node);

                    // Add the call node to the end of the operand bindings and make a sequence
                    operand_temp_nodes.push(temp);
                    let mut sequence_node = Node::from(SequenceNodeData::new(operand_temp_nodes));

                    // Replace the original call node with the new sequence
                    swap(&mut sequence_node, node);
                }
            }
        }

        Node::Reference(data) =>
        {
            /* Reference
                (ref (if A then B else C))
                =>
                {
                    let temp = Nothing
                    (if A then (temp = B) else (temp = C))
                    (ref temp)
                }
            */

            if let Node::Conditional(_) = data.get_target()
            {
                fn get_target(node: &mut Node) -> &mut Node
                {
                    if let Node::Reference(data) = node
                    {
                        return data.get_target_mut();
                    }

                    unreachable!();
                }

                lift_conditional(node, get_target, temp_names);
            }
        }
        Node::Dereference(data) =>
        {
            /* Dereference
                (deref (if A then B else C))
                =>
                {
                    let temp = Nothing
                    (if A then (temp = B) else (temp = C))
                    (deref temp)
                }
            */

            if let Node::Conditional(_) = data.get_target()
            {
                fn get_target(node: &mut Node) -> &mut Node
                {
                    if let Node::Dereference(data) = node
                    {
                        return data.get_target_mut();
                    }

                    unreachable!();
                }

                lift_conditional(node, get_target, temp_names);
            }
        }

        Node::Binding(data) =>
        {
            /* Binding
                (let x = (if A then B else C))
                =>
                /
                    let temp = Nothing
                    (if A then (temp = B) else (temp = C))
                    let x = temp
                \
            */

            if let Node::Conditional(_) = data.get_binding()
            {
                fn get_binding(node: &mut Node) -> &mut Node
                {
                    if let Node::Binding(data) = node
                    {
                        return data.get_binding_mut();
                    }

                    unreachable!();
                }

                lift_conditional(node, get_binding, temp_names);
            }
        }
        Node::Assignment(data) =>
        {
            /*
                LHS + RHS
                (if A then B else C) = (if X then Y else Z)
                =>
                {
                    (let temp = Nothing)
                    (if A
                        then (temp = (ref B))
                        else (temp = (ref C))
                    )
                    (if X
                        then ((deref temp) = Y)
                        else ((deref temp) = Z)
                    )
                }

                LHS
                (if A then B else C) = x
                =>
                {
                    (let temp = Nothing)
                    (if A
                        then (temp = (ref A))
                        else (temp = (ref B))
                    )
                    (deref temp) = x
                }

                RHS
                x = (if A then B else C)
                =>
                {
                    (let temp = (ref x))
                    (if A
                        then ((deref temp) = B)
                        else ((deref temp) = C)
                    )
                }
            */

            match (data.get_lhs(), data.get_rhs())
            {
                (Node::Conditional(_), Node::Conditional(_)) =>
                {
                    let temp_name = temp_names.next();
                    let temp_type = data.get_lhs().get_type().make_reference(ReferenceMode::Mutable);

                    // Extract the original LHS and RHS
                    let mut original_lhs = Node::Nothing;
                    let mut original_rhs = Node::Nothing;
                    swap(&mut original_lhs, data.get_lhs_mut());
                    swap(&mut original_rhs, data.get_rhs_mut());

                    if let Node::Conditional(mut lhs_conditional_data) = original_lhs
                    {
                        let mut temp = Node::Nothing;

                        // Convert LHS then branch into an assignment
                        {
                            swap(&mut temp, lhs_conditional_data.get_then_mut());
                            temp = Node::from(AssignmentNodeData::new(
                                make_temp_variable(&temp_name, &temp_type),
                                Node::from(ReferenceNodeData::new_infer_type(
                                    temp,
                                    ReferenceMode::Mutable,
                                )),
                            ));

                            // The new RHS could be a conditional
                            extract_conditionals(&mut temp, temp_names);

                            swap(&mut temp, lhs_conditional_data.get_then_mut());
                        }
                        // Convert LHS else branch into an assignment
                        {
                            swap(&mut temp, lhs_conditional_data.get_else_mut());
                            temp = Node::from(AssignmentNodeData::new(
                                make_temp_variable(&temp_name, &temp_type),
                                Node::from(ReferenceNodeData::new_infer_type(
                                    temp,
                                    ReferenceMode::Mutable,
                                )),
                            ));

                            // The new RHS could be a conditional
                            extract_conditionals(&mut temp, temp_names);

                            swap(&mut temp, lhs_conditional_data.get_else_mut());
                        }

                        if let Node::Conditional(mut rhs_conditional_data) = original_rhs
                        {
                            // Convert RHS then branch into an assignment
                            {
                                swap(&mut temp, rhs_conditional_data.get_then_mut());
                                temp = Node::from(AssignmentNodeData::new(
                                    Node::from(DereferenceNodeData::new(make_temp_variable(
                                        &temp_name, &temp_type,
                                    ))),
                                    temp,
                                ));

                                // The new RHS could be a conditional
                                extract_conditionals(&mut temp, temp_names);

                                swap(&mut temp, rhs_conditional_data.get_then_mut());
                            }
                            // Convert RHS else branch into an assignment
                            {
                                swap(&mut temp, rhs_conditional_data.get_else_mut());
                                temp = Node::from(AssignmentNodeData::new(
                                    Node::from(DereferenceNodeData::new(make_temp_variable(
                                        &temp_name, &temp_type,
                                    ))),
                                    temp,
                                ));

                                // The new RHS could be a conditional
                                extract_conditionals(&mut temp, temp_names);

                                swap(&mut temp, rhs_conditional_data.get_else_mut());
                            }

                            // let temp = Nothing
                            let bind_temp =
                                Node::from(BindingNodeData::new_empty(temp_name, temp_type));

                            // (if A then (temp = (ref B)) else (temp = (ref C)))
                            let assign_temp_ref = Node::from(lhs_conditional_data);

                            // (if X then ((deref temp) = Y) else ((deref temp) = Z))
                            let assign_temp = Node::from(rhs_conditional_data);

                            // Create the new enclosing sequence
                            let mut outer_sequence = Node::from(SequenceNodeData::new(vec![
                                bind_temp,
                                assign_temp_ref,
                                assign_temp,
                            ]));

                            // Replace the original node with the new sequence
                            swap(&mut outer_sequence, node);
                        }
                        else
                        {
                            // We know the RHS is a conditional
                            unreachable!();
                        }
                    }
                    else
                    {
                        // We know the LHS is a conditional
                        unreachable!();
                    }
                }
                (Node::Conditional(_), _) =>
                {
                    let temp_name = temp_names.next();
                    let temp_type = data.get_lhs().get_type().make_reference(ReferenceMode::Mutable);

                    // Extract the original LHS, replacing with (deref temp)
                    let mut original_lhs = Node::from(DereferenceNodeData::new(
                        make_temp_variable(&temp_name, &temp_type),
                    ));
                    swap(&mut original_lhs, data.get_lhs_mut());

                    if let Node::Conditional(mut conditional_data) = original_lhs
                    {
                        let mut temp = Node::Nothing;

                        // Convert then branch into an assignment
                        {
                            swap(&mut temp, conditional_data.get_then_mut());
                            temp = Node::from(AssignmentNodeData::new(
                                make_temp_variable(&temp_name, &temp_type),
                                Node::from(ReferenceNodeData::new_infer_type(
                                    temp,
                                    ReferenceMode::Mutable,
                                )),
                            ));

                            // The new RHS could be a conditional
                            extract_conditionals(&mut temp, temp_names);

                            swap(&mut temp, conditional_data.get_then_mut());
                        }
                        // Convert else branch into an assignment
                        {
                            swap(&mut temp, conditional_data.get_else_mut());
                            temp = Node::from(AssignmentNodeData::new(
                                make_temp_variable(&temp_name, &temp_type),
                                Node::from(ReferenceNodeData::new_infer_type(
                                    temp,
                                    ReferenceMode::Mutable,
                                )),
                            ));

                            // The new RHS could be a conditional
                            extract_conditionals(&mut temp, temp_names);

                            swap(&mut temp, conditional_data.get_else_mut());
                        }

                        // let temp = Nothing
                        let bind_temp =
                            Node::from(BindingNodeData::new_empty(temp_name, temp_type));

                        // (if ... then (temp = A) else (temp = B))
                        let assign_temp = Node::from(conditional_data);

                        // Extract the original node, which now uses (deref temp) in place of a conditional
                        let mut use_temp = Node::Nothing;
                        swap(&mut use_temp, node);

                        // Create the new enclosing sequence
                        let mut outer_sequence = Node::from(SequenceNodeData::new(vec![
                            bind_temp,
                            assign_temp,
                            use_temp,
                        ]));

                        // Replace the original node with the new sequence
                        swap(&mut outer_sequence, node);
                    }
                    else
                    {
                        // We know the LHS is a conditional
                        unreachable!();
                    }
                }
                (_, Node::Conditional(_)) =>
                {
                    let temp_name = temp_names.next();
                    let temp_type = data.get_lhs().get_type().make_reference(ReferenceMode::Mutable);

                    // Extract the original RHS
                    let mut original_rhs = Node::Nothing;
                    swap(&mut original_rhs, data.get_rhs_mut());

                    if let Node::Conditional(mut conditional_data) = original_rhs
                    {
                        let mut temp = Node::Nothing;

                        // Convert then branch into an assignment
                        {
                            swap(&mut temp, conditional_data.get_then_mut());
                            temp = Node::from(AssignmentNodeData::new(
                                Node::from(DereferenceNodeData::new(make_temp_variable(
                                    &temp_name, &temp_type,
                                ))),
                                temp,
                            ));

                            // The LHS could be a conditional
                            extract_conditionals(&mut temp, temp_names);

                            swap(&mut temp, conditional_data.get_then_mut());
                        }
                        // Convert else branch into an assignment
                        {
                            swap(&mut temp, conditional_data.get_else_mut());
                            temp = Node::from(AssignmentNodeData::new(
                                Node::from(DereferenceNodeData::new(make_temp_variable(
                                    &temp_name, &temp_type,
                                ))),
                                temp,
                            ));

                            // The RHS could be a conditional
                            extract_conditionals(&mut temp, temp_names);

                            swap(&mut temp, conditional_data.get_else_mut());
                        }

                        // Extract the original LHS and bind a reference to temp
                        let mut bind_temp = Node::Nothing;
                        swap(&mut bind_temp, data.get_lhs_mut());
                        bind_temp = Node::from(BindingNodeData::new(
                            temp_name,
                            Node::from(ReferenceNodeData::new_infer_type(
                                bind_temp,
                                ReferenceMode::Mutable,
                            )),
                        ));

                        // (if ... then ((deref temp) = A) else ((deref temp) = B))
                        let assign_temp = Node::from(conditional_data);

                        // Create the new enclosing sequence
                        let mut outer_sequence =
                            Node::from(SequenceNodeData::new(vec![bind_temp, assign_temp]));

                        // Replace the original node with the new sequence
                        swap(&mut outer_sequence, node);
                    }
                    else
                    {
                        // We know the RHS is a conditional
                        unreachable!();
                    }
                }
                _ =>
                {}
            }
        }

        Node::Sequence(data) =>
        {
            /* Sequence
                {... (if A then B else C)}
                =>
                {
                    let temp = Nothing
                    {... (if A then (temp = B) else (temp = C))}
                    temp
                }
            */

            if let Some(Node::Conditional(_)) = data.get_final_node()
            {
                // Ignore non-returning sequences, since we don't need to get a value out of those
                if !data.get_type().is_void()
                {
                    let target_node = data.get_final_node_mut().unwrap();

                    let temp_name = temp_names.next();
                    let target_type = target_node.get_type().clone();

                    // Extract the target node
                    let mut original_target = Node::Nothing;
                    swap(&mut original_target, target_node);

                    // original_target -> (if ...)
                    // target_node     -> Nothing

                    if let Node::Conditional(mut conditional_data) = original_target
                    {
                        let mut temp = Node::Nothing;

                        // Convert then branch into an assignment
                        {
                            swap(&mut temp, conditional_data.get_then_mut());
                            temp = Node::from(AssignmentNodeData::new(
                                make_temp_variable(&temp_name, &target_type),
                                temp,
                            ));

                            // The RHS could be a conditional
                            extract_conditionals(&mut temp, temp_names);

                            swap(&mut temp, conditional_data.get_then_mut());
                        }

                        // Convert else branch into an assignment
                        {
                            swap(&mut temp, conditional_data.get_else_mut());
                            temp = Node::from(AssignmentNodeData::new(
                                make_temp_variable(&temp_name, &target_type),
                                temp,
                            ));

                            // The RHS could be a conditional
                            extract_conditionals(&mut temp, temp_names);

                            swap(&mut temp, conditional_data.get_else_mut());
                        }

                        // let temp = Nothing
                        let bind_temp = Node::from(BindingNodeData::new_empty(
                            temp_name.clone(),
                            target_type.clone(),
                        ));

                        // (if ... then (temp = A) else (temp = B))
                        let mut assign_temp = Node::from(conditional_data);

                        // Put the conditional back where it was in the sequence
                        // note: this is different from default behavior in lift_conditional
                        swap(&mut assign_temp, target_node);

                        // Extract the original node, which contains the modified conditional
                        let mut use_temp = Node::Nothing;
                        swap(&mut use_temp, node);

                        let return_temp = make_temp_variable(&temp_name, &target_type);

                        // Create the new enclosing sequence
                        let mut outer_sequence =
                            Node::from(SequenceNodeData::new_transparent(vec![
                                bind_temp,
                                use_temp,
                                return_temp,
                            ]));

                        // Replace the original node with the new sequence
                        swap(&mut outer_sequence, node);
                    }
                    else
                    {
                        // We know the target is a conditional
                        unreachable!();
                    }
                }
            }
        }
        Node::Conditional(data) =>
        {
            if let Node::Conditional(_) = data.get_condition()
            {
                unimplemented!();
            }
        }

        Node::Function(_data) =>
        {
            // Functions don't need to change
        }

        Node::Type(_data) =>
        {
            // Types don't need to change
        }
        Node::Access(data) =>
        {
            /* Access
                ((if A then B else C) . property)
                =>
                {
                    let temp = Nothing
                    (if A then (temp = B) else (temp = C))
                    (temp . property)
                }
            */

            if let Node::Conditional(_) = data.get_target()
            {
                fn get_target(node: &mut Node) -> &mut Node
                {
                    if let Node::Access(data) = node
                    {
                        return data.get_target_mut();
                    }

                    unreachable!();
                }

                lift_conditional(node, get_target, temp_names);
            }
        }
    }
}

fn lift_conditional(
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

    if let Node::Conditional(mut conditional_data) = original_target
    {
        let mut temp = Node::Nothing;

        // Convert then branch into an assignment
        {
            swap(&mut temp, conditional_data.get_then_mut());
            temp = Node::from(AssignmentNodeData::new(
                make_temp_variable(&temp_name, &target_type),
                temp,
            ));

            // The RHS could be a conditional
            extract_conditionals(&mut temp, temp_names);

            swap(&mut temp, conditional_data.get_then_mut());
        }

        // Convert else branch into an assignment
        {
            swap(&mut temp, conditional_data.get_else_mut());
            temp = Node::from(AssignmentNodeData::new(
                make_temp_variable(&temp_name, &target_type),
                temp,
            ));

            // The RHS could be a conditional
            extract_conditionals(&mut temp, temp_names);

            swap(&mut temp, conditional_data.get_else_mut());
        }

        // let temp = Nothing
        let bind_temp = Node::from(BindingNodeData::new_empty(
            temp_name.clone(),
            target_type.clone(),
        ));

        // (if ... then (temp = A) else (temp = B))
        let assign_temp = Node::from(conditional_data);

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
        // If we called lift_conditional, we know the target is a conditional
        unreachable!();
    }
}
