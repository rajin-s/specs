#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Reference
{
    Immutable,
    Mutable,
}

#[derive(Clone, PartialEq, Debug)]
pub struct Type
{
    data_type:        DataType,
    reference_layers: Vec<Reference>,
}
impl Type
{
    // Interact with the data type
    pub fn get_data_type(&self) -> &DataType
    {
        return &self.data_type;
    }
    pub fn set_data_type(&mut self, new_data_type: DataType)
    {
        self.data_type = new_data_type;
    }

    // Interact with reference layers
    pub fn get_reference_layers(&self) -> &Vec<Reference>
    {
        return &self.reference_layers;
    }
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

    // Check if the type of the node is known yet
    pub fn is_unknown(&self) -> bool
    {
        return self.data_type == DataType::Unknown;
    }

    // Create a new value type
    pub fn new(data_type: DataType) -> Self
    {
        return Type {
            data_type:        data_type,
            reference_layers: Vec::new(),
        };
    }

    // Create a new type from some type data
    pub fn from<T: ToType>(value: T) -> Self
    {
        return value.to_type();
    }

    // Create new type constants
    pub const fn unknown() -> Self
    {
        return Type {
            data_type:        DataType::Unknown,
            reference_layers: Vec::new(),
        };
    }
    pub const fn new_constant(data_type: DataType) -> Self
    {
        return Type {
            data_type:        data_type,
            reference_layers: Vec::new(),
        };
    }
    // Get a reference to a static unknown type
    pub fn unknown_ref() -> &'static Self
    {
        static UNKNOWN_TYPE: Type = Type::unknown();
        return &UNKNOWN_TYPE;
    }
    // Get a reference to a static void type
    pub fn void_ref() -> &'static Self
    {
        static VOID_TYPE: Type = Type::new_constant(DataType::Void);
        return &VOID_TYPE;
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
    Callable(CallableTypeData),
}

/* -------------------------------------------------------------------------- */
/*                                  Functions                                 */
/* -------------------------------------------------------------------------- */

#[derive(Clone, Debug)]
pub struct CallableTypeData
{
    argument_types: Vec<Type>,
    return_type:    Box<Type>,
}
impl CallableTypeData
{
    pub fn get_argument_types(&self) -> &Vec<Type>
    {
        return &self.argument_types;
    }
    pub fn get_return_type(&self) -> &Type
    {
        return &self.return_type;
    }

    pub fn new(argument_types: Vec<Type>, return_type: Type) -> Self
    {
        return CallableTypeData {
            argument_types: argument_types,
            return_type:    Box::new(return_type),
        };
    }
}
impl ToType for CallableTypeData
{
    fn to_type(self) -> Type
    {
        return Type::new(DataType::Callable(self));
    }
}
impl PartialEq for CallableTypeData
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
