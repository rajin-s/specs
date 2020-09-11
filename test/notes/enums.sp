<<< Enumerated Types >>>

type Expression
{
    # A type can be marked with the internal Enumerated trait to turn it into an abstract enum base type
    #   - an Expression can't be directly instantiated, but it can be used as a non-partial parameter type
    is Enumerated

    public
    {
        # Functions, data, etc. can be defined as normal
        fn self.Evaluate -> int { 0 }
    }

    ==>
        # A data section containing a C enum and data union is generated
        #   - C enum has one value for each derived type
        #   - Maybe a pointer instead???
        data
        {
            __K : __ExpressionType
            __V : __Union {
                Data/Expression/Number
                Data/Expression/Add
                Data/Expression/Multiply
            }
        }

        # Methods are converted to matches on the type enum that forward the call and cast to the correct type
        #   - Only for derived types that implement the method, otherwise uses the default (potentially erroring if no default given)
        fn self.Evaluate -> int
        {
            when
            {
                (self.__K == Type/Expression/Number)   => (Evaluate/Expression/Number   self)
                (self.__K == Type/Expression/Add)      => (Evaluate/Expression/Add      self)
                (self.__K == Type/Expression/Multiply) => (Evaluate/Expression/Multiply self)
            }
            else
            {
                # If a method has a default implementation, it is used for the else case
                0
            }
        }
}

type Number
{
    data
    {
        value : int
    }

    is Expression
    {
        fn self.Evaluate -> int
        {
            self.value
        }
    }

    fn New [value int] -> Number
    {
        let new = (create Number)
        new.value <- value

        new
    }
}
    ==>
        # Moves variant data into its own container type
        type Data/Expression/Number
        {
            data { value: int }
        }

        type Number
        {
            # Includes __K, __V from expression base type
            is Expression
            {
                fn self.Evaluate -> int
                {
                    # Data member access turns into a cast / access on the data container
                    (self.__V as Data/Expression/Number).value
                }
            }

            fn New [value int] -> Number
            {
                # Create operator calls turn into creating the base type, then initializing the variant enum
                let new = (create Expression)
                    (new.__K <- Type/Expression/Number)

                (new.__V as Data/Expression/Number).value <- value
            }
        }

type Add
{
    data
    {
        a : Expression
        b : Expression
    }

    is Expression
    {
        fn self.Evaluate -> int
        {
            (self.a.Evaluate) + (self.b.Evaluate)
        }
        ==>
            fn Evaluate/Add [self Add] -> int
            {
                (Evaluate/Expression self.a)
                + (Evaluate/Expression self.b)
            }
    }

    public
    {
        fn IntoMultiply [x Add] -> Multiply
        {
            (Multiply.New x.a x.b)
        }
        ==>
            fn IntoMultiply [x Add] -> Multiply
            {
                (Multiply.New
                    x.__V.a
                    x.__V.b)
            }
    }
}

type Multiply
{
    data
    {
        a : Expression
        b : Expression
    }

    is Expression
    {
        fn self.Evalutae -> int
        {
            (self.a.Evaluate) * (self.b.Evaluate)
        }
    }

    public
    {
        fn IntoAdd [x Multiply] -> Add
        {
            (Add.New x.a x.b)
        }
    }
}

let a = (Number 100)
let b = (Number 200)
let c = (Add a b)
let d = (Add c (Number 1))

let e = (d.Evaluate)

# A derived type is always treatable as the base type
fn Swap [n Number] [a Expression]
{
    n <-> a
}

# User-defined match statements can turn a base type into a derived type
let e =
    match d
    {
        Add
        {
            (Add.IntoMultiply d)
        }
        ==> # Checks variant enum for each case
            (d.__K == Type/Expression/Add) =>
            {
                # Re-binds the matched name (given or just copy from variable)
                let d = (d as Add)
                (Add.IntoMultiply d)
            }

        Multiply
        {
            (Multiply.IntoAdd d)
        }
    }

let f =
    match foo_result = (Foo)
    {
        Add
        {
            (Add.IntoMultiply foo_result)
        }

        Multiply or Number
        {
            foo_result
        }
    }

    ==> let f = {
        let foo_result = (Foo)
        if foo_result.__K == Type/Expression/Add then
        {
            let foo_result = (foo_result as Add)
            (Add.IntoMultiply foo_result)
        }
        else if foo_result.__K == Type/Expression/Multiply or foo_result.__K == Type/Expression/Number
        {
            # Can't cast because the type isn't fully known
            #   - What about derived types that are ALSO variants of another thing?
            #   - What about traits shared by particular derived types?
            foo_result
        }
    }

type Animal
{
    is Enumerated
        # Type/Animal/Dog
        # Type/Animal/Elephant
    public
    {
        fn self.GetSpecies { ... }
    }
}

type Pet
{
    is Enumerated
        # Type/Pet/Dog
        # Type/Pet/Rock
        # Type/Pet/MiniElephant

    public
    {
        fn self.Greet { ... }
    }
}

type Dog
{
    is Animal
    is Pet
}

type Rock
{
    is Pet
}

type Elephant
{
    is Animal
    is Enumerated
}

type NormalElephant
{
    is Elephant
}
type MiniElephant
{
    is Elephant
    is Pet
}

type Maybe
    T : is Type
{
    is Enumerated
    is Sealed

    type Some
    {
        is Maybe
        
        data { self.value : T }
        public { (get self.value) }
    }

    type None
    {
        is Maybe
    }
}

let v = (Maybe.Some 123)
let v = ((Maybe int).Some 123)
let v = (Some 123)

fn Half [x int] -> (Maybe int)
{
    if x % 2 == 0
    {
        (Maybe.Some (x / 2))
    }
    else
    {
        (Maybe.None)
    }
}

fn Increment [x (Maybe int)] -> (Maybe int)
{
    match x
    {
        Maybe.Some => (Maybe.Some x.value + 1)
        Maybe.None => (Maybe.None)
    }
}

type Maybe [T is Type]
{
    is Enumerated

    # Prevent additional variants in other files
    is Sealed

    # Prevent inclusion as trait for other types
    is Final
}

type Some [T is Type]
{
    is (Maybe T)
    is Final

    data { value : T }
    public
    {
        (get value)
    }
}

type None [T is Type]
{
    is (Maybe T)
    is Final
}