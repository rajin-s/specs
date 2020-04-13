// /*
// Impose restrictions on where sequence nodes can appear.
// The following cases are valid in Specs, but invalid in C
//     Binding:     let x = {...}
//     Assignment:  x = {...}
//     Calls:       ({...} {...})
//     Reference:   (ref {...})
//     Dereference: (@ {...})

// Bindings
//     let x = { ... y }
//     =>
//     /
//         let temp = nothing
//         { ... (temp = y) }
//         let x = temp
//     \

// Assignments
//     x = { ... y }
//     =>
//     /
//         let temp = nothing
//         { ... (temp = y) }
//         x = temp
//     \

// Calls:
//     ({... op} {... arg})
//     =>
//     /
//         let temp_op = nothing
//         { ... (temp_op = op) }
//         let temp_arg = nothing
//         { ... (temp_arg = arg) }

//         (temp_op temp_arg)
//     \
// Reference:
//     (ref {... x})
//     =>
//     {
//         let temp = nothing
//         {... temp = (ref x)}
//         temp
//     }
// Dereference:
//     (deref {... x})
//     =>
//     {
//         let temp  = nothing
//         { ... temp = (deref x)}
//         temp
//     }
// */

// use super::utilities::TempName;
// use crate::language::nodes::*;

// pub fn apply(root: &mut Node)
// {
//     let mut temp_name = TempName::new("seqresult");
//     extract(root, &mut temp_name);
// }

// fn extract(node: &mut Node, temp_name_generator: &mut TempName)
// {
//     // Make sure bindings to sequences have been simplified in child nodes
//     node.recur_transformation(extract, temp_name_generator);

//     // Perform passes until no further passes are needed
//     let mut needs_another_pass = extract_single_pass(node, temp_name_generator);
//     while needs_another_pass == true
//     {
//         needs_another_pass = extract_single_pass(node, temp_name_generator);
//     }
// }

// fn extract_single_pass(node: &mut Node, temp_name_generator: &mut TempName) -> bool
// {
//     match node
//     {
//         Node::Call(call_data) =>
//         {
//             // Handle operator first
//             if let Node::Sequence(_) = call_data.get_operator()
//             {
//                 fn get_operator(node: &mut Node) -> &mut Node
//                 {
//                     if let Node::Call(data) = node
//                     {
//                         return data.get_operator_mut();
//                     }

//                     unreachable!();
//                 }

//                 lift_sequence(node, get_operator, temp_name_generator);

//                 // We might need to do more passes for each operand
//                 return true;
//             }

//             // No more passes are needed
//             return false;
//         }

//         Node::Reference(data) => {

//         }
//         Node::Dereference(data) => {}

//         Node::Binding(binding_data) =>
//         {
//             if let Node::Sequence(_) = binding_data.get_binding()
//             {
//                 fn get_binding(node: &mut Node) -> &mut Node
//                 {
//                     if let Node::Binding(data) = node
//                     {
//                         return data.get_binding_mut();
//                     }

//                     unreachable!();
//                 }

//                 lift_sequence(node, get_binding, temp_name_generator);
//             }

//             // Bindings will only ever need one pass
//             return false;
//         }
//         Node::Assignment(assignment_data) =>
//         {
//             // Handle LHS first
//             if let Node::Sequence(_) = assignment_data.get_lhs()
//             {
//                 fn get_lhs(node: &mut Node) -> &mut Node
//                 {
//                     if let Node::Assignment(data) = node
//                     {
//                         return data.get_lhs_mut();
//                     }

//                     unreachable!();
//                 }

//                 lift_sequence_with_reference(node, get_lhs, temp_name_generator);

//                 // We might need to another pass for the LHS
//                 return true;
//             }
//             else if let Node::Sequence(_) = assignment_data.get_rhs()
//             {
//                 fn get_rhs(node: &mut Node) -> &mut Node
//                 {
//                     if let Node::Assignment(data) = node
//                     {
//                         return data.get_rhs_mut();
//                     }

//                     unreachable!();
//                 }

//                 lift_sequence(node, get_rhs, temp_name_generator);
//             }

//             // No more passes are needed
//             return false;
//         }
//         _ =>
//         {
//             return false;
//         }
//     }
// }

// fn lift_sequence(
//     original_node: &mut Node,
//     target_node_accessor: fn(&mut Node) -> &mut Node,
//     temp_name_generator: &mut TempName,
// )
// {
//     // Helper functions
//     fn make_temp_variable(name: &String, node_type: &Type) -> Node
//     {
//         return Node::from(VariableNodeData::new_typed(name.clone(), node_type.clone()));
//     }

//     use std::mem::swap;

//     // Get the target node from the original node
//     // note: this is needed because we can't borrow both at the same time as function arguments
//     let target_node = target_node_accessor(original_node);

//     // Create a temporary variable to hold the result of the sequence
//     let temp_name = temp_name_generator.next();
//     let result_type = target_node.get_type().clone();

