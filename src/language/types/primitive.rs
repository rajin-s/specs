use super::*;

#[derive(Debug)]
pub enum PrimitiveType
{
    Integer,
    Boolean,
    Float,
}
impl PrimitiveType
{
    // note: All primitive types share the same trait set
    pub fn get_traits(&self) -> Indirect<traits::TraitSet>
    {
        match self
        {
            PrimitiveType::Integer => traits::common::indirect::integer(),
            PrimitiveType::Boolean => traits::common::indirect::boolean(),
            PrimitiveType::Float => traits::common::indirect::empty(),
        }
    }
}

impl std::fmt::Display for PrimitiveType
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result
    {
        write!(
            f,
            "{}",
            match self
            {
                PrimitiveType::Integer => "int",
                PrimitiveType::Boolean => "bool",
                PrimitiveType::Float => "float",
            }
        )
    }
}
