use crate::lang_sexp::sexp::SExpression;

use super::super::nodes::*;
use super::super::symbols;
use super::super::types::*;

use super::common::*;

impl FullType
{
    pub fn from_sexp(expression: &SExpression) -> Self
    {
        use SExpression::*;

        match expression
        {
            List(_, elements) =>
            {
                match elements.as_slice()
                {
                    // (fn [T...] -> T)
                    [Symbol(function), List(_, argument_types), Symbol(arrow), return_type]
                        if function == symbols::keywords::FUNCTION
                            && arrow == symbols::keywords::ARROW =>
                    {
                        unimplemented!();
                    }
                    // (&* &mut* T)
                    _ =>
                    {
                        let mut qualifiers: Vec<Qualifier> = Vec::new();
                        for element in elements.iter()
                        {
                            match element
                            {
                                Symbol(s) if s == symbols::operators::REFERENCE =>
                                {
                                    qualifiers.push(Qualifier::Reference);
                                }
                                Symbol(s) if s == symbols::operators::MUTABLE_REFERENCE =>
                                {
                                    qualifiers.push(Qualifier::MutableReference);
                                }
                                Symbol(s) =>
                                {
                                    let data_type = parse_data_type(s);
                                    return FullType::new(qualifiers, data_type);
                                }
                                List(_, _) =>
                                {
                                    unimplemented!();
                                }
                            }
                        }
                        parse_error("Missing type name", expression);
                        return FullType::unknown();
                    }
                }
            }
            Symbol(name) =>
            {
                // &*T
                let mut start_index = 0;
                let mut qualifiers: Vec<Qualifier> = Vec::new();
                for c in name.chars()
                {
                    if c == symbols::operators::REFERENCE_CHAR
                    {
                        qualifiers.push(Qualifier::Reference);
                        start_index += 1;
                    }
                    else
                    {
                        break;
                    }
                }
                let type_name = String::from(&name[start_index..]);
                let data_type = parse_data_type(&type_name);
                return FullType::new(qualifiers, data_type);
            }
        }
    }
}

fn parse_data_type(symbol: &String) -> Type
{
    match symbol.as_str()
    {
        symbols::primitive_data_types::INTEGER => Type::Integer,
        symbols::primitive_data_types::FLOAT => Type::Float,
        symbols::primitive_data_types::BOOLEAN => Type::Boolean,
        symbols::primitive_data_types::VOID => Type::Void,
        symbols::primitive_data_types::LONGSTRING => Type::LongString,

        _ => Type::Struct(symbol.clone()),
    }
}
