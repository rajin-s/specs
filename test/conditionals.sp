<<< Conditional
    Binding >>>

let x = (if true then 1 else 2)

<<< Conditional
    Assignment >>>

let y = 100
y = (if false then 3 else 4)

let a = 100
let b = 100
(if false
    then (if true then a else b)
    else (if true then b else a)
) = 200

<<< Conditional
    Operands >>>
(
    FOO
    (if true  then 1 else 2)
    (if false then 2 else 1)
)

(
    FOO
    1
    (if true
        then (if false then 1 else 2)
        else (if true  then 3 else 4)
    )
)

<<< Conditional
    Operator >>>

(
    (if true then FOO else BAR)
    1
    2
)

if true then
{
    (
        (if (FOO 1 2) > 100
            then FOO
            else BAR
        )
        (if (BAR 3 4) < 0
            then 10
            else 20
        )
        (if (BAR (FOO 5 6) 7) > 999
            then 0
            else 1
        )
    )
}
else
{
    2
}

(
    (if 1 > 2
        then { and }
        else { xor }
    )
    true
    false
)

<<< Control Flow >>>
if true then
{
    if false then
    {
        (FOO 1 2)
    }
}
else
{
    discard (BAR 3 4)
}

<<< When blocks >>>

when
{
    A : B
    C : D
    E : F
}
else
{
    G
}

#=>

(if A
    then B
    else
    (if C
        then D
        else
        (if E
            then F
            else { G })))

let cool =
    when
    {
        (func-1 1 2 3) : true
        (func-2 4 5)   : (func-3 5 2)

        1 + 3 < 12 : false
        3 + 2 > 4  : (func-4 1 2 3 4)
    }
    else
    {
        true
    }

let N = 100
when
{
    (N == 100) then
    {
        (do-stuff ...)
        (do-stuff ...)
        (do-stuff ...)
    }

    (N < 40) then
    {
        (do-stuff ...)
    }

    else
    {
        (do-stuff ...)
        (do-stuff ...)
    }
}