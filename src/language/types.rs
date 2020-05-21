use crate::language::symbols;
use std::collections::{HashMap, HashSet};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Reference
{
    Immutable,
    Mutable,
}

pub type TraitSet = HashSet<String>;

#[derive(Clone, PartialEq, Debug)]
pub struct Type
{
    data_type:        DataType,
    reference_layers: Vec<Reference>,
}
impl Type
{
    get!(data_type : get_data_type -> &DataType);
    // get!(traits : get_traits -> &Option<TraitSet>);
    get!(reference_layers : get_reference_layers -> &Vec<Reference>);

    pub fn set_data_type(&mut self, new_data_type: DataType)
    {
        self.data_type = new_data_type;
    }

    // Check reference qualities
    pub fn is_value(&self) -> bool
    {
        return self.reference_layers.is_empty();
    }
    pub fn is_reference(&self) -> bool
    {
        return !self.is_value();
    }
    pub fn is_mutable_reference(&self) -> bool
    {
        match self.reference_layers.last()
        {
            Some(Reference::Mutable) => true,
            _ => false,
        }
    }
    pub fn is_single_reference_layer(&self) -> bool
    {
        return self.reference_layers.len() == 1;
    }

    // Check data type qualities
    pub fn data_type_is(&self, data_type: DataType) -> bool
    {
        return self.data_type == data_type;
    }
    pub fn is_unknown(&self) -> bool
    {
        return self.data_type == DataType::Unknown;
    }
    pub fn is_void(&self) -> bool
    {
        return self.data_type == DataType::Void;
    }
    pub fn is_callable(&self) -> bool
    {
        match (&self.data_type, self.reference_layers.is_empty())
        {
            (DataType::Function(_), true) => true,
            _ => false,
        }
    }

    // Reference / Dereference
    pub fn make_reference(&self, reference_type: Reference) -> Self
    {
        let mut new_type = self.clone();
        new_type.reference_layers.push(reference_type);

        return new_type;
    }
    pub fn make_dereference(&self) -> Option<Self>
    {
        // Make sure we aren't dereferencing a value
        if self.is_reference()
        {
            let mut new_type = self.clone();
            new_type.reference_layers.pop();

            return Some(new_type);
        }
        else
        {
            return None;
        }
    }

    // Create a new type
    pub fn new(
        data_type: DataType,
        reference_layers: Vec<Reference>,
    ) -> Self
    {
        return Self {
            data_type:        data_type,
            reference_layers: reference_layers,
        };
    }
    pub fn unknown() -> Type
    {
        return basic_types::unknown().clone();
    }

    // Create a new type from some type data
    pub fn from<T: ToType>(value: T) -> Self
    {
        return value.to_type();
    }
}

pub trait ToType
{
    fn to_type(self) -> Type;
}

/* -------------------------------------------------------------------------- */
/*                                 Data Types                                 */
/* -------------------------------------------------------------------------- */

#[derive(Clone, PartialEq, Debug)]
pub enum DataType
{
    Unknown,
    Void,
    Integer,
    Boolean,

    Function(FunctionTypeData),

    Type(TypeTypeData),
    Instance(InstanceTypeData),
}

/* -------------------------------------------------------------------------- */
/*                                  Functions                                 */
/* -------------------------------------------------------------------------- */

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum FunctionType
{
    Basic,
    InstanceMethod,
    StaticMethod,
}

#[derive(Clone, Debug)]
pub struct FunctionMetadata
{
    function_type: FunctionType,
}
impl FunctionMetadata
{
    get!(function_type : get_type -> FunctionType);

    pub fn new(function_type: FunctionType) -> FunctionMetadata
    {
        return FunctionMetadata {
            function_type: function_type,
        };
    }
    pub fn new_basic() -> FunctionMetadata
    {
        return FunctionMetadata::new(FunctionType::Basic);
    }
}

#[derive(Clone, Debug)]
pub struct FunctionTypeData
{
    argument_types: Vec<Type>,
    return_type:    Box<Type>,
    metadata:       FunctionMetadata,
}
impl FunctionTypeData
{
    pub fn get_argument_types(&self) -> &Vec<Type>
    {
        return &self.argument_types;
    }
    pub fn get_return_type(&self) -> &Type
    {
        return &self.return_type;
    }

    get!(metadata : get_metadata -> &FunctionMetadata);

