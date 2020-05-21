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
                        // (a . b)
                        [a, Symbol(op), Symbol(property_name)]
                            if op == symbols::operators::ACCESS =>
                        {
                            match parse_node_recursive(a)
                            {
                                Ok(target_node) =>
                                {
                                    let node = Node::from(AccessNodeData::new(
                                        target_node,
                                        property_name.clone(),
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
                        // (deref a)
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
                        // (~ a)
                        [Symbol(op), a] if symbols::operators::is_unary(op) =>
                        {
                            if let Some(operator_node) = parse_primitive_operator(op)
                            {
                                let a_result = parse_node_recursive(a);
                                match a_result
                                {
                                    Ok(a_node) =>
                                    {
                                        let node = Node::from(CallNodeData::new(
                                            operator_node,
                                            vec![a_node],
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
                                errors.push(ParseError::Internal(format!(
                                    "Unhandled primitive unary operator {}",
                                    op
                                )));
                                return Err(errors);
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
                        // (fn name <arguments> {body})
                        // (fn name -> T {body})
                        // (fn name -> T <arguments> {body})
                        [Symbol(function_keyword), _, _]
                        | [Symbol(function_keyword), _, _, _]
                        | [Symbol(function_keyword), _, _, _, _]
                        | [Symbol(function_keyword), _, _, _, _, _]
                            if function_keyword == symbols::keywords::FUNCTION =>
                        {
                            match parse_function(elements.as_slice(), FunctionType::Basic)
                            {
                                Ok(result) =>
                                {
                                    return Ok(Node::from(result.function_data));
                                }
                                Err(mut new_errors) =>
                                {
                                    errors.append(&mut new_errors);
                                    return Err(errors);
                                }
                            }
                        }

                        // (type name {body})
                        [Symbol(type_keyword), Symbol(name), List(BracketType::Curly, body_expressions)]
                            if type_keyword == symbols::keywords::TYPE =>
                        {
                            let mut type_data = TypeNodeData::new_empty(name.clone());
                            let mut new_errors = ParseErrorList::new();

                            let mut public_read_members: Vec<(String, MemberScope)> = Vec::new();
                            let mut public_write_members: Vec<(String, MemberScope)> = Vec::new();

                            for body_expression in body_expressions.iter()
                            {
                                match body_expression
                                {
                                    List(BracketType::Round, expressions) =>
                                    {
                                        match expressions.as_slice()
                                        {
                                            // is Name
                                            [Symbol(is_keyword), Symbol(name)]
                                                if is_keyword == symbols::keywords::IS =>
                                            {
                                                type_data.add_trait_name(name.clone());
                                            }
                                            // Unknown (...) list in type body
                                            _ =>
                                            {
                                                new_errors.push(ParseError::InvalidTypeExpression(
                                                    name.clone(),
                                                    "Expected `is ...`".to_owned(),
                                                    body_expression.clone(),
                                                ))
                                            }
                                        }
                                    }

                                    List(BracketType::None, expressions) =>
                                    {
                                        match expressions.as_slice()
                                        {
                                            // data {...}
                                            [Symbol(group_keyword), List(BracketType::Curly, data_expressions)]
                                                if group_keyword == symbols::keywords::DATA =>
                                            {
                                                match parse_type_members(data_expressions)
                                                {
                                                    Ok(mut new_members) =>
                                                    {
                                                        type_data
                                                            .get_members_mut()
                                                            .append(&mut new_members);
                                                    }
                                                    Err(mut new_errors) =>
                                                    {
                                                        errors.append(&mut new_errors);
                                                    }
                                                }
                                            }
                                            // public {...}
                                            [Symbol(group_keyword), List(BracketType::Curly, expressions)]
                                                if group_keyword == symbols::keywords::PUBLIC =>
                                            {
                                                match parse_type_methods(expressions)
                                                {
                                                    Ok(mut result) =>
                                                    {
                                                        // Mark methods as public
                                                        for method in result.methods.iter_mut()
                                                        {
                                                            method
                                                                .set_visibility(Visibility::Public);
                                                        }
                                                        // Track all publicly accessibly members
                                                        // note: could be marked accessible before being declared
                                                        //       so we can't update existing entries until later
                                                        public_read_members
                                                            .append(&mut result.read_members);
                                                        public_write_members
                                                            .append(&mut result.write_members);
                                                        // Add new methods
                                                        type_data
                                                            .get_methods_mut()
                                                            .append(&mut result.methods);
                                                    }
                                                    Err(mut new_errors) =>
                                                    {
                                                        errors.append(&mut new_errors);
                                                    }
                                                }
                                            }
                                            // private {...}
                                            [Symbol(group_keyword), List(BracketType::Curly, expressions)]
                                                if group_keyword == symbols::keywords::PRIVATE
                                                    && false =>
                                            {
                                                match parse_type_methods(expressions)
                                                {
                                                    Ok(mut result) =>
                                                    {
                                                        // Add new methods
                                                        // note: read/write members in private blocks are ignored
                                                        type_data
                                                            .get_methods_mut()
                                                            .append(&mut result.methods);
                                                    }
                                                    Err(mut new_errors) =>
                                                    {
                                                        errors.append(&mut new_errors);
                                                    }
                                                }
                                            }

                                            // Unknown <...> list in type body
                                            _ =>
                                            {
                                                new_errors.push(ParseError::InvalidTypeExpression(
                                                    name.clone(),
                                                    "Expected `data/private/public {...}`"
                                                        .to_owned(),
                                                    body_expression.clone(),
                                                ))
                                            }
                                        }
                                    }
                                    _ =>
                                    {
                                        // Some other list / symbol in type body
                                        new_errors.push(ParseError::InvalidTypeExpression(
                                            name.clone(),
                                            "Expected `data/private/public {...}`".to_owned(),
                                            body_expression.clone(),
                                        ));
                                    }
                                }
                            }

                            if new_errors.is_empty()
                            {
                                // Mark publicly readable members
                                for (name, scope) in public_read_members.iter()
                                {
                                    for member in type_data.get_members_mut().iter_mut()
                                    {
                                        if member.get_name() == name && member.get_scope() == *scope
                                        {
                                            member.set_read_visibility(Visibility::Public);
                                            break;
                                        }
                                    }
                                }

                                // Mark publicly writable members
                                for (name, scope) in public_write_members.iter()
                                {
                                    for member in type_data.get_members_mut().iter_mut()
                                    {
                                        if member.get_name() == name && member.get_scope() == *scope
                                        {
                                            member.set_write_visibility(Visibility::Public);
                                            break;
                                        }
                                    }
                                }

                                return Ok(Node::from(type_data));
                            }
                            else
                            {
                                errors.append(&mut new_errors);
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
    let operator = match symbol.as_str()
    {
        // Arithmetic operators
        symbols::operators::PLUS => Some(PrimitiveOperator::Add),

        // Comparison operators
        symbols::operators::EQUAL => Some(PrimitiveOperator::Equal),
        symbols::operators::NOT_EQUAL => Some(PrimitiveOperator::NotEqual),
        symbols::operators::LESS => Some(PrimitiveOperator::Less),
        symbols::operators::GREATER => Some(PrimitiveOperator::Greater),
        symbols::operators::LESS_EQUAL => Some(PrimitiveOperator::LessEqual),
        symbols::operators::GREATER_EQUAL => Some(PrimitiveOperator::GreaterEqual),

        // Logical operators
        symbols::operators::AND => Some(PrimitiveOperator::And),
        symbols::operators::OR => Some(PrimitiveOperator::Or),
        symbols::operators::XOR => Some(PrimitiveOperator::ExclusiveOr),

        // Memory operators
        symbols::operators::CREATE => Some(PrimitiveOperator::Create),

        _ => None,
    };

    match operator
    {
        Some(operator) => Some(Node::from(PrimitiveOperatorNodeData::new(operator))),
        None => None,
    }
}

fn parse_type(expression: &SExpression) -> Result<Type, ParseErrorList>
{
    match expression
    {
        SExpression::Symbol(symbol) => match symbol.as_str()
        {
            symbols::primitive_data_types::INTEGER => Ok(basic_types::integer().clone()),
            symbols::primitive_data_types::BOOLEAN => Ok(basic_types::boolean().clone()),
            symbols::primitive_data_types::VOID => Ok(basic_types::void().clone()),
            _ => Ok(Type::from(InstanceTypeData::new(symbol.clone()))),
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

struct ParseFunctionResult
{
    pub function_data: FunctionNodeData,
    pub owner_name:    Option<String>,
}
fn parse_function(
    expressions: &[SExpression],
    function_type: FunctionType,
) -> Result<ParseFunctionResult, ParseErrorList>
{
    use SExpression::*;

    let mut errors = ParseErrorList::new();

    // note: [fn ...] has already been matched
    let (name, owner_name) = match &expressions[1]
    {
        Symbol(name) => (name.clone(), None),
        List(BracketType::Round, name_elements) => match name_elements.as_slice()
        {
            [Symbol(owner_name), Symbol(access_operator), Symbol(name)]
                if access_operator == symbols::operators::ACCESS =>
            {
                (name.clone(), Some(owner_name.clone()))
            }
            _ =>
            {
                errors.push(ParseError::InvalidFunctionName(format!(
                    "{}",
                    &expressions[1]
                )));
                (String::new(), None)
            }
        },
        _ =>
        {
            errors.push(ParseError::InvalidFunctionName(format!(
                "{}",
                &expressions[1]
            )));
            (String::new(), None)
        }
    };
    // note: [fn Name ...] has already been matched
    let (argument_expressions, return_type_expression, body_expression) = match &expressions[2..]
    {
        // fn Name {...}
        [body_expression] => (None, None, Some(body_expression)),
        // fn Name <Arguments> {...}
        [List(BracketType::None, argument_expressions), body_expression] =>
        {
            (Some(argument_expressions), None, Some(body_expression))
        }
        // fn Name -> ReturnType {...}
        [Symbol(arrow_keyword), return_type_expression, body_expression]
            if arrow_keyword == symbols::keywords::ARROW =>
        {
            (None, Some(return_type_expression), Some(body_expression))
        }
        // fn Name <Arguments> -> ReturnType {...}
        [List(BracketType::None, argument_expressions), Symbol(arrow_keyword), return_type_expression, body_expression]
            if arrow_keyword == symbols::keywords::ARROW =>
        {
            (
                Some(argument_expressions),
                Some(return_type_expression),
                Some(body_expression),
            )
        }

        _ =>
        {
            errors.push(ParseError::InvalidFunctionBody(
                format!("{}", &expressions[1]),
                "Failed to parse structure".to_owned(),
            ));
            (None, None, None)
        }
    };

    let body_node = match &body_expression
    {
        None =>
        {
            return Err(errors);
        }
        Some(List(BracketType::Curly, _)) => match parse_node_recursive(&body_expression.unwrap())
        {
            Ok(body_node) => body_node,
            Err(mut new_errors) =>
            {
                errors.append(&mut new_errors);
                return Err(errors);
            }
        },
        _ =>
        {
            errors.push(ParseError::InvalidFunctionBody(
                format!("{}", &expressions[1]),
                "Expected {...} list".to_owned(),
            ));
            return Err(errors);
        }
    };

    let arguments = match argument_expressions
    {
        Some(argument_expressions) => match parse_arguments(argument_expressions)
        {
            Ok(arguments) => arguments,
            Err(mut new_errors) =>
            {
                errors.append(&mut new_errors);
                return Err(errors);
            }
        },
        None => Vec::new(),
    };

    let return_type = match return_type_expression
    {
        None => basic_types::void().clone(),
        Some(expression) => match parse_type(expression)
        {
            Ok(return_type) => return_type,
            Err(mut new_errors) =>
            {
                errors.append(&mut new_errors);
                return Err(errors);
            }
        },
    };

    // Make sure there weren't any errors in previous steps
    if errors.is_empty()
    {
        let final_function_type = match (function_type, &owner_name)
        {
            (FunctionType::StaticMethod, Some(name)) if name == symbols::keywords::SELF =>
            {
                FunctionType::InstanceMethod
            }
            _ => function_type,
        };

        let function_data = FunctionNodeData::new(
            name,
            arguments,
            return_type,
            body_node,
            FunctionMetadata::new(final_function_type),
        );
        return Ok(ParseFunctionResult {
            function_data: function_data,
            owner_name:    owner_name,
        });
    }
    else
    {
        return Err(errors);
    }
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

fn parse_scoped_name(name_expression: &SExpression) -> Result<(String, MemberScope), ()>
{
    match name_expression
    {
        SExpression::List(BracketType::Round, name_elements) => match name_elements.as_slice()
        {
            // Name is (self . Symbol)
            [SExpression::Symbol(self_keyword), SExpression::Symbol(access_keyword), SExpression::Symbol(name)]
                if self_keyword == symbols::keywords::SELF
                    && access_keyword == symbols::operators::ACCESS =>
            {
                return Ok((name.clone(), MemberScope::Instance));
            }

            // Name isn't (self . Name)
            _ =>
            {
                return Err(());
            }
        },
        // Name is symbol
        SExpression::Symbol(name) =>
        {
            return Ok((name.clone(), MemberScope::Static));
        }
        // Name isn't (...)
        _ =>
        {
            return Err(());
        }
    }
}

fn parse_type_members(expressions: &Vec<SExpression>) -> Result<Vec<MemberData>, ParseErrorList>
{
    let mut errors = ParseErrorList::new();
    let mut members: Vec<MemberData> = Vec::new();

    for expression in expressions.iter()
    {
        match expression
        {
            // Is [...]
            SExpression::List(BracketType::Square, elements) => match elements.as_slice()
            {
                // Is [Name Type]
                [name_expression, type_expression] => match parse_type(type_expression)
                {
                    Ok(member_type) => match parse_scoped_name(name_expression)
                    {
                        Ok((name, scope)) =>
                        {
                            members.push(MemberData::new(
                                name,
                                member_type,
                                Visibility::Private,
                                Visibility::Private,
                                scope,
                            ));
                        }
                        Err(_) =>
                        {
                            errors.push(ParseError::InvalidMemberExpression(expression.clone()));
                        }
                    },
                    Err(mut new_errors) =>
                    {
                        errors.append(&mut new_errors);
                    }
                },
                _ =>
                // Not [Name Type]
                {
                    errors.push(ParseError::InvalidMemberExpression(expression.clone()));
                }
            },
            // Not [...]
            _ =>
            {
                errors.push(ParseError::InvalidMemberExpression(expression.clone()));
            }
        }
    }

    if errors.is_empty()
    {
        return Ok(members);
    }
    else
    {
        return Err(errors);
    }
}

struct ParseMethodsResult
{
    pub methods:       Vec<MethodData>,
    pub read_members:  Vec<(String, MemberScope)>,
    pub write_members: Vec<(String, MemberScope)>,
}
fn parse_type_methods(expressions: &Vec<SExpression>)
    -> Result<ParseMethodsResult, ParseErrorList>
{
    let mut errors = ParseErrorList::new();
    let mut result = ParseMethodsResult {
        methods:       Vec::new(),
        read_members:  Vec::new(),
        write_members: Vec::new(),
    };

    for expression in expressions.iter()
    {
        match expression
        {
            SExpression::List(BracketType::Round, elements) => match elements.as_slice()
            {
                // (read ...)
                [SExpression::Symbol(read_keyword), name_expression]
                    if read_keyword == symbols::keywords::READ =>
                {
                    match parse_scoped_name(name_expression)
                    {
                        Ok(name) => result.read_members.push(name),
                        Err(_) =>
                        {
                            errors.push(ParseError::InvalidMemberExpression(expression.clone()));
                        }
                    }
                }
                // (write ...)
                [SExpression::Symbol(write_keyword), name_expression]
                    if write_keyword == symbols::keywords::WRITE =>
                {
                    match parse_scoped_name(name_expression)
                    {
                        Ok(name) => result.read_members.push(name),
                        Err(_) =>
                        {
                            errors.push(ParseError::InvalidMemberExpression(expression.clone()));
                        }
                    }
                }
                // (...) maybe a function?
                _ =>
                {
                    if let Some(SExpression::Symbol(first_symbol)) = elements.first()
                    {
                        if first_symbol == symbols::keywords::FUNCTION
                        {
                            match parse_function(elements, FunctionType::StaticMethod)
                            {
                                Ok(function_result) => match &function_result.owner_name
                                {
                                    Some(name) if name == symbols::keywords::SELF =>
                                    {
                                        result.methods.push(MethodData::new(
                                            function_result.function_data,
                                            Visibility::Private,
                                            MemberScope::Instance,
                                        ));
                                    }
                                    None =>
                                    {
                                        result.methods.push(MethodData::new(
                                            function_result.function_data,
                                            Visibility::Private,
                                            MemberScope::Static,
                                        ));
                                    }
                                    _ =>
                                    {
                                        errors.push(ParseError::InvalidMemberExpression(
                                            expression.clone(),
                                        ));
                                    }
                                },
                                Err(mut new_errors) =>
                                {
                                    errors.append(&mut new_errors);
                                }
                            }
                        }
                        else
                        {
                            errors.push(ParseError::InvalidMemberExpression(expression.clone()));
                        }
                    }
                    else
                    {
                        errors.push(ParseError::InvalidMemberExpression(expression.clone()));
                    }
                }
            },
            _ =>
            {
                errors.push(ParseError::InvalidMemberExpression(expression.clone()));
            }
        }
    }

    if errors.is_empty()
    {
        return Ok(result);
    }
    else
    {
        return Err(errors);
    }
}