//     // Extract the target node
//     let mut original_target = make_temp_variable(&temp_name, &result_type);
//     let new_target = target_node;
//     swap(&mut original_target, new_target);

//     // original_target -> the sequence that is being extracted
//     // new_target      -> a new temporary variable put in place of the sequence

//     if let Node::Sequence(mut sequence_data) = original_target
//     {
//         // Change the last node of the sequence to an assignment of to the new temporary variable
//         if let Some(final_sequence_node) = sequence_data.get_nodes_mut().last_mut()
//         {
//             // Extract the final node of the sequence
//             let mut final_node = Node::Nothing;
//             let placeholder = final_sequence_node;
//             swap(&mut final_node, placeholder);

//             // Turn the final node into an assignment
//             final_node = Node::from(AssignmentNodeData::new(
//                 make_temp_variable(&temp_name, &result_type),
//                 final_node,
//             ));

//             // Make sure the RHS of the assignment isn't also a sequence
//             extract(&mut final_node, temp_name_generator);

//             // Put the assignment back at the end of the sequence
//             swap(&mut final_node, placeholder);
//         }
//         else
//         {
//             // If there weren't any nodes, this would not have passed type checking
//             unreachable!();
//         }

//         // let temp = nothing
//         let bind_temp = Node::from(BindingNodeData::new_empty(temp_name.clone(), result_type));

//         // { ... (temp = something) }
//         let assign_temp = Node::from(sequence_data);

//         // Extract the original node, which now uses temp in place of a sequence
//         let mut use_temp = Node::Nothing;
//         swap(&mut use_temp, original_node);

//         // Create the new enclosing sequence
//         let mut outer_sequence = Node::from(SequenceNodeData::new_transparent(vec![
//             bind_temp,
//             assign_temp,
//             use_temp,
//         ]));

//         // Replace the original node with the new sequence
//         swap(&mut outer_sequence, original_node);
//     }
//     else
//     {
//         // We already checked that the target was a sequence
//         unreachable!();
//     }
// }

// fn lift_sequence_with_reference(
//     original_node: &mut Node,
//     target_node_accessor: fn(&mut Node) -> &mut Node,
//     temp_name_generator: &mut TempName,
// )
// {
//     // Helper functions
//     fn make_temp_variable(name: &String, node_type: &Type) -> Node
//     {
//         return Node::from(VariableNodeData::new_typed(name.clone(), node_type.clone()));
//     }

//     use std::mem::swap;

//     // Get the target node from the original node
//     // note: this is needed because we can't borrow both at the same time as function arguments
//     let target_node = target_node_accessor(original_node);

//     // Create a temporary variable to hold the result of the sequence
//     let temp_name = temp_name_generator.next();
//     let result_type = target_node.get_type();
//     let temp_type = result_type.make_reference(Reference::Mutable);

//     // Extract the target node, replacing it with (deref temp)
//     let mut original_target = make_temp_variable(&temp_name, &result_type);
//     let new_target = target_node;
//     swap(&mut original_target, new_target);

//     // original_target -> the sequence that is being extracted
//     // new_target      -> a new temporary variable put in place of the sequence

//     if let Node::Sequence(mut sequence_data) = original_target
//     {
//         // Change the last node of the sequence to an assignment of to the new temporary variable
//         if let Some(final_sequence_node) = sequence_data.get_nodes_mut().last_mut()
//         {
//             // Extract the final node of the sequence
//             let mut final_node = Node::Nothing;
//             let placeholder = final_sequence_node;
//             swap(&mut final_node, placeholder);

//             // Turn the final node into an assignment
//             final_node = Node::from(AssignmentNodeData::new(
//                 make_temp_variable(&temp_name, &result_type),
//                 final_node,
//             ));

//             // Make sure the RHS of the assignment isn't also a sequence
//             extract(&mut final_node, temp_name_generator);

//             // Put the assignment back at the end of the sequence
//             swap(&mut final_node, placeholder);
//         }
//         else
//         {
//             // If there weren't any nodes, this would not have passed type checking
//             unreachable!();
//         }

//         // let temp = nothing
//         let bind_temp = Node::from(BindingNodeData::new_empty(temp_name.clone(), result_type));

//         // { ... (temp = something) }
//         let assign_temp = Node::from(sequence_data);

//         // Extract the original node, which now uses temp in place of a sequence
//         let mut use_temp = Node::Nothing;
//         swap(&mut use_temp, original_node);

//         // Create the new enclosing sequence
//         let mut outer_sequence = Node::from(SequenceNodeData::new_transparent(vec![
//             bind_temp,
//             assign_temp,
//             use_temp,
//         ]));

//         // Replace the original node with the new sequence
//         swap(&mut outer_sequence, original_node);
//     }
//     else
//     {
//         // We already checked that the target was a sequence
//         unreachable!();
//     }
// }