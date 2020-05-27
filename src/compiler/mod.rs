mod internal;
mod type_system;

use crate::language::node::*;

pub fn compile(root_node: Node) -> Option<Node>
{
    use internal::*;
    let mut root_ref = OtherNode::new(root_node);

    macro_rules! passes {
        { $root:ident => $( $pass:expr, )* } => {
            $(
                {
                    let pass = $pass;
                    println!("Pass: {}", pass.get_name());

                    match apply_compiler_pass(pass, &mut $root)
                    {
                        PassResult::Ok(warnings) =>
                        {
                            println!("finished => {}", &$root);
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
        root_ref =>
            type_system::build_definition_types::Pass::new(),
            type_system::infer::Pass::new(),
            type_system::print_types::Pass::new(),
    }

    return Some(root_ref.unwrap());
}
