pub struct TempNameGenerator
{
    name:   String,
    number: usize,
}
impl TempNameGenerator
{
    pub fn new(name: &str) -> Self
    {
        return Self {
            name:   name.to_owned(),
            number: 0,
        };
    }

    pub fn next(&mut self) -> String
    {
        self.number += 1;
        format!("_{}_{}", self.name, self.number)
    }
}

use crate::compiler::internal::*;

// Compiler pass instance
pub struct Pass {}
impl Pass
{
    pub fn new() -> Self
    {
        Self {}
    }
}

// Pass state
//  - Generated when descending the AST
//  - Potentially modified while ascending the AST (in execution order)
pub struct State {}
impl PassState for State
{
    fn empty() -> Self
    {
        return State {};
    }
}