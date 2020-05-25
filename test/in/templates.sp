type Summable
{
    # Allows non-implemented functions, functions can only be instantiated
    is Trait

    data
    {
        lastSum : int
    }

    private
    {
        # Instantiated as
        # fn Sum3.Calculate [self Sum3]
        # fn Sum3.Calculate [self Sum4]
        fn self.Calculate -> int { 0 }
    }

    public
    {
        (read lastSum)

        # Instanted as
        # fn Summable.Sum [self Sum3]
        # fn Summable.Sum [self Sum4]
        fn self.Sum -> int
        { 
            self.lastSum = (self.Calculate)
            self.lastSum
        }
    }
}

type Sum3
{
    data
    {
        self.a : int
        self.b : int
        self.c : int
    }

    is Summable
    {
        # Instantiated as
        # fn Sum3.Calculate [self Sum3]
        # fn Sum3.Calculate [self Sum4]
        fn self.Calculate -> int
        {
            (self.PartialSum) + self.a
        }
    }

    public
    {
        (read self.a)
        (read self.b)
        (read self.c)

        # Instantiated as
        # fn Sum3.PartialSum [self Sum3]
        # fn Sum4.PartialSum [self Sum4]
        fn self.PartialSum -> int
        {
            self.b + self.c
        }
    }
}

type Sum4
{
    data
    {
        self.d : int
    }

    is Sum3
    {
        fn self.PartialSum -> int
        {
            self.b + self.c + self.d
        }
    }

    public
    {
        (read self.d)
    }
}

# Instantiated with
#   x : Sum3, Sum4
fn PrintSum [x ref is Summable] -> void
{
    let sum = (x.Sum)
    (PrintInt sum)
}

let A = (Sum3 1 2 3)
let B = (Sum4 1 2 3 4)

# Instantiates
#   - PrintSum        [self Sum3]
#   - Summable.Sum    [self Sum3]
#   - Sum3.Calculate  [self Sum3]
#   - Sum3.PartialSum [self Sum3]
(PrintSum (ref A))

# Instantiates
#   - PrintSum        [self Sum4]
#   - Summable.Sum    [self Sum4]
#   - Sum3.Calculate  [self Sum4]
#   - Sum4.PartialSum [self Sum4]
(PrintSum (ref B))

(PrintInt A.lastSum)
(PrintInt B.lastSum)

fn Foo
    x  : int
    x2 : ref int
    y  : is Copyable
    y2 : ref is Copyable
    z  : is { Summable Copyable }
    z2 : ref is { Summable Copyable }
    w  : is not Copyable
    w2 : ref is { (not Copyable) (not Summable) }
{
    let y3 = (copy y2)
    let z3 = (copy z2)

    (PrintInt
        x + (deref x2) + (z.Sum) + (z2.Sum) + (z3.Sum))
}

fn Add [a is Numeric] [b is Numeric]
{
    a + b
}

fn Map [f (int -> int)]
{}

type Numbered
    T : is Type
{
    data
    {
        self.value : T
        self.ID    : int

        nextID : int
    }

    public
    {
        (read self.value)
        (read self.ID)
    }
}

type NumberedSum3 = (Numbered Sum3)
fn f1 [x ref NumberedSum3] -> int
{
    (x.Sum) + x.ID
}

type CopySum = is { Copyable Summable }
fn f2 [x ref CopySum] -> int
{
    (x.Sum) + 1
} >>>

<<<

# Struct definition
struct Foo
{
    <<<
    # Built in traits with optional implementation
    is PassByValue
    {
        fn self.Pass -> Foo
        {
            ...
        }
    }
    is Copyable
    {
        fn self.Copy -> Foo
        {
            ...
        }
    }

    # Built in traits that do require implementation
    is Printable
    {
        fn self.Print
        {
            (Print "[Foo x=" self.x ", y=" self.y "]") # or something...
        }
    }
    is ManuallyAllocated
    {
        fn Allocate -> Foo
        {
            ...
        }
        fn Free [instance Foo]
        {
            ...
        }
    }
    >>>

    # Top level can only be private/public groups
    private
    {
        # Members specified with `[name Type]` (or equivalently `name : Type`)
        x : int
    }
    public
    {
        y : bool

        # Instance methods defined with self.Name
        # called with (<some instance>.Name ...)
        fn self.Add [amount int]
        {
            self.x = self.x + amount
        }

        fn self.GetX -> int
        {
            self.x
        }

        # Static methods defined like normal functions
        # called with (Type.Name ...)
        fn New [y bool] -> Foo
        {
            # Allocate form is only available inside the struct?
            # How to control where things are allocated???
            # - Inferred? (ownership transfer = heap?)
            # - In alocate statement? (allocate T) / (allocate local T)
            # - As a kind of argument? (Foo.New heap ...) / (Foo.New stack ...)
            # - Heuristic? (size < N Bytes = on stack)
            allocate Foo
            {
                # All members must be initialized in the allocate body
                x : 0
                y : y
            }
        }
    }
}

let foo = (Foo.New false)
(foo.Add 100)

foo = (Foo.New true)
(foo.Add 200)

if foo.y
{
    (foo.GetX)
}
else
{
    (foo.GetX) + 10
}

<<< 
let bar = (foo.Copy)
let baz = (bar.Copy)
>>>