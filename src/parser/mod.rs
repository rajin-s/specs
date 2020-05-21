pub mod errors;
pub use errors::*;

pub mod s_expression;
use s_expression::*;

mod parse_s_expression;
mod preprocessor;
mod parse_node;

use crate::language::nodes::*;

pub type ParseResult = Result<Node, ParseErrorList>;

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
        preprocessor::make_associative_groups::apply(source);
        preprocessor::make_function_groups::apply(source);
        preprocessor::make_type_groups::apply(source);

        preprocessor::make_operator_groups::apply(source);
        preprocessor::make_conditional_groups::apply(source);
        preprocessor::make_when_groups::apply(source);
        preprocessor::make_assign_groups::apply(source);

        preprocessor::expand_operator_chains::apply(source);
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
                match parse_node::parse_node_recursive(element)
                {
                    Ok(node) => nodes.push(node),
                    Err(mut new_errors) => errors.append(&mut new_errors),
                }
            }

            if errors.len() == 0
            {
                return Ok(SequenceNodeData::new(nodes).to_node());
            }
            else
            {
                return Err(errors);
            }
        }
        else
        {
            errors.push(ParseError::Internal(String::from(
                "Invalid top-level SExpression",
            )));
            return Err(errors);
        }
    }

    pub fn new() -> Self
    {
        return Self {};
    }
}