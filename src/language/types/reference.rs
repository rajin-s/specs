use super::*;

#[derive(Debug)]
pub struct ReferenceType
{
    mode:   ReferenceMode,
    target: OtherType,
}
impl ReferenceType
{
    pub fn new(mode: ReferenceMode, target: Type) -> Self
    {
        return Self {
            mode,
            target: OtherType::new(target),
        };
    }
    pub fn from(mode: ReferenceMode, target: OtherType) -> Self
    {
        return Self { mode, target };
    }

    get!(get_mode   -> mode : ReferenceMode);
    get!(get_target -> target.clone() : OtherType);

    // note: Reference types share the same traits as their targets
    pub fn get_traits(&self) -> Indirect<traits::TraitSet>
    {
        self.target.borrow().get_traits()
    }
}

impl std::fmt::Display for ReferenceType
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result
    {
        match self.mode
        {
            ReferenceMode::Immutable => write!(f, "(ref {})", self.target),
            ReferenceMode::Mutable => write!(f, "(mut-ref {})", self.target),
        }
    }
}
