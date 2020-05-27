(bar)

fn foo
    x : int
    y : bool
    -> int
{
    let z = if (y and x < 100) then x else x + 1
    x + z
    let q = (ref x)
    (deref q)
}

(foo 100 true)

fn bar [x int] [y int] {}