#![feature(const_vec_new)]

use std::fs;

mod compiler;
mod language;
mod parser;

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
        println!("Source Text:");
        println!("\t{}\n", source.replace("\n", "\n\t"));

        let parser = Parser::new();

        if let Some(mut s_expression) = parser.parse_source(&source)
        {
            println!("S-Expression Result:");
            println!("\t{}\n", &s_expression);

            parser.preprocess(&mut s_expression);
            println!("Preprocessor Result:");
            println!("\t{}\n", &s_expression);

            match parser.parse(&s_expression)
            {
                Ok(node) =>
                {
                    println!("Parse Result:");
                    println!("\t{}\n", &node);

                    let mut options = CompilerOptions::new();
                    {
                        options.show_debug_output = true;
                    }

                    let mut compiler = Compiler::new(options, node);
                    let compile_result = compiler.compile();

                    if let Some(c_string) = compile_result
                    {
                        println!("C output:\n");
                        println!("{}", c_string);
                        fs::write(input_path.replace(".sp", ".c"), c_string);
                    }
                }
                Err(errors) =>
                {
                    for error in errors.iter()
                    {
                        println!("Parse error: {}", error);
                    }
                }
            }
        }
    }
    else
    {
        println!("Failed to open file: '{}'", &input_path);
    }
}
