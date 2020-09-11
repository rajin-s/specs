fn Foo -> int
    x : int
    y : int
{
    x * (x + y)
}

let a = 100
let b = (Foo 10 30)

let c = (a + b) - b