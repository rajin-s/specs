pub mod runtime;
pub mod symbols;

#[macro_use]
pub mod node;

#[macro_use]
pub mod types;

/* -------------------------------------------------------------------------- */
/*                                 Basic Stuff                                */
/* -------------------------------------------------------------------------- */

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ReferenceMode
{
    Mutable,
    Immutable,
}
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum MemberScope
{
    Static,
    Instance,
}
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Visibility
{
    Private,
    Public,
}

/* -------------------------------------------------------------------------- */
/*                                   Display                                  */
/* -------------------------------------------------------------------------- */
impl std::fmt::Display for ReferenceMode
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result
    {
        write!(
            f,
            "{}",
            match self
            {
                ReferenceMode::Immutable => "ref",
                ReferenceMode::Mutable => "ref-mut",
            }
        )
    }
}