use crate::language::nodes::*;
use crate::language::symbols;
use std::mem::swap;

pub fn apply(root_node: &mut Node)
{
    let mut params = ();

    root_node.recur_transformation(convert_method_definitions, &mut params);
    root_node.recur_transformation(convert_method_invocation, &mut params);
}

// Convert method definitions from (self.Function [args ...]) to (Function [self args ...])
fn convert_method_definitions(node: &mut Node, _params: &mut ())
{
    match node
    {
        Node::Type(data) =>
        {
            let self_type = data.get_instance_type().make_reference(Reference::Mutable);

            for method in data.get_methods_mut().iter_mut()
            {
                if method.get_scope() == MemberScope::Instance
                {
                    // Create the self argument
                    let self_argument =
                        ArgumentData::new(String::from(symbols::keywords::SELF), self_type.clone());

                    // Insert self as the first function argument
                    method
                        .get_function_data_mut()
                        .get_arguments_mut()
                        .insert(0, self_argument);
                    // note: The method is still marked as MemberScope::Instance
                    //       The function type metadata still lists it as an instance method
                }
            }
        }
        _ =>
        {}
    }
    node.recur_transformation(convert_method_definitions, _params);
}

// Convert accesses that result in instance methods into static function calls
// (instance.Function args...) => (T.Function instance args...)
fn convert_method_invocation(node: &mut Node, _params: &mut ())
{
    // Extract the target of an access node if it produces an instance method
    fn extract_method_access(node: &mut Node) -> Option<Node>
    {
        if let Node::Access(data) = node
        {
            let node_type = data.get_type();
            let target_type = data.get_target().get_type();

            if let DataType::Instance(instance_data) = target_type.get_data_type()
            {
                // The target is an instance of some type

                if let DataType::Function(function_data) = node_type.get_data_type()
                {
                    // The result of the access is a function

                    if function_data.get_metadata().get_type() == FunctionType::InstanceMethod
                    {
                        // The result of the access is an instance method

                        // Create a new variable to point to the static type
                        let type_name = instance_data.get_name();
                        let mut new_access_target = Node::from(VariableNodeData::new_typed(
                            type_name.clone(),
                            Type::from(TypeTypeData::new_empty(type_name.clone())),
                        ));

                        // Extract the instance node, replacing with the new variable
                        swap(&mut new_access_target, data.get_target_mut());

                        // Return the instance node for later use
                        return Some(new_access_target);
                    }
                }
            }
        }

        return None;
    }

    match node
    {
        Node::Call(call_data) =>
        {
            if let Some(instance_node) = extract_method_access(call_data.get_operator_mut())
            {
                // Insert the original instance as the first operand
                call_data.get_operands_mut().insert(0, instance_node);
            }
        }
        _ =>
        {}
    }
    node.recur_transformation(convert_method_invocation, _params);

}
