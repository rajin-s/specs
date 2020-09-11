use super::Indirect;
use std::collections::HashMap;

#[derive(Hash, PartialEq, Eq, Debug)]
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

pub struct TraitSet
{
    set: HashMap<Trait, ()>,
}
impl TraitSet
{
    pub fn empty() -> Self
    {
        return Self {
            set: HashMap::new(),
        };
    }
    pub fn new(traits: Vec<Trait>) -> Self
    {
        return Self {
            set: traits.into_iter().map(|t| (t, ())).collect(),
        };
    }

    pub fn has(&self, name: &String) -> bool
    {
        let t = Trait::new(name.clone());
        return self.set.contains_key(&t);
    }
}

pub mod common
{
    use super::*;

    pub fn empty() -> TraitSet
    {
        return TraitSet::empty();
    }
    pub mod indirect
    {
        use super::*;

        thread_local! {
            static EMPTY: Indirect<TraitSet> = Indirect::new(TraitSet::empty());
            static INTEGER: Indirect<TraitSet> = Indirect::new(TraitSet::new(vec![]));
            static BOOLEAN: Indirect<TraitSet> = Indirect::new(TraitSet::new(vec![]));
            static FLOAT: Indirect<TraitSet> = Indirect::new(TraitSet::new(vec![]));
        }
        pub fn empty() -> Indirect<TraitSet>
        {
            return EMPTY.with(|t| t.clone());
        }
        pub fn integer() -> Indirect<TraitSet>
        {
            return INTEGER.with(|t| t.clone());
        }
        pub fn boolean() -> Indirect<TraitSet>
        {
            return BOOLEAN.with(|t| t.clone());
        }
        pub fn float() -> Indirect<TraitSet>
        {
            return FLOAT.with(|t| t.clone());
        }
    }
}

/* -------------------------------------------------------------------------- */
/*                                   Display                                  */
/* -------------------------------------------------------------------------- */

impl std::fmt::Display for Trait
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result
    {
        write!(f, "[trait {}]", self.name)
    }
}
impl std::fmt::Debug for TraitSet
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result
    {
        write!(f, "[TraitSet]")
    }
}
