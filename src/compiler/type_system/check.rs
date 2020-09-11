use super::super::common::*;
use super::Check;

pub struct State {}

impl RecurTransform<Node, State, Error> for Check
{
    fn get_root_state(&mut self, _root: &Node) -> State
    {
        State {}
    }

    fn exit(&mut self, node: &mut Node, _state: &mut State) -> ResultLog<(), Error>
    {
        match node
        {
            Node::Call(call) =>
            {
                let operator_type_indirect = call.get_operator().get_type();
                let operator_type = operator_type_indirect.borrow();

                let operand_types: Vec<_> = call
                    .get_operands()
                    .iter()
                    .map(|operand| operand.get_type())
                    .collect();

                match &*operator_type
                {
                    Type::Function(function) =>
                    {
                        // Check that the number of operands matches the number of function arguments

                        if function.get_arguments().len() != operand_types.len()
                        {
                            let argument_types =
                                function.get_arguments().iter().map(|t| t.clone()).collect();
                            let error = Error::BadOperandTypes(
                                operand_types,
                                argument_types,
                                call.get_source(),
                            );
                            return ResultLog::new_error(error);
                        }

                        // Check that each operand type matches each function argument type

                        let mut operands_match_arguments = true;

                        for (t_operand, t_argument) in
                            operand_types.iter().zip(function.get_arguments().iter())
                        {
                            let t_operand = t_operand.borrow();
                            let t_argument = t_argument.borrow();

                            if &*t_operand != &*t_argument
                            {
                                operands_match_arguments = false;
                                break;
                            }
                        }

                        if !operands_match_arguments
                        {
                            let argument_types =
                                function.get_arguments().iter().map(|t| t.clone()).collect();
                            let error = Error::BadOperandTypes(
                                operand_types,
                                argument_types,
                                call.get_source(),
                            );
                            return ResultLog::new_error(error);
                        }
                    }
                    _ =>
                    {
                        let error = Error::BadOperatorType(
                            operator_type_indirect.clone(),
                            call.get_source(),
                        );
                        return ResultLog::new_error(error);
                    }
                }
            }
            Node::Assign(_) =>
            {}
            Node::Sequence(_) =>
            {}
            Node::Conditional(_) =>
            {}
            Node::Function(_) =>
            {}
            _ => (),
        }
        ResultLog::Ok(())
    }
}
