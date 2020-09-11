use super::common::*;

///
/// ## Flatten Bindings Pass
///
/// - Converts complex bindings into an empty temporary variable followed by a complex expression
///     that assigns to the temporary
///
/// ex.
///
///     let foo = { bar }
///     ==>
///     /
///         let _temp = [nothing]
///         {
///             temp = bar
///         }
///         let foo = temp
///     \
///
pub struct FlattenBindings
{
    temp_names: TempNameGenerator,
}

impl FlattenBindings
{
    pub fn new() -> FlattenBindings
    {
        FlattenBindings {
            temp_names: TempNameGenerator::new("xbind"),
        }
    }
}

pub struct PassState {}

impl PassState
{
    pub fn new() -> PassState
    {
        PassState {}
    }
}

impl RecurTransform<Node, PassState, Error> for FlattenBindings
{
    fn get_root_state(&mut self, _node: &Node) -> PassState
    {
        PassState::new()
    }

    fn exit(&mut self, node: &mut Node, _state: &mut PassState) -> ResultLog<(), Error>
    {
        match node
        {
            Node::Binding(binding) if binding.get_binding().is_complex() =>
            {
                let binding_type = binding.get_binding().get_type();
                let binding_source_outer = binding.get_source();
                let binding_source_inner = binding.get_binding().get_source();

                // Create a new temporary variable

                let temporary_name = self.temp_names.next();
                let bind_temporary = Binding::new(
                    temporary_name.clone(),
                    Node::nothing_typed(binding_type.clone(), binding_source_inner),
                    binding_source_outer.clone(),
                )
                .to_node();

                // Extract the original binding, replacing it with the temporary variable

                let temporary_variable = Variable::new_typed(
                    temporary_name.clone(),
                    binding_type.clone(),
                    binding_source_outer.clone(),
                )
                .to_node();

                let mut assign_temporary = binding.get_binding_mut().extract(temporary_variable);

                // Wrap the result of the original binding with an assignment to the temporary variable

                let wrap_pass = WrapPass::new(|result_node| {
                    let source = result_node.get_source();
                    let temporary_variable = Variable::new_typed(
                        temporary_name.clone(),
                        binding_type.clone(),
                        source.clone(),
                    )
                    .to_node();
                    Assign::new(temporary_variable, result_node, source).to_node()
                });

                wrap_pass.apply(&mut assign_temporary);

                // Get the original binding node (let original = temporary)

                let bind_original = node.extract_temp();
                let original_source = bind_original.get_source();

                let sequence = Sequence::new(
                    SequenceMode::Transparent,
                    vec![bind_temporary, assign_temporary, bind_original],
                    original_source,
                )
                .to_node();

                // Put the sequence back in place of the original node

                *node = sequence;
            }
            _ => (),
        }
        ResultLog::Ok(())
    }
}
