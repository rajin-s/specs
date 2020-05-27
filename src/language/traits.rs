use crate::language::symbols;
use std::collections::HashSet;

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct Trait
{
    name: String,
}
impl Trait
{
    pub fn new(name: String) -> Self
    {
        return Self { name };
    }
}

#[derive(Clone, PartialEq)]
pub struct TraitSet
{
    set: HashSet<Trait>,
}
impl TraitSet
{
    pub fn new_empty() -> Self
    {
        return Self {
            set: HashSet::new(),
        };
    }
    pub fn from_names<T: ToString>(names: Vec<T>) -> Self
    {
        return Self {
            set: names
                .into_iter()
                .map(|x| Trait::new(x.to_string()))
                .collect(),
        };
    }

    pub fn has(&self, name: String) -> bool
    {
        return self.set.contains(&Trait::new(name));
    }
    pub fn add(&mut self, new_trait: Trait)
    {
        self.set.insert(new_trait);
    }
}

pub mod basic_traits
{
    use super::*;

    pub fn none() -> &'static TraitSet
    {
        lazy_static! {
            static ref S: TraitSet = TraitSet::new_empty();
        }

        return &S;
    }
    pub fn numeric() -> &'static TraitSet
    {
        lazy_static! {
            static ref S: TraitSet =
                TraitSet::from_names(vec![
                    symbols::traits::VALUE,     // PassByValue
                    symbols::traits::NUMERIC,   // #Numeric
            ]);
        }

        return &S;
    }
}