    pub fn new(argument_types: Vec<Type>, return_type: Type, metadata: FunctionMetadata) -> Self
    {
        return FunctionTypeData {
            argument_types: argument_types,
            return_type:    Box::new(return_type),
            metadata:       metadata,
        };
    }
}
impl ToType for FunctionTypeData
{
    fn to_type(self) -> Type
    {
        return Type::new(DataType::Function(self), vec![]);
    }
}
impl PartialEq for FunctionTypeData
{
    fn eq(&self, other: &Self) -> bool
    {
        if self.argument_types.len() != other.argument_types.len()
        {
            return false;
        }

        for i in 0..self.argument_types.len()
        {
            if self.argument_types[i] != other.argument_types[i]
            {
                return false;
            }
        }

        return self.get_return_type() == other.get_return_type();
    }
}

/* -------------------------------------------------------------------------- */
/*                                 Structures                                 */
/* -------------------------------------------------------------------------- */

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Visibility
{
    Private,
    Public,
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MemberScope
{
    Static,
    Instance,
}
#[derive(Clone, Debug, PartialEq)]
pub struct TypeMemberData
{
    member_type: Box<Type>,
    visibility:  Visibility,
    scope:       MemberScope,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TypeTypeData
{
    name:    String,
    members: HashMap<String, Type>,

    traits: HashSet<String>,

    instance_members: HashSet<String>,
    static_members:   HashSet<String>,

    publicly_readable_members: HashSet<String>,
    publicly_writable_members: HashSet<String>,
}
impl TypeTypeData
{
    get!(name : get_name -> &String);

    pub fn add_member(
        &mut self,
        name: String,
        member_type: Type,
        scope: MemberScope,
        is_publicly_readable: bool,
        is_publicly_writable: bool,
    )
    {
        if is_publicly_readable
        {
            self.publicly_readable_members.insert(name.clone());
        }
        if is_publicly_writable
        {
            self.publicly_readable_members.insert(name.clone());
            self.publicly_writable_members.insert(name.clone());
        }

        match scope
        {
            MemberScope::Instance =>
            {
                self.instance_members.insert(name.clone());
            }
            MemberScope::Static =>
            {
                self.static_members.insert(name.clone());
            }
        }

        self.members.insert(name, member_type);
    }
    pub fn add_trait(&mut self, name: String)
    {
        self.traits.insert(name);
    }

    pub fn get_member_type(&self, name: &String) -> Option<&Type>
    {
        return self.members.get(name);
    }
    pub fn has_instance_member(&self, name: &String) -> bool
    {
        return self.instance_members.contains(name);
    }
    pub fn has_static_member(&self, name: &String) -> bool
    {
        return self.static_members.contains(name);
    }

    pub fn get_instance_type(&self) -> Type
    {
        return Type::new(
            DataType::Instance(InstanceTypeData::new(self.name.clone())),
            vec![],
        );
    }

    pub fn new_empty(name: String) -> Self
    {
        return Self {
            name:    name,
            members: HashMap::new(),

            traits: HashSet::new(),

            static_members:   HashSet::new(),
            instance_members: HashSet::new(),

            publicly_readable_members: HashSet::new(),
            publicly_writable_members: HashSet::new(),
        };
    }
}
impl ToType for TypeTypeData
{
    fn to_type(self) -> Type
    {
        let type_traits = self.traits.clone();
        return Type::new(DataType::Type(self), vec![]);
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct InstanceTypeData
{
    name: String,
}
impl InstanceTypeData
{
    get!(name : get_name -> &String);

    pub fn new(name: String) -> Self
    {
        return Self { name: name };
    }
}
impl ToType for InstanceTypeData
{
    fn to_type(self) -> Type
    {
        return Type::new(DataType::Instance(self), vec![]);
    }
}

/* -------------------------------------------------------------------------- */
/*                                    Users                                   */
/* -------------------------------------------------------------------------- */

pub trait Typed
{
    fn get_type(&self) -> &Type;
}
pub trait TypedInferred: Typed
{
    fn set_type(&mut self, new_type: Type);
}

/* -------------------------------------------------------------------------- */
/*                              Static References                             */
/* -------------------------------------------------------------------------- */
pub mod basic_types
{
    use super::*;

    pub fn unknown() -> &'static Type
    {
        lazy_static! {
            static ref T: Type = Type::new(DataType::Unknown, vec![]);
        }
        return &T;
    }
    pub fn void() -> &'static Type
    {
        lazy_static! {
            static ref T: Type = Type::new(DataType::Void, vec![]);
        }
        return &T;
    }
    pub fn integer() -> &'static Type
    {
        lazy_static! {
            static ref T: Type = Type::new(
                DataType::Integer,
                vec![]
            );
        }
        return &T;
    }
    pub fn boolean() -> &'static Type
    {
        lazy_static! {
            static ref T: Type = Type::new(
                DataType::Boolean,
                vec![]
            );
        }
        return &T;
    }
}
