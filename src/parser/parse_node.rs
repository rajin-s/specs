use super::s_expression::*;

use crate::language::nodes::*;
use crate::language::symbols;

use super::errors::*;
use super::ParseResult;

pub fn parse_node_recursive(expression: &SExpression) -> ParseResult
{
    use SExpression::*;

    let mut errors = ParseErrorList::new();

    match expression
    {
        Symbol(symbol) =>
        {
            if let Some(node) = parse_integer(symbol)
            {
                return Ok(node);
            }
            else if let Some(node) = parse_boolean(symbol)
            {
                return Ok(node);
            }
            else if let Some(node) = parse_primitive_operator(symbol)
            {
                return Ok(node);
            }
            else
            {
                let node = Node::from(VariableNodeData::new(symbol.clone()));
                return Ok(node);
            }
        }
        SExpression::List(bracket_type, elements) =>
        {
            match bracket_type
            {
                BracketType::Round =>
                {
                    match elements.as_slice()
                    {
                        // (a ~ b)
                        [a, Symbol(op), b] if symbols::operators::is_binary(op) =>
                        {
                            if let Some(operator_node) = parse_primitive_operator(op)
                            {
                                let a_result = parse_node_recursive(a);
                                let b_result = parse_node_recursive(b);
                                match (a_result, b_result)
                                {
                                    (Ok(a_node), Ok(b_node)) =>
                                    {
                                        let node = Node::from(CallNodeData::new(
                                            operator_node,
                                            vec![a_node, b_node],
                                        ));
                                        return Ok(node);
                                    }
                                    (a_result, b_result) =>
                                    {
                                        if let Err(mut new_errors) = a_result
                                        {
                                            errors.append(&mut new_errors);
                                        }
                                        if let Err(mut new_errors) = b_result
                                        {
                                            errors.append(&mut new_errors);
                                        }
                                        return Err(errors);
                                    }
                                }
                            }
                            else
                            {
                                errors.push(ParseError::Internal(format!(
                                    "Unhandled primitive binary operator {}",
                                    op
                                )));
                                return Err(errors);
                            }
                        }
                        // (ref a)
                        [Symbol(reference_operator), a]
                            if reference_operator == symbols::operators::REFERENCE =>
                        {
                            match parse_node_recursive(a)
                            {
                                Ok(target_node) =>
                                {
                                    let node = Node::from(ReferenceNodeData::new(
                                        target_node,
                                        Reference::Immutable,
                                    ));
                                    return Ok(node);
                                }
                                Err(mut new_errors) =>
                                {
                                    errors.append(&mut new_errors);
                                    return Err(errors);
                                }
                            }
                        }
                        // (mut-ref a)
                        [Symbol(reference_operator), a]
                            if reference_operator == symbols::operators::MUTABLE_REFERENCE =>
                        {
                            match parse_node_recursive(a)
                            {
                                Ok(target_node) =>
                                {
                                    let node = Node::from(ReferenceNodeData::new(
                                        target_node,
                                        Reference::Mutable,
                                    ));
                                    return Ok(node);
                                }
                                Err(mut new_errors) =>
                                {
                                    errors.append(&mut new_errors);
                                    return Err(errors);
                                }
                            }
                        }
                        // (@ a)
                        [Symbol(dereference_operator), a]
                            if dereference_operator == symbols::operators::DEREFERENCE =>
                        {
                            match parse_node_recursive(a)
                            {
                                Ok(target_node) =>
                                {
                                    let node = Node::from(DereferenceNodeData::new(target_node));
                                    return Ok(node);
                                }
                                Err(mut new_errors) =>
                                {
                                    errors.append(&mut new_errors);
                                    return Err(errors);
                                }
                            }
                        }

                        // (let name = expression)
                        [Symbol(binding_keyword), Symbol(binding_name), Symbol(binding_operator), binding_expression]
                            if binding_keyword == symbols::keywords::BINDING
                                && binding_operator == symbols::operators::ASSIGN =>
                        {
                            match parse_node_recursive(binding_expression)
                            {
                                Ok(binding_node) =>
                                {
                                    let node = Node::from(BindingNodeData::new(
                                        binding_name.clone(),
                                        binding_node,
                                    ));
                                    return Ok(node);
                                }
                                Err(mut new_errors) =>
                                {
                                    errors.append(&mut new_errors);
                                    return Err(errors);
                                }
                            }
                        }
                        // (a = b)
                        [a, Symbol(binding_operator), b]
                            if binding_operator == symbols::operators::ASSIGN =>
                        {
                            let a_result = parse_node_recursive(a);
                            let b_result = parse_node_recursive(b);

                            match (a_result, b_result)
                            {
                                (Ok(a_node), Ok(b_node)) =>
                                {
                                    let node = Node::from(AssignmentNodeData::new(a_node, b_node));
                                    return Ok(node);
                                }
                                (a_result, b_result) =>
                                {
                                    if let Err(mut new_errors) = a_result
                                    {
                                        errors.append(&mut new_errors);
                                    }
                                    if let Err(mut new_errors) = b_result
                                    {
                                        errors.append(&mut new_errors);
                                    }
                                    return Err(errors);
                                }
                            }
                        }

                        // (if condition then then_branch else else_branch)
                        [Symbol(if_keyword), condition_expression, Symbol(then_keyword), then_expression, Symbol(else_keyword), else_expression]
                            if if_keyword == symbols::keywords::IF
                                && then_keyword == symbols::keywords::THEN
                                && else_keyword == symbols::keywords::ELSE =>
                        {
                            let condition_result = parse_node_recursive(condition_expression);
                            let then_result = parse_node_recursive(then_expression);
                            let else_result = parse_node_recursive(else_expression);

                            match (condition_result, then_result, else_result)
                            {
                                (Ok(condition_node), Ok(then_node), Ok(else_node)) =>
                                {
                                    let node = Node::from(ConditionalNodeData::new(
                                        condition_node,
                                        then_node,
                                        else_node,
                                    ));

                                    return Ok(node);
                                }
                                (condition_result, then_result, else_result) =>
                                {
                                    if let Err(mut new_errors) = condition_result
                                    {
                                        errors.append(&mut new_errors);
                                    }
                                    if let Err(mut new_errors) = then_result
                                    {
                                        errors.append(&mut new_errors);
                                    }
                                    if let Err(mut new_errors) = else_result
                                    {
                                        errors.append(&mut new_errors);
                                    }

                                    return Err(errors);
                                }
                            }
                        }
                        // (if condition then then_branch)
                        [Symbol(if_keyword), condition_expression, Symbol(then_keyword), then_expression]
                            if if_keyword == symbols::keywords::IF
                                && then_keyword == symbols::keywords::THEN =>
                        {
                            let condition_result = parse_node_recursive(condition_expression);
                            let then_result = parse_node_recursive(then_expression);

                            match (condition_result, then_result)
                            {
                                (Ok(condition_node), Ok(then_node)) =>
                                {
                                    let node = Node::from(ConditionalNodeData::new(
                                        condition_node,
                                        then_node,
                                        Node::Nothing,
                                    ));

                                    return Ok(node);
                                }
                                (condition_result, then_result) =>
                                {
                                    if let Err(mut new_errors) = condition_result
                                    {
                                        errors.append(&mut new_errors);
                                    }
                                    if let Err(mut new_errors) = then_result
                                    {
                                        errors.append(&mut new_errors);
                                    }
                                    return Err(errors);
                                }
                            }
                        }
                        // (when {...} else else_branch)
                        [Symbol(when_keyword), List(BracketType::Curly, when_expressions), Symbol(else_keyword), else_expression]
                            if when_keyword == symbols::keywords::WHEN
                                && else_keyword == symbols::keywords::ELSE =>
                        {
                            let mut branch_nodes: Vec<(Node, Node)> = Vec::new();
                            for when_expression in when_expressions.iter()
                            {
                                match parse_when_branch(when_expression)
                                {
                                    Ok(nodes) => branch_nodes.push(nodes),
                                    Err(mut new_errors) => errors.append(&mut new_errors),
                                }
                            }

                            match parse_node_recursive(else_expression)
                            {
                                Ok(else_node) =>
                                {
                                    // Make sure none of the branches produced an error
                                    if errors.is_empty()
                                    {
                                        if branch_nodes.is_empty()
                                        {
                                            // A when with no branch nodes just executes the else
                                            return Ok(else_node);
                                        }
                                        else
                                        {
                                            // We want the else branch to happen after the *last* condition branch
                                            branch_nodes.reverse();

                                            // Build up the final node
                                            let mut node = else_node;
                                            for (condition, result) in branch_nodes
                                            {
                                                node = Node::from(ConditionalNodeData::new(
                                                    condition, result, node,
                                                ));
                                            }

                                            return Ok(node);
                                        }
                                    }
                                }
                                Err(mut new_errors) =>
                                {
                                    errors.append(&mut new_errors);
                                }
                            }

                            return Err(errors);
                        }
                        // (when {...})
                        [Symbol(when_keyword), List(BracketType::Curly, when_expressions)]
                            if when_keyword == symbols::keywords::WHEN =>
                        {
                            let mut branch_nodes: Vec<(Node, Node)> = Vec::new();
                            for when_expression in when_expressions.iter()
                            {
                                match parse_when_branch(when_expression)
                                {
                                    Ok(nodes) => branch_nodes.push(nodes),
                                    Err(mut new_errors) => errors.append(&mut new_errors),
                                }
                            }

                            // Make sure none of the branches produced an error
                            if errors.is_empty()
                            {
                                if branch_nodes.is_empty()
                                {
                                    // A when with no branch nodes does nothing (equivalent to an empty sequence)
                                    return Ok(Node::from(SequenceNodeData::new(vec![])));
                                }
                                else
                                {
                                    // We want successive branches to be checked *after* the previous branch
                                    branch_nodes.reverse();
                                    
                                    // Build up the final node
                                    let mut node = Node::Nothing;
                                    for (condition, result) in branch_nodes
                                    {
                                        node = Node::from(ConditionalNodeData::new(
                                            condition, result, node,
                                        ));
                                    }

                                    return Ok(node);
                                }
                            }

                            return Err(errors);
                        }

                        // (fn name {body})
                        [Symbol(function_keyword), Symbol(name), body_expression]
                            if function_keyword == symbols::keywords::FUNCTION =>
                        {
                            if let List(BracketType::Curly, _) = body_expression
                            {
                                let body_result = parse_node_recursive(body_expression);
                                match body_result
                                {
                                    Ok(body_node) =>
                                    {
                                        let node = Node::from(FunctionNodeData::new(
                                            name.clone(),
                                            Vec::new(),
                                            Type::new(DataType::Void),
                                            body_node,
                                        ));

                                        return Ok(node);
                                    }
                                    Err(mut new_errors) =>
                                    {
                                        errors.append(&mut new_errors);
                                        return Err(errors);
                                    }
                                }
                            }
                            else
                            {
                                errors.push(ParseError::InvalidFunctionBody(
                                    name.clone(),
                                    "Expected {...} list".to_owned(),
                                ));
                                return Err(errors);
                            }
                        }
                        // (fn name <arguments> {body})
                        [Symbol(function_keyword), Symbol(name), List(BracketType::None, argument_expressions), body_expression]
                            if function_keyword == symbols::keywords::FUNCTION =>
                        {
                            if let List(BracketType::Curly, _) = body_expression
                            {
                                let body_result = parse_node_recursive(body_expression);
                                match body_result
                                {
                                    Ok(body_node) => match parse_arguments(argument_expressions)
                                    {
                                        Ok(arguments) =>
                                        {
                                            let node = Node::from(FunctionNodeData::new(
                                                name.clone(),
                                                arguments,
                                                Type::new(DataType::Void),
                                                body_node,
                                            ));
                                            return Ok(node);
                                        }
                                        Err(mut new_errors) =>
                                        {
                                            errors.append(&mut new_errors);
                                            return Err(errors);
                                        }
                                    },
                                    Err(mut new_errors) =>
                                    {
                                        errors.append(&mut new_errors);
                                        return Err(errors);
                                    }
                                }
                            }
                            else
                            {
                                errors.push(ParseError::InvalidFunctionBody(
                                    name.clone(),
                                    "Expected {...} list".to_owned(),
                                ));
                                return Err(errors);
                            }
                        }
                        // (fn name -> T {body})
                        [Symbol(function_keyword), Symbol(name), Symbol(arrow_keyword), return_type_expression, body_expression]
                            if function_keyword == symbols::keywords::FUNCTION
                                && arrow_keyword == symbols::keywords::ARROW =>
                        {
                            if let List(BracketType::Curly, _) = body_expression
                            {
                                let body_result = parse_node_recursive(body_expression);
                                match body_result
                                {
                                    Ok(body_node) => match parse_type(return_type_expression)
                                    {
                                        Ok(return_type) =>
                                        {
                                            let node = Node::from(FunctionNodeData::new(
                                                name.clone(),
                                                Vec::new(),
                                                return_type,
                                                body_node,
                                            ));
                                            return Ok(node);
                                        }
                                        Err(mut new_errors) =>
                                        {
                                            errors.append(&mut new_errors);
                                            return Err(errors);
                                        }
                                    },
                                    Err(mut new_errors) =>
                                    {
                                        errors.append(&mut new_errors);
                                        return Err(errors);
                                    }
                                }
                            }
                            else
                            {
                                errors.push(ParseError::InvalidFunctionBody(
                                    name.clone(),
                                    "Expected {...} list".to_owned(),
                                ));
                                return Err(errors);
                            }
                        }
                        // (fn name -> T <arguments> {body})
                        [Symbol(function_keyword), Symbol(name), List(BracketType::None, argument_expressions), Symbol(arrow_keyword), return_type_expression, body_expression]
                            if function_keyword == symbols::keywords::FUNCTION
                                && arrow_keyword == symbols::keywords::ARROW =>
                        {
                            if let List(BracketType::Curly, _) = body_expression
                            {
                                let body_result = parse_node_recursive(body_expression);
                                match body_result
                                {
                                    Ok(body_node) => match parse_arguments(argument_expressions)
                                    {
                                        Ok(arguments) => match parse_type(return_type_expression)
                                        {
                                            Ok(return_type) =>
                                            {
                                                let node = Node::from(FunctionNodeData::new(
                                                    name.clone(),
                                                    arguments,
                                                    return_type,
                                                    body_node,
                                                ));
                                                return Ok(node);
                                            }
                                            Err(mut new_errors) =>
                                            {
                                                errors.append(&mut new_errors);
                                                return Err(errors);
                                            }
                                        },
                                        Err(mut new_errors) =>
                                        {
                                            errors.append(&mut new_errors);
                                            return Err(errors);
                                        }
                                    },
                                    Err(mut new_errors) =>
                                    {
                                        errors.append(&mut new_errors);
                                        return Err(errors);
                                    }
                                }
                            }
                            else
                            {
                                errors.push(ParseError::InvalidFunctionBody(
                                    name.clone(),
                                    "Expected {...} list".to_owned(),
                                ));
                                return Err(errors);
                            }
                        }

                        _ =>
                        {
                            // Handle the general case of operators with an arbitrary number of operands
                            if elements.len() > 0
                            {
                                let operator_result = parse_node_recursive(&elements[0]);
                                let mut operands: Vec<Node> = Vec::new();
                                for i in 1..elements.len()
                                {
                                    match parse_node_recursive(&elements[i])
                                    {
                                        Ok(node) => operands.push(node),
                                        Err(mut new_errors) => errors.append(&mut new_errors),
                                    }
                                }
                                match operator_result
                                {
                                    Ok(node) =>
                                    {
                                        if errors.len() == 0
                                        {
                                            let node =
                                                Node::from(CallNodeData::new(node, operands));
                                            return Ok(node);
                                        }
                                        else
                                        {
                                            return Err(errors);
                                        }
                                    }
                                    Err(mut new_errors) =>
                                    {
                                        errors.append(&mut new_errors);
                                        return Err(errors);
                                    }
                                }
                            }
                            else
                            {
                                errors.push(ParseError::InvalidSExpression(expression.clone()));
                                return Err(errors);
                            }
                        }
                    }
                }

                BracketType::Curly =>
                {
                    // Handle sequence nodes
                    let mut nodes: Vec<Node> = Vec::new();
                    for element in elements.iter()
                    {
                        match parse_node_recursive(element)
                        {
                            Ok(node) => nodes.push(node),
                            Err(mut new_errors) => errors.append(&mut new_errors),
                        }
                    }

                    if errors.len() == 0
                    {
                        let node = Node::from(SequenceNodeData::new(nodes));
                        return Ok(node);
                    }
                    else
                    {
                        return Err(errors);
                    }
                }

                BracketType::Square =>
                {
                    // Any square-bracketed lists aren't valid
                    errors.push(ParseError::InvalidSExpression(expression.clone()));
                    return Err(errors);
                }
                BracketType::None =>
                {
                    // Any internally-generated lists aren't valid
                    errors.push(ParseError::InvalidSExpression(expression.clone()));
                    return Err(errors);
                }
            }
        }
    }
}

