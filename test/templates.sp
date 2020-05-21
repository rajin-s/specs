<<< It's all functions? >>>

fn Equals
    [T Type]
    -> (T T -> bool)
{
    fn [a T] [b T] -> bool
    {
        a == b
    }
}

if ((Equals int) 3 4) then
{
    1
}
else
{
    0
}

fn List
    [T Type]
    -> Type
{
    struct List
    {
        private
        {
            length : int
            data   : [T]
        }

        public
        {
            fn self.Get [index int] -> (ref T)
            {
                self.data at index
            }

            fn self.Add [value T]
            {
                # Check if we have capacity to fulfil the request
                if self.data.length == self.length
                {
                    # The data array is full, so extend it a little bit
                    # note: potentially reallocates and copies the array
                    self.data = (Specs.Memory.Extend self.data 4)
                }

                # Write the new value into the data array
                (self.data at self.length) = value

                # Add one to the length
                self.length = self.length + 1
            }

            fn self.Length -> int
            {
                self.length
            }

            fn New -> List
            {
                # Create a new list with 0 capacity and 0 elements
                # note: nothing is allocated until the first Add call
                make List from [0 []]
            }
        }
    }
}

let intList = ((List int) . New)
{
    (intList.Add 1)
    (intList.Add 2)
    (intList.Add 4)
    (intList.Add 8)
}
let boolList = ((List bool) . New)
{
    (boolList.Add true)
    (boolList.Add false)
    (boolList.Add true)
    (boolList.Add false)
}

<<< Function-like annotations? >>>
struct Dictionary
    [KeyType (is Hashable)]
    [ValueType (is Type)]
{
    ...
    public
    {
        fn self.Get [k (ref KeyType)] -> (ref ValueType)
        {
            let hash = (k.GetHash)
            ...
        }
    }
}

struct Dictionary
    KeyType   : (is Hashable)
    ValueType : (is Type)
{
    public
    {
        fn self.Get
            key : (ref KeyType)
            -> (ref ValueType)
        {
            let hash = (key.GetHash)
            ...
        }
    }
}

# Can alias types at compile-time
# note: need to differentiate between type bindings and normal bindings
#       maybe use "let type T = ..."?
type MyDictionaryType = (Dictionary string int)
let d = (MyDictionaryType.New)

# This works fine, just looks kinda weird
let d2 = ((Dictionary string int).New)

<<< As a pseudo-argument? >>>
fn Equal
    [T (is Primitive)] # Traits!
    [a (is Type)]
    [b (is Type)]
    -> bool
{
    a == b
}

let foo = (Equal int 1 2)

# Type arguments can be inferred inferred
# => pretend it's not there, get concrete types, back up information
let bar = (Equal false true)

# Results in a type-error
let baz = (Equal true 3)

<<< Traits! >>>

trait Cool
{
    private
    {
        [id int]

        fn GetDescription -> string {}
    }
    public
    {
        fn self.SayHi
        {
            (Print.Line "Hello, my ID is " self.id ". I am " (GetDescription))
        }
    }
}

trait VeryCool
    is Cool
{
    private
    {
        fn GetDescription -> string
        {
            "Very Cool B-)"
        }
        fn self.GetCatchphrase -> string {}
    }
}

trait Soft
{
    public
    {
        fn self.OnPat {}
    }
}

struct Robo
    is Cool
{
    private
    {
        fn Cool.GetDescription -> string
        {
            "Robo"
        }
    }
    public
    {
        fn New [id int] -> Robo
        {
            let new = make Robo from [id]
            new
        }
    }
}

struct Cat
    is Cool
    is Soft
{
    private
    {
        [name string]
    }
    public
    {
        # Cool functions
        fn Cool.GetDescription -> string
        {
            "Cat"
        }
        fn self.Cool.SayHi
        {
            (Print.Line "Meow, my name is " self.name " and my ID is " self.Cool.id)
        }

        # Soft functions
        fn self.Soft.OnPat
        {
            (Print.Line "Thank you for patting " self.name " the cat :3")
        }

        # Cat functions
        fn New [id int] [name string] -> Cat
        {
            let new = make Cat from [id name]
            new
        }
    }
}

struct Tim
    is VeryCool
{
    private
    {
        fn self.Cool.GetDescription -> string
        {
            "Tim"
        }
        fn self.VeryCool.GetCatchphrase -> string
        {
            "sup"
        }
    }
}

fn Hello
    [target (ref (is Cool))] # checks the traits of the input type => instantiates the template if they match
{
    (target.Cool.SayHi)
}

<<< Eventually...
    => Instantiate templates
    => Extract instance methods

    fn Hello.Cat
        [target (ref Cat)]
    {
        (Cat.Cool.SayHi target)
    }
    fn Hello.Robo
        [target (ref Robo)]
    {
        (Robo.Cool.SayHi target)
    }
    fn Hello.Tim
        [target (ref Tim)]
    {
        (Tim.Cool.SayHi target)
    }
>>>

fn Pat
    [target (ref (is Soft Cool))]
    [name string]
{
    (target.Cool.SayHi)
    (Print.Line "Hello soft friend! *pat*")
    (target.Soft.OnPat)
}

let kitty = (Cat.New 1 "Kitty")
(Hello ref kitty) # => Instantiates Hello.Cat
(Pat   ref kitty) # => Instantiates Pat.Cat

let zip = (Robo.New 442)
(Hello ref zip) # => Instantiates Hello.Robo

let tim = (Tim.New ...)
(Hello ref tim) # => Instantiates Hello.Tim
(Print.Line (tim.VeryCool.GetCatchphrase))

fn RefIdentity
    [x (ref (is Type))]
    -> (ref (is Type))
{
    x
}

let a = 1
let b = 2
let c = (Cat.New 2 "Kitten")

let a2 = (RefIdentity ref a)
let b2 = (RefIdentity ref b)
let c2 = (RefIdentity ref c)

let f = (Cat . New)
(a + b)

(asdf asdf)
(asdf (foo asdf) foo)
(foo asdf foo asdf foo)
(foo.asdf foo)

(foo.asdf)