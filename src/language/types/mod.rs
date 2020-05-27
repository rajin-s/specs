pub mod class;
pub mod function;
pub mod primitive;
pub mod reference;
pub mod traits;

pub mod all
{
    pub use super::*;
    pub use class::*;
    pub use function::*;
    pub use primitive::*;
    pub use reference::*;
    pub use traits::*;
}

use crate::language::ReferenceMode;
use crate::utilities::Indirect;

/* -------------------------------------------------------------------------- */
/*                                    Type                                    */
/* -------------------------------------------------------------------------- */

#[derive(Debug)]
pub enum Type
{
    Unknown,
    Void,

    Reference(reference::ReferenceType),

    Primitive(primitive::PrimitiveType),
    Instance(class::InstanceType),
    Function(function::FunctionType),
    Class(class::ClassType),
}
impl Type
{
    pub fn from<T: ToType>(data: T) -> Type
    {
        return data.to_type();
    }
    pub fn into_reference(self, mode: ReferenceMode) -> Type
    {
        return reference::ReferenceType::new(mode, self).to_type();
    }

    /* -------------------------------------------------------------------------- */
    /*                              Reference Checks                              */
    /* -------------------------------------------------------------------------- */

    pub fn is_reference(&self) -> bool
    {
        match self
        {
            Type::Reference(_) => true,
            _ => false,
        }
    }
    pub fn is_value(&self) -> bool
    {
        match self
        {
            Type::Primitive(_) => true,
            Type::Instance(_) => true,
            _ => false,
        }
    }
    pub fn is_value_or_single_reference(&self) -> bool
    {
        match self
        {
            t if t.is_value() => true,

            Type::Reference(reference) => match &*reference.get_target().borrow()
            {
                t if t.is_value() => true,
                _ => false,
            },
            _ => false,
        }
    }
    pub fn dereference(&self) -> Option<OtherType>
    {
        match self
        {
            Type::Reference(reference) => Some(reference.get_target()),
            _ => None,
        }
    }

    /* -------------------------------------------------------------------------- */
    /*                                 Type Checks                                */
    /* -------------------------------------------------------------------------- */

    pub fn is_unknown(&self) -> bool
    {
        match self
        {
            Type::Unknown => true,
            _ => false,
        }
    }

    /* -------------------------------------------------------------------------- */
    /*                                   Traits                                   */
    /* -------------------------------------------------------------------------- */

    pub fn get_traits(&self) -> Indirect<traits::TraitSet>
    {
        match self
        {
            Type::Unknown | Type::Void => traits::common::indirect::empty(),

            Type::Reference(reference) => reference.get_traits(),

            Type::Primitive(primitive) => primitive.get_traits(),
            Type::Instance(instance) => instance.get_traits(),

            Type::Function(function) => function.get_traits(),
            Type::Class(_) => traits::common::indirect::empty(),
        }
    }
}

pub type OtherType = Indirect<Type>;

/* -------------------------------------------------------------------------- */
/*                                   Traits                                   */
/* -------------------------------------------------------------------------- */

pub trait ToType
{
    fn to_type(self) -> Type;
}
impl ToType for reference::ReferenceType
{
    fn to_type(self) -> Type
    {
        return Type::Reference(self);
    }
}
impl ToType for primitive::PrimitiveType
{
    fn to_type(self) -> Type
    {
        return Type::Primitive(self);
    }
}
impl ToType for class::InstanceType
{
    fn to_type(self) -> Type
    {
        return Type::Instance(self);
    }
}
impl ToType for function::FunctionType
{
    fn to_type(self) -> Type
    {
        return Type::Function(self);
    }
}
impl ToType for class::ClassType
{
    fn to_type(self) -> Type
    {
        return Type::Class(self);
    }
}

/* -------------------------------------------------------------------------- */
/*                                 Basic Types                                */
/* -------------------------------------------------------------------------- */

pub mod basic_types
{
    use super::*;

    pub fn unknown() -> Type
    {
        return Type::Unknown;
    }
    pub fn void() -> Type
    {
        return Type::Void;
    }
    pub fn integer() -> Type
    {
        return Type::Primitive(primitive::PrimitiveType::Integer);
    }
    pub fn boolean() -> Type
    {
        return Type::Primitive(primitive::PrimitiveType::Boolean);
    }
    pub fn float() -> Type
    {
        return Type::Primitive(primitive::PrimitiveType::Float);
    }
    pub mod indirect
    {
        use super::*;
        thread_local! {
            static UNKNOWN: Indirect<Type> = Indirect::new(Type::Unknown);
            static VOID: Indirect<Type>    = Indirect::new(Type::Void);
            static INTEGER: Indirect<Type> = Indirect::new(Type::Primitive(primitive::PrimitiveType::Integer));
            static BOOLEAN: Indirect<Type> = Indirect::new(Type::Primitive(primitive::PrimitiveType::Boolean));
            static FLOAT: Indirect<Type>   = Indirect::new(Type::Primitive(primitive::PrimitiveType::Float));
        }
        pub fn unknown() -> Indirect<Type>
        {
            return UNKNOWN.with(|t| t.clone());
        }
        pub fn void() -> Indirect<Type>
        {
            return VOID.with(|t| t.clone());
        }
        pub fn integer() -> Indirect<Type>
        {
            return INTEGER.with(|t| t.clone());
        }
        pub fn boolean() -> Indirect<Type>
        {
            return BOOLEAN.with(|t| t.clone());
        }
        pub fn float() -> Indirect<Type>
        {
            return FLOAT.with(|t| t.clone());
        }
    }
}

/* -------------------------------------------------------------------------- */
/*                                   Display                                  */
/* -------------------------------------------------------------------------- */

impl std::fmt::Display for Type
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result
    {
        match self
        {
            Type::Unknown => write!(f, "unknown"),
            Type::Void => write!(f, "void"),

            Type::Reference(reference) => write!(f, "{}", reference),

            Type::Primitive(primitive) => write!(f, "{}", primitive),
            Type::Instance(instance) => write!(f, "{}", instance),

            Type::Function(function) => write!(f, "{}", function),
            Type::Class(class) => write!(f, "{}", class),
        }
    }
}
