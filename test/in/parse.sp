<<< Atomic >>>
123456789 -987654321
true false
foo bar baz

<<< Operators >>>
a +   b
a and b
a or  b
a xor b

(create a)

<<< Operator-Like >>>
a = b
a.b
(a . b)

(ref     a)
(mut-ref a)
(deref   a)

<<< Function Application >>>
(f a b c)
((foo.bar) a b c 1 2 3)
(thunk)

<<< Bindings >>>
let a = b
let b = c
(let d = e)

<<< Control Flow >>>
if A then B
if D then E else F
if (G) then { H I } else { J K }
if X then
    if Y then
        if Z then
            W

# when
# {
#     (foo bar)    => 100
#     100 == 200   => (foo bar)
#     (foo == baz) =>
#     {
#         a b c
#         d e g
#     }
# }

# when
# {
#     true  => false
#     false => true
# }
# else
# {
#     999
# }

<<< Functions >>>

fn F1
{
    a b c
}

fn F2 -> int
{
    d e f
}

fn F3 [x int]
{
    g h i
}

fn F4 [x int] -> int
{
    j k l
}

fn F5
    x : int
    y : bool
    -> int
{
    m n o
}

fn F6
    a : Foo
    b : Bar
    -> int
{
    (a.f b)
}

<<< Types >>>

fn Use5
    foobar : FooBar
    foo    : Foo
    x      : int
    y      : int
    z      : int
    -> Foo
{
    (Foo.New
        foo.value + foobar.value + x + y + z)
}

type Foo
{
    data
    {
        GlobalX : int
        GlobalY : int

        self.x : int
        self.y : int
    }

    public # Accessors
    {
        (read       GlobalX)
        (read-write GlobalY)

        (read       self.x)
        (read-write self.y)
    }

    public # Static Methods
    {
        fn SetGlobalX [value int]
        {
            if value < 100
                then Foo.GlobalX = value
                else Foo.GlobalX = 100
        }
    }

    public # Instance Methods
    {
        fn self.GetX -> int
        {
            self.x
        }
        fn self.GetSum -> int
        {
            self.x + self.y
        }
    }
}

if (F1 1 2 3 4)
    and (F2 5 6 7)
    and (F3 100 200) then
{
    let x = 10
}
else if (F2 0 0 0)
        and (F3 1 2) then
{
    let y = 100
}