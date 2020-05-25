pub mod s_expression;
use s_expression::*;

mod parse_s_expression;
mod preprocessor;
mod parse;

use crate::language::nodes::Node;

pub struct Parser {}
impl Parser
{
    pub fn new() -> Self
    {
        return Self {};
    }

    pub fn parse_source(&self, source: &String) -> Option<SExpression>
    {
        return parse_s_expression::parse_expression(source);
    }

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

    pub fn parse_expression(&self, source: &SExpression) -> ParseResult
    {
        let mut context = parse::Context::new();

        let result = parse::parse_root_expression(source, &mut context);
        let (errors, warnings) = context.get_messages();

        match result
        {
            Some(node) => ParseResult::Success(node, warnings),
            None => ParseResult::Error(errors, warnings),
        }
    }
}

pub enum ParseResult
{
    Success(Node, Vec<parse::Message>),
    Error(Vec<parse::Message>, Vec<parse::Message>),
}