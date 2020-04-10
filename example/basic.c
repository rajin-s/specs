fn Add
    [x int]
    [y int]
    -> int
{
    fn Add2
        [a int]
        [b int]
        -> int
    {
        return a + b;
    }

    return (Add2 x y)
}

<
    fn (Add/Add2 [a int] [b int]) -> int
    {
        (return (a + b))
    }

    fn (Add [x int] [y int]) -> int
    {
        (return (Add/Add2 x y))
    }
>

{ C
    int Add__Add2(int a, int b)
    {
        return a + b;
    }

    int Add(int x, int y)
    {
        return Add__Add2(x, y);
    }
}