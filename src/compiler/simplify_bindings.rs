use super::utilities::TempName;
use crate::language::nodes::*;

pub fn apply(root: &mut Node)
{
    let mut temp_name = TempName::new("bindseq");
    simplify_sequences(root, &mut temp_name);
}

/*
Bindings
let x = { ... y }

=>

/
let temp = nothing
{ ... (temp = y) }
let x = temp
\
Assignments
x = { ... y }

=>

/
let temp = nothing
{ ... (temp = y) }
x = temp
\
*/
fn simplify_sequences(node: &mut Node, temp_name_generator: &mut TempName)
{
    use std::mem::swap;

    // Make sure bindings to sequences have been simplified in child nodes
    node.recur_transformation(simplify_sequences, temp_name_generator);

    match node
    {
        Node::Binding(binding_data) =>
        {
            if let Node::Sequence(_) = binding_data.get_binding()
            {
                let binding_name = binding_data.get_name().clone();
                let binding_type = binding_data.get_binding_type().clone();

                // Create a temporary variable to hold the result of the sequence
                let temp_name = temp_name_generator.next();

                // Extract the original binding
                let mut original_binding = Node::Nothing;
                let new_binding = binding_data.get_binding_mut();
                swap(&mut original_binding, new_binding);

                if let Node::Sequence(mut sequence_data) = original_binding
                {
                    // Change the last node of the sequence to an assignment to the new temporary variable
                    if let Some(final_sequence_node) = sequence_data.get_nodes_mut().last_mut()
                    {
                        // Extract the final node
                        let mut final_node = Node::Nothing;
                        let temp = final_sequence_node;
                        swap(&mut final_node, temp);

                        // Create a new assignment to replace the final node
                        final_node = Node::from(AssignmentNodeData::new(
                            Node::from(VariableNodeData::new_typed(
                                temp_name.clone(),
                                binding_type.clone(),
                            )),
                            final_node,
                        ));

                        // Make sure the rhs of the assignment isn't a sequence
                        simplify_sequences(&mut final_node, temp_name_generator);

                        // Put the new assignment back at the end of the sequence
                        swap(temp, &mut final_node);
                    }

                    // let temp = nothing
                    // note: make sure binding type is preserved
                    let bind_temp = Node::from(BindingNodeData::new_empty(
                        temp_name.clone(),
                        binding_type.clone(),
                    ));

                    // { ... (temp = y) }
                    let inner_sequence = Node::from(sequence_data);

                    // let x = temp
                    // note: make sure binding type is preserved
                    let rebind_temp = Node::from(BindingNodeData::new(
                        binding_name.clone(),
                        Node::from(VariableNodeData::new_typed(
                            temp_name.clone(),
                            binding_type.clone(),
                        )),
                    ));

                    // Create the new enclosing sequence
                    // note: is transparent so the binding isn't trapped inside a new scope
                    let outer_sequence_nodes = vec![bind_temp, inner_sequence, rebind_temp];
                    let mut outer_sequence =
                        Node::from(SequenceNodeData::new_transparent(outer_sequence_nodes));

                    // Replace the original node with the new sequence
                    swap(node, &mut outer_sequence);
                }
                else
                {
                    unreachable!();
                }
            }
        }
        Node::Assignment(assignment_data) =>
        {
            if let Node::Sequence(_) = assignment_data.get_rhs()
            {
                // Create a temporary variable to hold the result of the sequence
                let temp_name = temp_name_generator.next();
                let rhs_type = assignment_data.get_rhs().get_type().clone();

                // Extract the original rhs and replace with with the new temporary
                let mut original_rhs = Node::from(VariableNodeData::new(temp_name.clone()));
                let new_rhs = assignment_data.get_rhs_mut();
                swap(&mut original_rhs, new_rhs);

                if let Node::Sequence(mut sequence_data) = original_rhs
                {
                    // Change the last node of the sequence to an assignment to the new temporary variable
                    if let Some(final_sequence_node) = sequence_data.get_nodes_mut().last_mut()
                    {
                        // Extract the final node
                        let mut final_node = Node::Nothing;
                        let temp = final_sequence_node;
                        swap(&mut final_node, temp);

                        // Create a new assignment to replace the final node
                        final_node = Node::from(AssignmentNodeData::new(
                            Node::from(VariableNodeData::new(temp_name.clone())),
                            final_node,
                        ));

                        // Make sure the rhs of the assignment isn't a sequence
                        simplify_sequences(&mut final_node, temp_name_generator);

                        // Put the new assignment back at the end of the sequence
                        swap(temp, &mut final_node);
                    }

                    // let temp = nothing
                    let create_temp =
                        Node::from(BindingNodeData::new_empty(temp_name.clone(), rhs_type));

                    // { ... (temp = y) }
                    let inner_sequence = Node::from(sequence_data);

                    // Extract original node, which is now (x = temp)
                    let mut assignment_node = Node::Nothing;
                    swap(node, &mut assignment_node);

                    // Create the new enclosing sequence
                    // note: transparency doesn't matter, since this isn't a binding
                    let outer_sequence_nodes = vec![create_temp, inner_sequence, assignment_node];
                    let mut outer_sequence =
                        Node::from(SequenceNodeData::new_transparent(outer_sequence_nodes));

                    // Replace the original node with the new sequence
                    swap(node, &mut outer_sequence);
                }
                else
                {
                    unreachable!();
                }
            }
        }
        _ =>
        {}
    }
}
