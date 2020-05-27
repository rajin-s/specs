use crate::compiler::internal::*;

use crate::language::node::primitive::Operator;
use crate::language::types::{function::FunctionType, primitive::PrimitiveType, Type};

pub fn infer_from_operands(
    operator: Operator,
    operands: &Vec<Indirect<Node>>,
) -> Option<Indirect<Type>>
{
    use Operator::*;
    use PrimitiveType::*;

    // Helper to expand a tuple of primitives into a tuple (to match on) or a Vec (for return type)
    macro_rules! expand_primitives {
        (tuple; ($x:ident)) => {
            Type::Primitive($x)
        };
        (tuple; ($( $x:ident ),*)) => {
            ($( Type::Primitive($x), )*)
        };
        (vec; ($( $x:ident ),*)) => {
            vec![$( Type::Primitive($x), )*]
        };
    }

    // Generate match arms and function types from a simple syntax
    //  TODO: Use indirect references instead of creating new types (at least for arguments, etc.)
    macro_rules! match_types {
        {
            ( $operator:ident, $target:expr)
            $( $args:tt => { $( ($( $op:ident ),* => $ret:ident), )* } )*
        } => {
            match $target
            {
                $(
                    expand_primitives!(tuple; $args) => match $operator
                    {
                        $(
                            $($op)|* => Type::from(FunctionType::new(
                                expand_primitives!(vec; $args),
                                Type::Primitive($ret)
                            )),
                        )*
                        _ => { return None; }
                    },
                )*
                _ => { return None; }
            }
        };
    }

    let operator_type = match operands.len()
    {
        // Unary Operators
        1 =>
        {
            let a = operands[0].borrow();
            let a_type = a.get_type();
            let a_type = a_type.borrow();

            match_types! {
                (operator, &*a_type)

                // ~ int
                (Integer) => {
                    (Subtract => Integer),
                }
                // ~ float
                (Float) => {
                    (Subtract => Float),
                }
            }
        }

        // Binary Operators
        2 =>
        {
            let (a, b) = (operands[0].borrow(), operands[1].borrow());
            let (a_type, b_type) = (a.get_type(), b.get_type());
            let (a_type, b_type) = (a_type.borrow(), b_type.borrow());

            match_types! {
                (operator, (&*a_type, &*b_type))

                // int ~ int
                (Integer, Integer) => {
                    (Add, Subtract, Multiply, Divide, Modulo => Integer),
                    (Less, Greater, LessEqual, GreaterEqual  => Boolean),
                }
                // int ~ float
                (Integer, Float) => {
                    (Add, Subtract, Multiply, Divide, Modulo => Float),
                    (Less, Greater, LessEqual, GreaterEqual  => Boolean),
                }
                // float ~ int
                (Float, Integer) => {
                    (Add, Subtract, Multiply, Divide, Modulo => Float),
                    (Less, Greater, LessEqual, GreaterEqual  => Boolean),
                }
                // float ~ float
                (Float, Float) => {
                    (Add, Subtract, Multiply, Divide, Modulo => Float),
                    (Less, Greater, LessEqual, GreaterEqual  => Boolean),
                }
            }
        }

        _ =>
        {
            return None;
        }
    };

    Some(Indirect::new(operator_type))
}
