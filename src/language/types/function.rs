use super::*;
use traits::TraitSet;

#[derive(Debug)]
pub struct FunctionType
{
    arguments:   Vec<OtherType>,
    return_type: OtherType,
    traits:      Indirect<TraitSet>,
}
impl FunctionType
{
    pub fn new(arguments: Vec<Type>, return_type: Type) -> Self
    {
        return Self {
            arguments:   arguments.into_iter().map(OtherType::new).collect(),
            return_type: OtherType::new(return_type),
            traits:      traits::common::indirect::empty(),
        };
    }

    pub fn from(arguments: Vec<OtherType>, return_type: OtherType) -> Self
    {
        return Self {
            arguments,
            return_type,
            traits: traits::common::indirect::empty(),
        };
    }

    get!(get_arguments   -> arguments : &Vec<OtherType>);
    get!(get_return_type -> return_type.clone() : OtherType);
    get!(get_traits      -> traits.clone() : Indirect<TraitSet>);
}

impl std::fmt::Display for FunctionType
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result
    {
        let _ = write!(f, "(");
        for argument in self.arguments.iter()
        {
            let _ = write!(f, "{} ", argument);
        }

        write!(f, "-> {})", self.return_type)
    }
}
