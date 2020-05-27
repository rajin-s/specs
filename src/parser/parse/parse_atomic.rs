use crate::language::node::{atomic, primitive};
use crate::language::symbols::*;

use super::Context;

pub fn integer(symbol: &String, _context: &mut Context) -> Option<atomic::Integer>
{
    match symbol.parse::<i64>()
    {
        Ok(value) => Some(atomic::Integer::new(value)),
        Err(_) => None,
    }
}

pub fn boolean(symbol: &String, _context: &mut Context) -> Option<atomic::Boolean>
{
    match symbol.as_str()
    {
        constants::TRUE => Some(atomic::Boolean::new(true)),
        constants::FALSE => Some(atomic::Boolean::new(false)),
        _ => None,
    }
}

pub fn primitive_operator(
    symbol: &String,
    _context: &mut Context,
) -> Option<atomic::PrimitiveOperator>
{
    let operator = match symbol.as_str()
    {
        operators::PLUS => primitive::Operator::Add,
        operators::MINUS => primitive::Operator::Subtract,
        operators::TIMES => primitive::Operator::Multiply,
        operators::DIVIDE => primitive::Operator::Divide,
        operators::MODULO => primitive::Operator::Modulo,

        operators::EQUAL => primitive::Operator::Equal,
        operators::NOT_EQUAL => primitive::Operator::NotEqual,
        operators::LESS => primitive::Operator::Less,
        operators::GREATER => primitive::Operator::Greater,
        operators::LESS_EQUAL => primitive::Operator::LessEqual,

        operators::AND => primitive::Operator::And,
        operators::OR => primitive::Operator::Or,
        operators::XOR => primitive::Operator::ExclusiveOr,

        operators::CREATE => primitive::Operator::Create,

        _ =>
        {
            return None;
        }
    };

    Some(atomic::PrimitiveOperator::new(operator))
}

pub fn variable(symbol: &String, _context: &mut Context) -> Option<atomic::Variable>
{
    return Some(atomic::Variable::new(symbol.clone()));
}
