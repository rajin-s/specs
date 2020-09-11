use super::parse_node::*;

pub fn integer(symbol: &String, source: &Source) -> Option<Integer>
{
    match symbol.parse::<i64>()
    {
        Ok(value) => Some(Integer::new(value, source.clone())),
        Err(_) => None,
    }
}

pub fn boolean(symbol: &String, source: &Source) -> Option<Boolean>
{
    match symbol.as_str()
    {
        constants::TRUE => Some(Boolean::new(true, source.clone())),
        constants::FALSE => Some(Boolean::new(false, source.clone())),
        _ => None,
    }
}

pub fn primitive_operator(symbol: &String, source: &Source) -> Option<PrimitiveOperator>
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

        operators::NOT => primitive::Operator::Not,
        operators::AND => primitive::Operator::And,
        operators::OR => primitive::Operator::Or,
        operators::XOR => primitive::Operator::ExclusiveOr,

        operators::CREATE => primitive::Operator::Create,

        _ =>
        {
            return None;
        }
    };

    Some(PrimitiveOperator::new(operator, source.clone()))
}

pub fn variable(symbol: String, source: Source) -> ResultLog<Variable, Error>
{
    if keywords::contains(&symbol) || primitive_data_types::contains(&symbol)
    {
        ResultLog::new_error(Error::UnexpectedKeyword(symbol, source))
    }
    else
    {
        ResultLog::Ok(Variable::new(symbol, source))
    }
}
