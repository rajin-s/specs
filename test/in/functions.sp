<<< Top-Level Definitions
    (order independent) >>>

fn Add-10 [x int] -> int
{
    x + 10
}

let x = (Add-10 1)
let y = (Add-10 3)
let z = (Add-10 (Add-10 4))

fn Is-Even [x int] -> bool
{
    when
    {
        x == 0 => true
        x < 0  => false
    }
    else
    {
        (Is-Even x + -2)
    }
}

let seven-is-even = (Is-Even 7)

fn Is-Seven [x int] -> bool
{
    # 6 and 8 are definitely not seven, so we have an early out
    when
    {
        x == 6      => false
        x == 8      => false
        (Is-Even x) => false

    }
    else
    {
        (2 == x + -5)
    }
}

let seven-is-seven = (Is-Seven 7)
let foo            = (Is-Seven (Add-10 -3))

fn Add
    [a int]
    [b int]
    -> int
{
    a + b
}

(Add-10 (Add 5 1))

# fn Foo -> int
# {
#     1 + z # <- z is free, so this results in a type error
# }

<<< Inner Definitions
    (order independent within scope) >>>

fn Factorial [n int] -> int
{
    (Accumulator n 1)

    fn Accumulator
        n   : int  
        acc : int
        -> int
    {
        if n <= 1
            then acc
            else (Accumulator
                    n + -1
                    (Multiply n acc 0))
    }

    fn Multiply
        a   : int  
        b   : int  
        acc : int
        -> int
    {
        fn Accumulator
            a   : int  
            b   : int  
            acc : int
            -> int
        {
            if b == 0
                then acc
                else (Accumulator
                        a
                        (b + -1)
                        (a + acc))
        }

        (Accumulator a b 0)
    }
}

fn Fibonacci [n int] -> int
{
    (Accumulator n 1 1)

    fn Accumulator
        n    : int
        prev : int
        acc  : int
        -> int
    {
        if n == 0
            then acc
            else (Accumulator
                    (n + -1)
                    acc
                    (prev + acc))
    }
}

(Factorial 5) + (Fibonacci 5)

<<< Alternate-style
    argument list >>>

fn DotSum
    A-x : int
    A-y : int
    B-x : int
    B-y : int
    -> int
{
    (A-x + B-x) + (A-y + B-y)
}

(DotSum 1 4
        5 2)
(DotSum (DotSum 1 2 3 4) (DotSum 5 6 7 8)
        (DotSum 9 10 11 12) (DotSum 13 14 15 16))

<<<
<<< Function pointers
    (no closures yet!) >>>

fn Add-5 [x int] -> int { x + 5 }

fn Get-Function [x int] -> (int -> int)
{
    fn Add-1   [x int] -> int { x + 1 }
    fn Add-10  [x int] -> int { x + 10 }
    fn Add-100 [x int] -> int { x + 100 }

    if x == 0 then
    {
        Add-100
    }
    else if x == 10 then
    {
        Add-10
    }
    else if x == 5 then
    {
        Add-5
    }
    else
    {
        Add-1
    }
}

let f = (Get-Function 0)
(f 100)
(f 10)
(f 5)
(f 0)

fn Apply
    [f (int -> int)]
    [x  int]
    -> int
{
    (f x)
}

(Apply f 1)
(Apply (Get-Function 777) 1)

<<< Anonymous Functions >>>

(
    fn [x int] -> int { x + 50 }
    100
)

fn Get-Function-2 [x int] -> (int -> int)
{
    if x == 0
    {
        fn [a int] [b int] -> int { a + b }
    }
    else
    {
        fn [a int] [b int] -> int { a + a + b + b }
    }
}

((Get-Function-2 1) 2 3)

let g = (Get-Function-2 0)
(g 100 101)

>>>