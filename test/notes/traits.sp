# Any type declaration can be used as a trait
type TypeA
{
    data
    {
        a : int
    }

    private
    {
        fn self.FooA [x int] -> int { self.a + x }
    }
    public
    {
        fn New -> TypeA { ... }
        fn self.BarA [x int] -> int { self.a - x }
    }
}
type TypeB
{
    # Essentially importants the entire definition of TypeA into TypeB
    #   - including data, private methods, and public
    #   - private methods and data are accessible in TypeB
    is TypeA
    
    data
    {
        #! Can data members be shadowed?
        b : int
    }

    private
    {
        fn self.FooB [x int] -> int { self.a + self.b }
    }
    public
    {
        #! Decide if method names from trait types can be shadowed? (maybe just in trait overrides?)
        fn New -> TypeB { ... }
        fn self.BarB [x int] -> int { self.a - self.b }
    }
}

# Abstract types (that are only used as traits) can be marked with the special 'Trait' trait
type TypeC
{
    is Trait

    data
    {
        c : int
    }

    private
    {
        # Method in Trait types can be left empty if there is no suitable default implementation
        #   - Type checker handles this as a special case, any non-empty default implementation must be valid
        fn self.FooC [x int] -> int {}
        fn self.BarC -> int { self.c + 123 }
        fn BazC -> int { 10 }
    }
}
type TypeD
{
    is TypeC
    {
        # Traits can have implementation blocks that specify method overrides
        #   - Essentially replaces the definition from the trait type
        fn self.FooC [x int] -> int { self.d * x }

        # Type casting can be used to access base implementation
        #!  Should this be just inside the override? Inside the type? Anywhere?
        fn self.BarC -> int
        {
            (self.TypeC.BarC) + 100
                => (TypeC.BarC (self as TypeC)) + 100
        }

        # Static methods in base type can still be accessed as normal
        #   - As long as there is an implementation
        fn BazC -> int
        {
            (TypeC.BazC) + 1
        }
    }

    data
    {
        d : int
    }
}

# Multiple traits can be implemented by the same type
type TypeE1
{
    is Trait
    data   { e : bool }
    public { fn FooE -> int {} }
}
type TypeE2
{
    is Trait
    data { e : int }
    public { fn FooE -> bool {} }
}

type TypeF
{
    data
    {
        e : float
    }

    is TypeE1
    {
        fn FooE -> int
        {
            # If a member name is shared between multiple trait types, the current scope is the default
            if self.e
            then
            {
                # Type casting can be used to access other members with the same name
                self.TypeE2.e
            }
            else
            {
                0
            }
        }
    }
    is TypeE2
    {
        fn FooE -> bool
        {
            self.TypeE1.e and self.TypeE2.e
        }
    }
}