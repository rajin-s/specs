use std::fs;

mod compiler;
mod language;
mod parser;
mod type_checker;

use compiler::*;
use language::nodes::*;
use parser::*;

// use type_checker::TypeChecker;
// use Compiler::Compiler;

fn print_type(node: &Node, _params: &mut ())
{
    println!("{}: {}", node, node.get_type());
}

fn main()
{
    let mut args = std::env::args();
    args.next();

    let input_path = if let Some(path) = args.next()
    {
        path
    }
    else
    {
        String::from("main.sp")
    };

    if let Ok(source) = fs::read_to_string(&input_path)
    {
        println!("source input:");
        println!("{}\n", source);

        let parser = Parser::new();

        if let Some(mut s_expression) = parser.parse_source(&source)
        {
            println!("parse_source result:");
            println!("{}\n", &s_expression);
            parser.preprocess(&mut s_expression);
            println!("preprocess result:");
            println!("{}\n", &s_expression);

            match parser.parse(&s_expression)
            {
                ParseResult::Ok(mut node) =>
                {
                    println!("parse result:");
                    println!("{}\n", &node);

                    type_checker::infer_types::apply(&mut node);

                    println!("types:");
                    let mut no_params = ();
                    node.parse_recursive(print_type, &mut no_params);
                    println!("");

                    let type_errors = type_checker::check_types::apply(&node);
                    for error in type_errors.iter()
                    {
                        println!("Type error: {}", error);
                    }

                    if type_errors.len() == 0
                    {
                        let mut compiler = Compiler::new(node);
                        let c_string = compiler.compile_c();

                        println!("C output:");
                        println!("{}", c_string);

                        fs::write(input_path.replace(".sp", ".c"), c_string);
                    }
                }
                ParseResult::Error(errors) =>
                {
                    for error in errors.iter()
                    {
                        match error
                        {
                            ParseError::Internal(message) =>
                            {
                                println!("Parse error (internal): {}", message)
                            }
                            ParseError::InvalidSExpression(expression) =>
                            {
                                println!("Parse error (invalid S-Expression): {}", expression)
                            }
                            ParseError::InvalidSymbol(symbol) =>
                            {
                                println!("Parse error (invalid symbol): '{}'", symbol)
                            }
                        }
                    }
                }
            }
        }
    }
    else
    {
        println!("Failed to open file: '{}'", &input_path);
    }

    // let mut expr = SExpression::from_string(&source.unwrap());
    // println!("{}", expr);
    // preprocessor::make_groups(&mut expr);
    // println!("{}", expr);
}
