pub mod errors;
pub use errors::*;

pub mod s_expression;
use s_expression::*;

mod parse_s_expression;
mod preprocessor;

use crate::language::nodes::*;
use crate::language::symbols;

pub enum ParseResult
{
    Ok(Node),
    Error(ParseErrorList),
}

pub struct Parser {}
impl Parser
{
    // Parse a source string into an S-Expression
    pub fn parse_source(&self, source: &String) -> Option<SExpression>
    {
        return parse_s_expression::parse_expression(source);
    }

    // Preprocess an S-Expression for parsing
    pub fn preprocess(&self, source: &mut SExpression)
    {
        preprocessor::make_operator_groups::apply(source);
    }

    // Parse a source S-Expression into a ParseResult (node or errors)
    pub fn parse(&self, source: &SExpression) -> ParseResult
    {
        let mut nodes: Vec<Node> = Vec::new();
        let mut errors = ParseErrorList::new();

        // All programs are wrapped in a top-level list (not user-defined)
        if let SExpression::List(BracketType::None, elements) = source
        {
            // Accumulate nodes and/or errors
            for element in elements.iter()
            {
                match parse_node_recursive(element)
                {
                    ParseResult::Ok(node) => nodes.push(node),
                    ParseResult::Error(mut new_errors) => errors.append(&mut new_errors),
                }
            }

            if errors.len() == 0
            {
                return ParseResult::Ok(SequenceNodeData::new(nodes).to_node());
            }
            else
            {
                return ParseResult::Error(errors);
            }
        }
        else
        {
            errors.push(ParseError::Internal(String::from(
                "Invalid top-level SExpression",
            )));
            return ParseResult::Error(errors);
        }
    }

    pub fn new() -> Self
    {
        return Self {};
    }
}

fn parse_node_recursive(expression: &SExpression) -> ParseResult
{
    use ParseResult::*;

    let mut errors = ParseErrorList::new();

    match expression
    {
        SExpression::Symbol(symbol) =>
        {
            if let Some(node) = parse_integer(symbol)
            {
                return Ok(node);
            }
            else if let Some(node) = parse_primitive_operator(symbol)
            {
                return Ok(node);
            }
            else
            {
                let node = VariableNodeData::new(symbol.clone()).to_node();
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
                        // Handle special case with infix binary operators
                        [a, SExpression::Symbol(op), b] if symbols::operators::is_binary(op) =>
                        {
                            if let Some(operator_node) = parse_primitive_operator(op)
                            {
                                let a_result = parse_node_recursive(a);
                                let b_result = parse_node_recursive(b);
                                match (a_result, b_result)
                                {
                                    (Ok(a_node), Ok(b_node)) =>
                                    {
                                        let node =
                                            CallNodeData::new(operator_node, vec![a_node, b_node])
                                                .to_node();
                                        return Ok(node);
                                    }
                                    (a_result, b_result) =>
                                    {
                                        if let Error(mut new_errors) = a_result
                                        {
                                            errors.append(&mut new_errors);
                                        }
                                        if let Error(mut new_errors) = b_result
                                        {
                                            errors.append(&mut new_errors);
                                        }
                                        return Error(errors);
                                    }
                                }
                            }
                            else
                            {
                                errors.push(ParseError::Internal(format!(
                                    "Unhandled primitive binary operator {}",
                                    op
                                )));
                                return Error(errors);
                            }
                        }
                        [SExpression::Symbol(binding_keyword), SExpression::Symbol(binding_name), SExpression::Symbol(binding_operator), binding_expression]
                            if binding_keyword == symbols::keywords::BINDING
                                && binding_operator == symbols::operators::ASSIGN =>
                        {
                            // (let name = expression)
                            match parse_node_recursive(binding_expression)
                            {
                                Ok(binding_node) =>
                                {
                                    let node =
                                        BindingNodeData::new(binding_name.clone(), binding_node)
                                            .to_node();
                                    return Ok(node);
                                }
                                Error(mut new_errors) =>
                                {
                                    errors.append(&mut new_errors);
                                    return Error(errors);
                                }
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
                                        Error(mut new_errors) => errors.append(&mut new_errors),
                                    }
                                }
                                match operator_result
                                {
                                    Ok(node) =>
                                    {
                                        if errors.len() == 0
                                        {
                                            let node = CallNodeData::new(node, operands).to_node();
                                            return Ok(node);
                                        }
                                        else
                                        {
                                            return Error(errors);
                                        }
                                    }
                                    Error(mut new_errors) =>
                                    {
                                        errors.append(&mut new_errors);
                                        return Error(errors);
                                    }
                                }
                            }
                            else
                            {
                                errors.push(ParseError::InvalidSExpression(expression.clone()));
                                return Error(errors);
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
                            Error(mut new_errors) => errors.append(&mut new_errors),
                        }
                    }

                    if errors.len() == 0
                    {
                        let node = SequenceNodeData::new(nodes).to_node();
                        return Ok(node);
                    }
                    else
                    {
                        return Error(errors);
                    }
                }

                BracketType::Square =>
                {
                    errors.push(ParseError::InvalidSExpression(expression.clone()));
                    return Error(errors);
                }
                BracketType::None =>
                {
                    errors.push(ParseError::InvalidSExpression(expression.clone()));
                    return Error(errors);
                }
            }
        }
        _ =>
        {
            errors.push(ParseError::InvalidSExpression(expression.clone()));
            return Error(errors);
        }
    }
}

fn parse_integer(symbol: &String) -> Option<Node>
{
    let result = symbol.parse::<i64>();
    if let Ok(value) = result
    {
        return Some(IntegerNodeData::new(value).to_node());
    }

    return None;
}
fn parse_primitive_operator(symbol: &String) -> Option<Node>
{
    match symbol.as_str()
    {
        symbols::operators::PLUS =>
        {
            Some(PrimitiveOperatorNodeData::new(PrimitiveOperator::Add).to_node())
        }
        _ => None,
    }
}
