use super::parse_node::*;
use crate::utilities::IntoN;

#[derive(PartialEq, Eq)]
pub enum Mode
{
    Any,
    StaticOnly,
    InstanceOnly,
}

pub fn definition(
    elements: Vec<SExpression>,
    mode: Mode,
    source: &Source,
) -> ResultLog<(NodeKind, PartialNodeData, Vec<ParseNode>), Error>
{
    use SExpression::*;

    let mut iter = elements.into_iter();
    iter.next(); // skip the fn keyword

    // Get the function name and check if it's an instance method
    let (name, is_instance_method) = match iter.next()
    {
        // fn name => (basic function / static method)
        Some(Symbol(name, _)) => (name.clone(), false),

        // fn (...) => (self . name) if instance methods are allowed
        Some(List(BracketType::Round, name_elements, name_source))
            if mode == Mode::Any || mode == Mode::InstanceOnly =>
        {
            match name_elements.as_slice()
            {
                // (self . name)
                [Symbol(x1, _), Symbol(x2, _), Symbol(name, _)]
                    if x1 == keywords::SELF && x2 == operators::ACCESS =>
                {
                    (name.clone(), true)
                }
                _ =>
                {
                    let error = Error::BadFunctionName(
                        format!(
                            "Unexpected function name layout: {}, expected (self . name)",
                            List(BracketType::Round, name_elements, name_source)
                        ),
                        source.clone(),
                    );
                    return ResultLog::new_error(error);
                }
            }
        }
        Some(list) =>
        {
            let error = Error::BadFunctionName(
                format!("Unexpected list in function name: {}", list),
                source.clone(),
            );
            return ResultLog::new_error(error);
        }
        None =>
        {
            let error = Error::Internal(format!("Function expression has no elements"));
            return ResultLog::new_error(error);
        }
    };

    // Get the function arguments, return type, and body expressions
    let (maybe_arguments, maybe_return_type, body) =
        match (iter.next(), iter.next(), iter.next(), iter.next())
        {
            // {...}
            (Some(body), None, None, None) => match &body
            {
                List(BracketType::Curly, _, _) => (None, None, body),
                _ =>
                {
                    let error = Error::BadFunctionLayout(format!("Expected body"), source.clone());
                    return ResultLog::new_error(error);
                }
            },
            // <...> {...}
            (Some(arguments), Some(body), None, None) => match (arguments, &body)
            {
                (List(BracketType::None, argument_elements, _), List(BracketType::Curly, _, _)) =>
                {
                    (Some(argument_elements), None, body)
                }
                _ =>
                {
                    let error = Error::BadFunctionLayout(
                        format!("Expected arguments and body"),
                        source.clone(),
                    );
                    return ResultLog::new_error(error);
                }
            },
            // -> Type {...}
            (Some(Symbol(x1, _)), Some(return_type), Some(body), None)
                if x1 == keywords::RETURNS =>
            {
                match &body
                {
                    List(BracketType::Curly, _, _) => (None, Some(return_type), body),
                    _ =>
                    {
                        let error =
                            Error::BadFunctionLayout(format!("Expected body"), source.clone());
                        return ResultLog::new_error(error);
                    }
                }
            }
            // -> Type <...> {...}
            // <...> -> Type {...}
            (Some(Symbol(x1, _)), Some(return_type), Some(arguments), Some(body))
            | (Some(arguments), Some(Symbol(x1, _)), Some(return_type), Some(body))
                if x1 == keywords::RETURNS =>
            {
                match (arguments, &body)
                {
                    (
                        List(BracketType::None, argument_elements, _),
                        List(BracketType::Curly, _, _),
                    ) => (Some(argument_elements), Some(return_type), body),
                    _ =>
                    {
                        let error = Error::BadFunctionLayout(
                            format!("Expected arguments and body"),
                            source.clone(),
                        );
                        return ResultLog::new_error(error);
                    }
                }
            }
            _ =>
            {
                let error = Error::BadFunctionLayout(
                    format!("Unexpected argument/return/body layout"),
                    source.clone(),
                );
                return ResultLog::new_error(error);
            }
        };

    // If a return type was given, create a new child ParseItem, otherwise use a complete void type
    //  note: void type isn't added to child_items, since it doesn't need to onto the parse queue
    let return_type = match maybe_return_type
    {
        Some(s_expression) => ParseNode::UnparsedType(s_expression),
        None => ParseNode::ParsedType(Type::Void),
    };

    let mut warnings = Vec::new();
    let mut errors = Vec::new();

    let mut argument_names = Vec::new();
    let mut types = vec![return_type];

    match maybe_arguments
    {
        Some(argument_s_expressions) =>
        {
            for s_expression in argument_s_expressions
            {
                match parse_argument(s_expression, source.clone())
                {
                    ResultLog::Ok((name, t)) =>
                    {
                        argument_names.push(name);
                        types.push(t);
                    }
                    ResultLog::Warn((name, t), mut new_warnings) =>
                    {
                        warnings.append(&mut new_warnings);

                        argument_names.push(name);
                        types.push(t);
                    }
                    ResultLog::Error(mut new_errors, mut new_warnings) =>
                    {
                        errors.append(&mut new_errors);
                        warnings.append(&mut new_warnings);
                    }
                }
            }
        }
        None => (),
    };

    ResultLog::maybe_error(
        (
            NodeKind::Function,
            PartialNodeData::Function(name, is_instance_method, argument_names, types),
            vec![ParseNode::UnparsedNode(body)],
        ),
        warnings,
        errors,
    )
}

fn parse_argument(
    s_expression: SExpression,
    function_source: Source,
) -> ResultLog<(String, ParseNode), Error>
{
    use SExpression::*;

    match s_expression
    {
        List(BracketType::Square, elements, _) if elements.len() == 2 => match elements.into_2()
        {
            (Symbol(name, _), argument_type) =>
            {
                ResultLog::Ok((name, ParseNode::UnparsedType(argument_type)))
            }
            (e1, e2) =>
            {
                let error = Error::BadFunctionArgument(format!("[{} {}]", e1, e2), function_source);
                ResultLog::new_error(error)
            }
        },
        s_expression =>
        {
            let error = Error::BadFunctionArgument(format!("{}", s_expression), function_source);
            ResultLog::new_error(error)
        }
    }
}
