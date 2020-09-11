mod parse_s_expression;

mod preprocess;

mod parse_node;
mod parse_atomic;
mod parse_function;

use crate::language::s_expression::*;

pub struct Parser {}
impl Parser
{
    pub fn new() -> Parser
    {
        return Parser {};
    }

    pub fn preprocess(&self, source: &mut SExpression)
    {
        preprocess::make_associative_groups::apply(source);
        preprocess::make_function_groups::apply(source);
        preprocess::make_type_groups::apply(source);

        preprocess::make_operator_groups::apply(source);
        preprocess::make_conditional_groups::apply(source);
        preprocess::make_when_groups::apply(source);
        preprocess::make_assign_groups::apply(source);

        preprocess::expand_operator_chains::apply(source);
    }

    // fn make_s_expression
    // fn make_node
}
