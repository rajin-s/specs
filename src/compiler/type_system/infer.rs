use super::super::common::*;
use super::Infer;

///
/// Pass state for Infer
///
/// - Maps symbol names to types, either newly created or referenced from other parts of the tree
///
type State = BindingState<Indirect<Type>>;

impl RecurTransform<Node, State, Error> for Infer
{
    fn get_root_state(&mut self, _node: &Node) -> State
    {
        State::root()
    }

    fn get_child_states(&mut self, state: &State, node: &Node) -> Vec<ChildState<State>>
    {
        match node
        {
            Node::Sequence(sequence) if !sequence.is_transparent() =>
            {
                // Create a new scope with all this sequence's functions
                //  Both bindings and definitions from the parent scope are visible

                let mut new_state = State::empty(state, true, true);

                let get_function_type = |function: &Function| function.get_type();
                new_state.add_definitions_from_functions(sequence.get_nodes(), get_function_type);

                vec![ChildState::New(new_state)]
            }
            Node::Function(function) =>
            {
                // Create a new scope with all this function's arguments
                //  Only definitions from the parent scope are visible

                let mut new_state = State::empty(state, true, false);

                let get_argument_type = |argument: &Argument| argument.get_type();
                new_state.add_bindings_from_arguments(function.get_arguments(), get_argument_type);

                vec![ChildState::New(new_state)]
            }

            _ => vec![ChildState::Inherit],
        }
    }

    fn exit(&mut self, node: &mut Node, state: &mut State) -> ResultLog<(), Error>
    {
        match node
        {
            Node::Variable(variable) =>
            {
                // Loop up variables in the current scope

                match state.lookup(variable.get_name())
                {
                    Some(t) => variable.set_type(t.clone()),
                    None =>
                    {
                        return ResultLog::new_error(Error::UnboundSymbolType(
                            variable.get_name().clone(),
                            variable.get_source(),
                        ));
                    }
                }
            }
            Node::PrimitiveOperator(operator) =>
            {
                let operator_type = match operator.get_value()
                {
                    Operator::And | Operator::Or | Operator::ExclusiveOr =>
                    {
                        // Binary logic operators are always (bool bool -> bool)

                        FunctionType::from(
                            vec![
                                basic_types::indirect::boolean(),
                                basic_types::indirect::boolean(),
                            ],
                            basic_types::indirect::boolean(),
                        )
                    }
                    Operator::Not =>
                    {
                        // Unary logic operators are always (bool -> bool)

                        FunctionType::from(
                            vec![basic_types::indirect::boolean()],
                            basic_types::indirect::boolean(),
                        )
                    }
                    _ =>
                    {
                        // Other operators need to be inferred from their operands

                        return ResultLog::Ok(());
                    }
                };

                operator.set_type(Indirect::new(operator_type.to_type()))
            }
            Node::Call(call) =>
            {
                let mut errors = Vec::new();
                let mut warnings = Vec::new();

                let call_source = call.get_source();
                let (operator, operands) = call.get_all_mut();

                let operator_type = operator.get_type();
                if operator_type.borrow().is_unknown()
                {
                    // Try to infer the operator type from its operands if it is not already known

                    match operator
                    {
                        Node::PrimitiveOperator(primitive) =>
                        {
                            match infer_primitive_operator_type(
                                primitive.get_value(),
                                operands,
                                call_source.clone(),
                            )
                            {
                                ResultLog::Ok(t) =>
                                {
                                    primitive.set_type(Indirect::new(t));
                                }
                                ResultLog::Warn(t, mut new_warnings) =>
                                {
                                    primitive.set_type(Indirect::new(t));
                                    warnings.append(&mut new_warnings);
                                }
                                ResultLog::Error(mut new_errors, mut new_warnings) =>
                                {
                                    errors.append(&mut new_errors);
                                    warnings.append(&mut new_warnings);
                                }
                            }
                        }
                        _ => warnings.push(Error::FailedToInferOperator(
                            format!("Can't infer type of non-primitive operator: {}", operator),
                            call_source,
                        )),
                    }
                }

                // Get return type from operator if possible

                let operator_type = operator.get_type();
                let operator_type = operator_type.borrow();

                match &*operator_type
                {
                    Type::Function(function) => call.set_type(function.get_return_type()),
                    _ => (),
                }

            }
            Node::Reference(reference) =>
            {
                let target_type = reference.get_target().get_type();
                let reference_type =
                    ReferenceType::from(reference.get_mode(), target_type).to_type();

                reference.set_type(Indirect::new(reference_type));
            }
            Node::Dereference(dereference) =>
            {
                let target_type = dereference.get_target().get_type();

                let dereferenced = target_type.borrow().dereference();
                match dereferenced
                {
                    Some(t) => dereference.set_type(t),
                    None =>
                    {
                        return ResultLog::new_error(Error::BadDereferenceType(
                            target_type,
                            dereference.get_source(),
                        ));
                    }
                }
            }
            Node::Assign(_) =>
            {
                // Assign nodes already have a void type
            }
            Node::Access(_) => unimplemented!(),
            Node::Binding(binding) =>
            {
                // Track the binding in the current state
                //  (the node itself already has a void type)

                state.add_binding(binding.get_name(), binding.get_binding().get_type());
            }
            Node::Sequence(sequence) =>
            {
                let result_type = match sequence.get_result_node()
                {
                    Some(result) => result.get_type(),
                    None => basic_types::indirect::void(),
                };

                sequence.set_type(result_type);
            }
            Node::Conditional(conditional) =>
            {
                let else_type = conditional.get_else().get_type();
                conditional.set_type(else_type);
            }
            Node::Function(_) =>
            {
                // Function types should have been built already
            }
            Node::Class(_) =>
            {
                // Class types should have been built already
            }

            _ =>
            {}
        }

        ResultLog::Ok(())
    }
}

