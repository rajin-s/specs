use crate::lang_sexp::sexp::SExpression;

pub fn parse_error(error_name: &'static str, source: &SExpression)
{
    println!("Parse Error [{}] @ '{}'", error_name, source);
}