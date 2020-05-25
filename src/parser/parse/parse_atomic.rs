use super::internal::*;
use imports::*;

pub fn integer(symbol: &String, _context: &mut Context) -> Option<IntegerNodeData>
{
    match symbol.parse::<i64>()
    {
        Ok(value) => Some(IntegerNodeData::new(value)),
        Err(_) => None,
    }
}

pub fn boolean(symbol: &String, _context: &mut Context) -> Option<BooleanNodeData>
{
    match symbol.as_str()
    {
        constants::TRUE => Some(BooleanNodeData::new(true)),
        constants::FALSE => Some(BooleanNodeData::new(false)),
        _ => None,
    }
}

pub fn primitive_operator(
    symbol: &String,
    _context: &mut Context,
) -> Option<PrimitiveOperatorNodeData>
{
    let operator = match symbol.as_str()
    {
        operators::PLUS => PrimitiveOperator::Add,

        operators::EQUAL => PrimitiveOperator::Equal,
        operators::NOT_EQUAL => PrimitiveOperator::NotEqual,
        operators::LESS => PrimitiveOperator::Less,
        operators::GREATER => PrimitiveOperator::Greater,
        operators::LESS_EQUAL => PrimitiveOperator::LessEqual,

        operators::AND => PrimitiveOperator::And,
        operators::OR => PrimitiveOperator::Or,
        operators::XOR => PrimitiveOperator::ExclusiveOr,

        operators::CREATE => PrimitiveOperator::Create,

        _ =>
        {
            return None;
        },
    };

    Some(PrimitiveOperatorNodeData::new(operator))
}

pub fn variable(symbol: &String, _context: &mut Context) -> Option<VariableNodeData>
{
    return Some(VariableNodeData::new(symbol.clone()));
}
