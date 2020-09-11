# Short arguments, short return
fn Foo [x int] [y int] -> int
{
    ...
}

# A few short arguments, short-medium return
fn Foo -> int
    [x int] [y int] [z int]
{
    ...
}

# A few short arguments, long return
fn Foo [x int] [y int] [z int]
    -> SomeLongReturnType
{
    ...
}

# Long arguments, short return
fn Foo -> int
    someLongArgumentName       : int
    anotherLongArgumentName    : ALongArgumentType
    yetAnotherLongArgumentName : AnotherLongArgumentType
    aFinalLongArgumentName     : int
{
    ...
}
    # Alternate (same-length arguments block style)
    fn Foo
        [x int]
        [y int]
        [z int] -> int
    {
        ...
    }

<<< 
# Long arguments, long return
fn Foo -> group {
    SomeLongTypeInTheReturnGroup1
    SomeLongTypeInTheReturnGroup2
    SomeLongTypeInTheReturnGroup3
}
    x : (is Trait1)
    y : (is Trait2
            and Trait3
            and Trait7
            and Trait8)
    z : (is Trait4
            and Trait5
            and Trait6)
    w : int
{
    ...
}
>>>