fn parse_integer(symbol: &String) -> Option<Node>
{
    let result = symbol.parse::<i64>();
    if let Ok(value) = result
    {
        let node = Node::from(IntegerNodeData::new(value));
        return Some(node);
    }

    return None;
}
fn parse_boolean(symbol: &String) -> Option<Node>
{
    match symbol.as_str()
    {
        symbols::constants::TRUE => Some(Node::from(BooleanNodeData::new(true))),
        symbols::constants::FALSE => Some(Node::from(BooleanNodeData::new(false))),
        _ => None,
    }
}
fn parse_primitive_operator(symbol: &String) -> Option<Node>
{
    match symbol.as_str()
    {
        // Arithmetic operators
        symbols::operators::PLUS => Some(Node::from(PrimitiveOperatorNodeData::new(
            PrimitiveOperator::Add,
        ))),

        // Comparison operators
        symbols::operators::EQUAL => Some(Node::from(PrimitiveOperatorNodeData::new(
            PrimitiveOperator::Equal,
        ))),
        symbols::operators::NOT_EQUAL => Some(Node::from(PrimitiveOperatorNodeData::new(
            PrimitiveOperator::NotEqual,
        ))),
        symbols::operators::LESS => Some(Node::from(PrimitiveOperatorNodeData::new(
            PrimitiveOperator::Less,
        ))),
        symbols::operators::GREATER => Some(Node::from(PrimitiveOperatorNodeData::new(
            PrimitiveOperator::Greater,
        ))),
        symbols::operators::LESS_EQUAL => Some(Node::from(PrimitiveOperatorNodeData::new(
            PrimitiveOperator::LessEqual,
        ))),
        symbols::operators::GREATER_EQUAL => Some(Node::from(PrimitiveOperatorNodeData::new(
            PrimitiveOperator::GreaterEqual,
        ))),
        // Logical operators
        symbols::operators::AND => Some(Node::from(PrimitiveOperatorNodeData::new(
            PrimitiveOperator::And,
        ))),
        symbols::operators::OR => Some(Node::from(PrimitiveOperatorNodeData::new(
            PrimitiveOperator::Or,
        ))),
        symbols::operators::XOR => Some(Node::from(PrimitiveOperatorNodeData::new(
            PrimitiveOperator::ExclusiveOr,
        ))),

        _ => None,
    }
}
fn parse_type(expression: &SExpression) -> Result<Type, ParseErrorList>
{
    match expression
    {
        SExpression::Symbol(symbol) => match symbol.as_str()
        {
            symbols::primitive_data_types::INTEGER => Ok(Type::new(DataType::Integer)),
            symbols::primitive_data_types::BOOLEAN => Ok(Type::new(DataType::Boolean)),
            symbols::primitive_data_types::VOID => Ok(Type::new(DataType::Void)),
            _ =>
            {
                let errors = vec![ParseError::InvalidType(expression.clone())];
                return Err(errors);
            }
        },
        SExpression::List(_bracket_type, _elements) =>
        {
            let errors = vec![ParseError::InvalidType(expression.clone())];
            return Err(errors);
        }
    }
}
fn parse_when_branch(expression: &SExpression) -> Result<(Node, Node), ParseErrorList>
{
    let mut errors = ParseErrorList::new();

    match expression
    {
        SExpression::List(BracketType::None, elements) => match elements.as_slice()
        {
            [condition_expression, SExpression::Symbol(associate_keyword), result_expression]
                if associate_keyword == symbols::keywords::ASSOCIATE =>
            {
                match (
                    parse_node_recursive(condition_expression),
                    parse_node_recursive(result_expression),
                )
                {
                    (Ok(condition_node), Ok(result_node)) =>
                    {
                        return Ok((condition_node, result_node));
                    }
                    (condition_result, result_result) =>
                    {
                        if let Err(mut new_errors) = condition_result
                        {
                            errors.append(&mut new_errors);
                        }
                        if let Err(mut new_errors) = result_result
                        {
                            errors.append(&mut new_errors);
                        }
                    }
                }
            }
            _ =>
            {
                errors.push(ParseError::InvalidWhenBranch(expression.clone()));
            }
        },
        _ =>
        {
            errors.push(ParseError::InvalidWhenBranch(expression.clone()));
        }
    }

    return Err(errors);
}

fn parse_arguments(expressions: &Vec<SExpression>) -> Result<Vec<ArgumentData>, ParseErrorList>
{
    let mut errors = ParseErrorList::new();
    let mut arguments: Vec<ArgumentData> = Vec::new();

    for expression in expressions.iter()
    {
        match expression
        {
            SExpression::List(BracketType::Square, elements) => match elements.as_slice()
            {
                [SExpression::Symbol(name), type_expression] => match parse_type(type_expression)
                {
                    Ok(argument_type) =>
                    {
                        arguments.push(ArgumentData::new(name.clone(), argument_type));
                    }
                    Err(mut new_errors) =>
                    {
                        errors.append(&mut new_errors);
                    }
                },
                _ =>
                {
                    errors.push(ParseError::InvalidFunctionArgument(expression.clone()));
                }
            },
            _ =>
            {
                errors.push(ParseError::InvalidFunctionArgument(expression.clone()));
            }
        }
    }

    if errors.is_empty()
    {
        return Ok(arguments);
    }
    else
    {
        return Err(errors);
    }
}
