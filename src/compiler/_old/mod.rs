mod internal;
mod utilities;

mod wrap_pass;
mod binding_state;

mod validate;
mod type_system;
mod basic;
mod flatten;
mod explicate;

mod specs_c;

use crate::language::node::*;
use crate::errors::compile_error::*;

pub struct Compiler {}

impl Compiler
{
    pub fn new() -> Compiler
    {
        Compiler {}
    }

    pub fn compile(mut root_node: Node) -> CompileResult
    {
        use self::internal::*;
        
        macro_rules! passes {
            { $( $pass:expr, )* } => {
                $(
                    {
                        let pass = $pass;
                        println!("Pass: {}", pass.get_name());
        
                        match apply_compiler_pass(pass, &mut root_node)
                        {
                            PassResult::Ok(warnings) =>
                            {
                                println!("finished => {}", &root_node);
                                println!("");
        
                                if !warnings.is_empty()
                                {
                                    for warning in warnings
                                    {
                                        println!("{}", warning);
                                    }
                                }
                                println!("");
                            }
                            PassResult::Err(errors) =>
                            {
                                println!("FAIL");
                                println!("");
                                for error in errors
                                {
                                    println!("{}", error);
                                }
                                return None;
                            }
                        }
                    }
                )*
            };
        }
        
        passes! {
            // => Source
        
            validate::specs_source::Pass::new(),
            // => SpecsSource
        
            type_system::build_definition_types::Pass::new(),
            type_system::infer::Pass::new(),
            type_system::print_types::Pass::new(),
            // validate::specs_typed::Pass::new(),
            // => SpecsTyped
            
            flatten::definition_names::Pass::new(),
            flatten::definitions::Pass::new(),
            flatten::operands::Pass::new(),
            flatten::bindings::Pass::new(),
            // validate::specs_flat::Pass::new(),
            // => SpecsFlat
        
            specs_c::make_user_main::Pass::new(),
            
            explicate::returns::Pass::new(),
            // explicate::allocation::Pass::new(),
            // validate::specs_explicit::Pass::new(),
            // => SpecsExplicit
            
            specs_c::sanitize_names::Pass::new(),
            // => SpecsC
        
            specs_c::convert::Pass::new(),
            // => C
        }
        
        return Some(root_node);
    }
}