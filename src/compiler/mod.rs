use crate::language::node::*;
use crate::utilities::*;

use crate::errors::compile_error::*;

mod common;

// Passes

mod type_system;

mod flatten_bindings;
mod flatten_definitions;
mod flatten_names;
mod flatten_operands;

mod explicate_main;
mod explicate_returns;

mod c_convert;
mod c_convert_names;

///
/// A compiler instance with associated configuration, etc.
///
pub struct Compiler {}

impl Compiler
{
    pub fn new() -> Compiler
    {
        Compiler {}
    }

    pub fn compile_c(&self, mut node: Node) -> ResultLog<CNode, Error>
    {
        let mut errors = Vec::new();
        let mut warnings = Vec::new();

        macro_rules! passes
        {
            {
                $($name:expr => $pass:expr,)+
            } =>
            {
                $(
                    match $pass.apply(&mut node)
                    {
                        ResultLog::Ok(()) => (),
                        ResultLog::Warn((), mut new_warnings) => warnings.append(&mut new_warnings),
                        ResultLog::Error(mut new_errors, mut new_warnings) =>
                        {
                            errors.append(&mut new_errors);
                            warnings.append(&mut new_warnings);
                            return ResultLog::Error(errors, warnings);
                        }
                    }

                    println!("# Pass {}\n\t{}\n", $name, node);
                )+
            };
        }

        passes! {
            "InferTypes"         => type_system::Infer::new(),
            "CheckTypes"         => type_system::Check::new(),

            "FlattenNames"       => flatten_names::FlattenNames::new(),
            "FlattenDefinitions" => flatten_definitions::FlattenDefinitions::new(),
            "FlattenOperands"    => flatten_operands::FlattenOperands::new(),
            "FlattenBindings"    => flatten_bindings::FlattenBindings::new(),

            "ExplicateMain"      => explicate_main::ExplicateMain::new("__SpecsMain__"),
            "ExplicateReturns"   => explicate_returns::ExplicateReturns::new(),

            "CConvertNames"      => c_convert_names::ConvertNames::new(),
            "CConvert"           => c_convert::Convert::new(),
        }

        match node
        {
            Node::CNode(cnode) => ResultLog::maybe_warn(cnode, warnings),
            _ =>
            {
                errors.push(Error::Internal(format!("Failed to get root CNode")));
                ResultLog::Error(errors, warnings)
            }
        }
    }
}
