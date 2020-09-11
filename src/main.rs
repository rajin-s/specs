#![feature(get_mut_unchecked)]
#![allow(dead_code)]

#[macro_use]
mod macros;
mod utilities;

mod compiler;
mod errors;
mod language;
mod parser;
mod source;

use parser::Parser;
use compiler::Compiler;
use std::fs;

fn main()
{
    use language::node::*;

    let mut args = std::env::args();
    args.next(); // Skip the first argument (executable name)

    let input_path = if let Some(path) = args.next()
    {
        path
    }
    else
    {
        String::from("main.sp")
    };

    let output_path = if let Some(path) = args.next()
    {
        path
    }
    else
    {
        input_path.replace(".sp", ".c")
    };

    let source = match fs::read_to_string(&input_path)
    {
        Ok(source) => source,
        Err(error) =>
        {
            eprintln!("Failed to open file '{}': {}", input_path, error);
            return;
        }
    };

    println!("Source Text:");
    println!("\t{}\n", source.replace("\n", "\n\t"));

    let parser = Parser::new();

    let mut s_expression = {
        use errors::s_expression_error::*;
        match parser.make_s_expression(source)
        {
            ResultLog::Ok(s_expression) => s_expression,
            ResultLog::Warn(s_expression, warnings) =>
            {
                print_warnings(&warnings);
                s_expression
            }
            ResultLog::Error(errors, warnings) =>
            {
                print_warnings(&warnings);
                print_errors(&errors);
                return;
            }
        }
    };

    println!("S-Expression Result:");
    println!("\t{}\n", &s_expression);

    parser.preprocess(&mut s_expression);

    println!("Preprocessor Result:");
    println!("\t{}\n", &s_expression);

    let node = {
        use errors::parse_error::*;
        match parser.make_node(s_expression)
        {
            ResultLog::Ok(node) => node,
            ResultLog::Warn(node, warnings) =>
            {
                print_warnings(&warnings);
                node
            }
            ResultLog::Error(errors, warnings) =>
            {
                print_warnings(&warnings);
                print_errors(&errors);
                return;
            }
        }
    };

    println!("Parse Result:");
    println!("\t{}\n", &node);

    let compiler = Compiler::new();

    let cnode = {
        use errors::compile_error::*;
        match compiler.compile_c(node)
        {
            ResultLog::Ok(cnode) => cnode,
            ResultLog::Warn(cnode, warnings) =>
            {
                print_warnings(&warnings);
                cnode
            }
            ResultLog::Error(errors, warnings) =>
            {
                print_warnings(&warnings);
                print_errors(&errors);
                return;
            }
        }
    };

    let c = format!("#include \"specs_runtime.h\"\n\n{}", cnode);
    println!("Compile Result:\n\n{}", c);

    fs::write(output_path, c).expect("Failed to write output file");
}