fn infer_primitive_operator_type(
    operator: Operator,
    operands: &Vec<Node>,
    call_source: Source,
) -> ResultLog<Type, Error>
{
    macro_rules! match_primitive_call {
        {
            ($operator:expr, $operands:expr, $source:expr)
            {
                $(
                    $($op:pat)|* =>
                    {
                        $(
                            ( $($name:ident : $t:expr),* => $result:ident ),
                        )*
                    }
                )*
            }
        } =>
        {
            match operator
            {
                $(
                    $($op)|* =>
                    {
                        match $operands.as_slice()
                        {
                            $(
                                [$($name,)*] if $( $name.is_type(&$t) )&&* =>
                                {
                                    FunctionType::new(
                                        vec![$( $t, )*],
                                        $result,
                                    )
                                }
                            )*
                            _ =>
                            {
                                let error = Error::UnexpectedOperands(
                                    format!("Unexpected operands for primitive operator: {}", $operator),
                                    call_source
                                );
                                return ResultLog::new_error(error);
                            }
                        }
                    }
                )*
                _ =>
                {
                    let error = Error::Internal(
                        format!("Unexpected primitive operator: {}", $operator),
                    );
                    return ResultLog::new_error(error);
                }
            }
        };
    }

    use Type::*;

    let function_type = match_primitive_call! {
        (operator, operands, call_source)
        {
            Operator::Subtract =>
            {
                (a: Integer, b: Integer => Integer),
                (a: Float, b: Float => Float),

                (a: Integer, b: Float => Float),
                (a: Float, b: Integer => Float),

                (a: Float => Float),
                (a: Integer => Integer),
            }

            Operator::Add
            | Operator::Multiply
            | Operator::Divide =>
            {
                (a: Integer, b: Integer => Integer),
                (a: Float, b: Float => Float),

                (a: Integer, b: Float => Float),
                (a: Float, b: Integer => Float),
            }
            
            Operator::Greater
            | Operator::Less
            | Operator::GreaterEqual
            | Operator::LessEqual =>
            {
                (a: Integer, b: Integer => Boolean),
                (a: Float, b: Float => Boolean),
                (a: Integer, b: Float => Boolean),
                (a: Float, b: Integer => Boolean),
            }

            Operator::Equal
            | Operator::NotEqual =>
            {
                (a: Integer, b: Integer => Boolean),
                (a: Float, b: Float => Boolean),
                (a: Boolean, b: Boolean => Boolean),
                (a: Integer, b: Float => Boolean),
                (a: Float, b: Integer => Boolean),
            }
        }
    };

    ResultLog::Ok(function_type.to_type())
}
