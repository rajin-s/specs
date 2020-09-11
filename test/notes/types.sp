<<< Motivations
    - Things that are bad
        - Repeatedly defining trivial accessor functions
        - Wacky constructor syntax
        - Implicitly defined constructors with unexpected behavior
        - mutable-by-default instance methods
>>>


<<< Basic
    Syntax >>>

type FooType
{
    # A type can define one data block
    #   - All data is always private to the type itself (not accessible to derived types)
    data
    {
        # Data members are specified with the same format as function arguments
        #   - [name Type]
        #   - name : Type

        # Names with no prefix are static members
        #! Should static members even be a feature?
        a : int
        b : bool

        # Names prefixed by self are instance members
        self.x : int
        self.y : float
        self.z : bool
    }

    # A type can define any number of public blocks
    #   - Public members are accessible anywhere the type is bound
    public
    {
        # Accessors can appear in public blocks
        #   - Effectively expose data members (implementation TBD)
        #! Should these be allowed for static members??? (do they even exist???)

        # Unqualified accessors to instance members consume the instance and move data out
        #   - probably not too common
        (get self.x)

        # Qualified accessors return a reference to a instance members
        #   - const-ness of self must match at the call site
        #! Can the same member be exposed in multiple ways? (ref and mut-ref)
        #!  - Maybe multiple reference types (inferred based on use?)
        #!  - Maybe call-site has to be a reference node? ie. (ref instance.y)
        (get mut-ref self.z)
        (get ref     self.y)
            ==> fn self.__get_y -> (ref float) { (ref self.y) }

        # Functions can appear in public or private blocks

        # Unprefixed names are static methods
        fn Foo -> int { ... }

        # Named prefixed by self are instance method
        #   - Eventually converted to static methods that take a reference as the first argument
        #   - self is either (ref Self) or (mut-ref Self) depending on usage
        #! Should self constness be inferred or explicit?
        #! How early are instance methods converted to static methods?
        #! Can instance methods be referenced statically in user code?
        fn self.Bar -> int { ... }

        # Type defnitions can appear in public or private blocks
        #   - Bound in the scope of the type definition
        #   - Must be accessed through the type name externally (including derived types)
        type Inner
        {
            # Inner types have access to the same scope as the original type (including private members)
        }

        # Inside the type, Self is bound to the type
        #! Only really useful once more complex type features (traits, template types) are implemented
        fn New -> Self
        {
            # A constructor keyword is available only inside the type
            #   - Public "constructors" are defined as normal public methods
            #! What should the syntax for this look like?
            #! Should there be a special public constructor syntax?
            #!  - Would enable things like (Vector 1 2 3)
            #!  - Maybe allow ONE constructor function which has the same name as the type and must return Self
            let new = make FooType
            {
                let x = 0
                let y = 100.2
                let z = false
            }

            new.x <- 1

            new
        }
    }

    # A type can define any number of private blocks
    #   - Private members are only accessible in the type definition or in the definitions of derived types
    #! Or should private be just this type, and we use protected for derived types?
    private
    {
        # Accessors can appear in private blocks, allowing data to be accessed by derived classes
        (get self.x)
    }

    #! Public / Private blocks can be named to introduce organizational access bits?
    #!  - Is this a good idea?
    public GroupName
    {
        ...
    }
}

# Static methods are accessed via the . operator with the name of the type
let foo = (FooType.New)

# Instance methods are accessed via the . operator with an instance of the type
let a = (foo.Bar)

# Publicly accessible members use the same syntax as direct access inside the type
#! Confusing if the access actually returns a reference?
#!  - Could be a case for requiring ref or mut-ref around the access to make it clear/explicit?
let b = foo.y

<<< Traits
    Syntax >>>

# Any type can be a trait
type FooType
{
    # Traits can contain their own data
    data
    {
        x : int
    }

    public
    {
        # Traits can provide default method implementations
        #   - Required for non-abstract types
        fn self.Bar -> int { ... }

        #! How to handle the Self type for base/derived classes?
        fn New -> Self { ... }
    }

    #! See public/private questions above
    private
    {
        fn self.PrivateBar -> int { ... }
    }
}

type Derived
{
    # Specifying implemented traits is done with the is keyword
    is FooType
    {
        # Base type methods can be (re)implemented inside the is block
        #   - No need to re-specify access modifiers
        #   - Types and self constness must match the original
        fn self.Bar -> int { ... }
        fn self.PrivateBar -> int { ... }

        #! How to handle static methods (ie. New, that returns an instance of the base type)?
        #!  - And what about constructors if there is a "canonical constructor" syntax
        fn New -> Self { ... }
    }

    # Derived types can also contain their own data
    #! How to ensure they are fully initialized?
    data
    {
        y : int
    }
}

# The compiler provides some built-in traits that have special semantics
type BarType
{
    # Specify that the type is an abstract trait
    #   - The type is not itself instantiable (create operator is not defined)
    #   - The type can have empty function definitions when no default implementation is needed
    #       - Derived types must implement any empty methods
    is Trait

    public
    {
        #! How to handle empty void-returning functions?
        #!  - Some special 'abstract method' syntax?
        fn self.DoStuff { interface }

        #! How to handle the Self type?
        fn New -> Self { ... }
    }
}

<<< Enumerated
    Types Syntax >>>

# Enumerated types allow for run-time polymorphism
#   - Uses enum + union at the C level
type Animal
{
    # Mark that this is an abstract base type that will have derived variants defined
    #   - The type is not itself instantiable
    #   - The type can have empty function definitions when no default implementation is needed
    #       - Derived types must implement any empty methods
    is Enumerated

    public
    {
        #! How to handle void-returning abstract methods?
        fn self.SayHello { interface }
    }
}

type Dog
{
    is Animal
    {
        fn self.SayHello
        {
            (Print.Line "woof")
        }
    }
}
type Cat
{
    is Animal
    {
        fn self.SayHello
        {
            (Print.Line "meow")
        }
    }
}
type Person
{
    data
    {
        name : string
    }

    is Animal
    {
        fn self.SayHello
        {
            (Print.Line `hello my name is {self.name}`)
        }
    }
}

fn Greet [x Animal]
{
    (Print.Line "Hello!")
    (x.SayHello)
}

    ==>
        struct Animal
        {
            enum {
                __Animal__Dog,
                __Animal__Cat,
                __Animal__Person
            } __Animal__Type;

            union {
                __Animal__Dog__Data,
                __Animal__Cat__Data,
                __Animal__Person__Data
            } __Animal__Data;

            __Animal__Type __type;
            __Animal__Data __data;
        };

        void SayHello__Animal(Animal* x)
        {
            switch(x->__type)
            {
                case __Animal__Dog:
                    SayHello__Animal__Dog((Dog*) x)
                    break;
                case __Animal__Cat:
                    SayHello__Animal__Cat((Cat*) x)
                    break;
                case __Animal__Person:
                    SayHello__Animal__Person((Person*) x)
                    break;
            }
        }

        typedef Animal Dog;
        struct __Animal__Dog__Data
        {};
        void SayHello__Animal__Dog(Dog* x)
        {
            Print__Line("bark");
        }

        typedef Animal Cat;
        struct __Animal__Cat__Data
        {};
        void SayHello__Animal__Cat(Cat* x)
        {
            Print__Line("meow");
        }

        typedef Animal Person;
        struct __Animal__Person__Data
        {
            string* name;
        };
        void SayHello__Animal__Person(Person* x)
        {
            Print__Line(format ... "hello my name is " ((__Animal__Person__Data) x->__data).name);
        }
