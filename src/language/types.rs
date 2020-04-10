#[derive(Clone, PartialEq, Debug)]
pub struct Type
{
    data_type: DataType,
}
impl Type
{
    pub fn get_data_type(&self) -> &DataType
    {
        return &self.data_type;
    }
    pub fn set_data_type(&mut self, new_data_type: DataType)
    {
        self.data_type = new_data_type;
    }

    // Check if the type of the node is known yet
    pub fn is_unknown(&self) -> bool
    {
        return self.data_type == DataType::Unknown;
    }
    pub fn is_known(&self) -> bool
    {
        return !self.is_unknown();
    }

    // Create a new type
    pub fn new(data_type: DataType) -> Self
    {
        return Type {
            data_type: data_type,
        };
    }

    // Create new type constants
    pub const fn unknown() -> Self
    {
        return Type {
            data_type: DataType::Unknown,
        };
    }
    pub const fn new_constant(data_type: DataType) -> Self
    {
        return Type {
            data_type: data_type,
        };
    }

    // Get a reference to a static unknown type
    pub fn unknown_ref() -> &'static Self
    {
        static UNKNOWN_TYPE: Type = Type::unknown();
        return &UNKNOWN_TYPE;
    }
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
    pub fn to_type(self) -> Type
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
