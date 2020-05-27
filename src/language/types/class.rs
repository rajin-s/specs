use super::*;

use std::cell::Ref;
use traits::TraitSet;

/* -------------------------------------------------------------------------- */
/*                                  Instance                                  */
/* -------------------------------------------------------------------------- */

#[derive(Debug)]
pub struct InstanceType
{
    name:       String,
    traits:     Indirect<TraitSet>,
    class_type: OtherType,
}
impl InstanceType
{
    pub fn new(name: String) -> Self
    {
        return Self {
            name,
            traits: traits::common::indirect::empty(),
            class_type: basic_types::indirect::unknown(),
        };
    }

    get!(get_name   -> name : &String);
    get!(get_traits -> traits.clone() : Indirect<TraitSet>);

    pub fn set_traits(&mut self, new_traits: Indirect<TraitSet>)
    {
        self.traits = new_traits;
    }

    pub fn get_class(&self) -> Option<Ref<ClassType>>
    {
        let class_type_ref = self.class_type.borrow();
        match &*class_type_ref
        {
            Type::Class(_) =>
            {
                let class_ref = Ref::map(class_type_ref, |t| match t
                {
                    Type::Class(class) => class,
                    _ =>
                    {
                        unreachable!();
                    }
                });

                Some(class_ref)
            }
            _ => None,
        }
    }
}

/* -------------------------------------------------------------------------- */
/*                                    Class                                   */
/* -------------------------------------------------------------------------- */

#[derive(Debug)]
pub struct ClassType {}

/* -------------------------------------------------------------------------- */
/*                                   Display                                  */
/* -------------------------------------------------------------------------- */

impl std::fmt::Display for InstanceType
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result
    {
        write!(f, "{}", self.name)
    }
}
impl std::fmt::Display for ClassType
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result
    {
        write!(f, "[ClassType]")
    }
}
