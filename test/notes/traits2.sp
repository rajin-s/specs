# class Types

type FooTrait
{
    is Trait
    fn self.Foo -> int {}
}
type BarTrait
{
    is Trait
    fn self.Bar -> int {}
}
type BazTrait
{
    is Trait
    fn self.Baz -> int {}
}

type FooBar
{
    is FooTrait
    {
        fn self.Foo -> int { ... }
    }

    is BarTrait
}

type FooBaz
{
    is FooTrait
    {
        fn self.Foo -> int { ... }
    }

    is BazTrait
}

type FooBarBaz
{
    is FooTrait
    is BazTrait
}

# Compile-time type binding
    # Create an instance of a partial type
    let SomeFoo = is FooTrait

    # Definitions go through the type system with an unbound partial type
    fn ChooseFoo [fooA SomeFoo] [fooB SomeFoo] -> SomeFoo
    {
        if (fooA.Foo) > 100
            then fooA
            else fooB
    }

    # Partial types can also be used per-argument (if consistency isn't needed)
    fn CallFoo [fooA is FooTrait] -> int
    {
        (fooA.Foo)
    }

    # Partial types must be bindable, so they can't be used only for a return type
    #   - If bound elsewhere (in an argument), then they can also appear as a return type
    # fn GetFoo -> is FooTrait {...}

        # Partial types are bound at usage-sites (call w/ args, closure creation, etc.)
        #   => We see that the first arg is an unbound partial type SomeFoo
        #   => Bind SomeFoo to FooBar
        #   => We see that the second arg is a partial type SomeFoo bound to FooBar
        (ChooseFoo FooBar FooBaz) #=> error (can't bind SomeFoo to two different types at the same time)
        (ChooseFoo FooBar FooBar) #=> ChooseFoo__FooBar
        (ChooseFoo FooBaz FooBaz) #=> ChooseFoo__FooBaz

# Names
type Trait1
{
    public
    {
        fn self.Action     { (Print.Line "Do Trait1 Stuff") }
        fn self.FromTrait1 { ... }
    }
}
type Trait2
{
    public
    {
        fn self.Action     { (Print.Line "Do Trait2 Stuff") }
        fn self.FromTrait2 { ... }
    }
}

type Foo
{
    is Trait1
    is Trait2

    public
    {
        fn self.Action
        {
            (Print.Line "Do Foo Stuff")
            (self.Trait1.Action 1 2 3)
            (self.Trait2.Action 1 2 3)
        }
    }
}

# Trait masking uses access operator?
(Foo.FromTrait1) <=> (Foo.Trait1.FromTrait1)
(Foo.FromTrait2) <=> (Foo.Trait2.FromTrait1)

fn UseTrait1 [x is Trait1]
{
    (x.Action)
}
    ==> fn UseTrait1/Foo [x Foo]
        {
            (x.Trait1.Action) # Doesn't know about Action in Foo or Trait2
        }
        (UseTrait1 Foo) # Output: "Do Trait1 Stuff"


# Thinkin' 'bout Syntax

fn Complex
    type T : Trait1 and Trait2 and Trait3 and Trait4 and Trait5
    x : T
    y : T and Trait6
    z : Trait1 and Trait2 and (not Trait3)
{
    (x.Trait1.Foo) # good
    (y.Trait6.Foo) # good
}

fn Complex2 [x T]
{
    (x.Trait1.Foo)
}

<<<
# Partial types as templates
    fn UseFoo
        x : is FooTrait
        y : is FooTrait
        -> int
    {
        (x.Foo) + (y.Foo)
    }
        (UseFoo FooBar FooBar) #=> { (x.Foo__FooBar) + (y.Foo__FooBar) }
        (UseFoo FooBar FooBaz) #=> { (x.Foo__FooBar) + (y.Foo__FooBaz) }

    fn UseFoo2
        x : is FooTrait and not BarTrait
        y : is BarTrait and not BazTrait
    {
        ...
    }
        (UseFoo2 FooBaz FooBar)

# Separate type arguments as templates
    fn ChooseFoo <T is FooTrait>
        fooA : T
        fooB : T
        -> T
    {
        if (fooA.Foo) > 100
            then fooA
            else fooB
    }

# Meta functions
    fn UseFoo [T is FooTrait] -> (T T -> int)
    {
        fn F [x T] [y T] -> int
        {
            (x.Foo) + (y.Foo)
        }

        F
    }
        ((UseFoo FooBar) x y)

# Virtual functions
    fn UseFoo
        x : is FooTrait
        y : is FooTrait
        -> int
    {
        (x.Foo) + (y.Foo)
    }
        (UseFoo FooBar FooBaz) #=> { (x.vtable.Foo) + (y.vtable.Foo) }
>>>