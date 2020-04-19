let bar = 100
let baz = 6
let value = 1

when
{
    (foo bar baz)          => 1
    (some-predicate value) =>
    {
        let foo = 123
        let bar = 321

        when
        {
            foo == bar => 3
            foo > bar  => 4
        }
        else
        {
            5
        }
    }
}
else
{
    255
}

fn foo [x int] [y int] -> bool
{
    x < y and x > 1
}

fn some-predicate [x int] -> bool
{
    x == 1
}