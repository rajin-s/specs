{
    let a = 1
    let b = 2
}
let a = {
    let b = 1
    b + b + 3
}
let b = {
    let b = 1
    b + a + b
}
let c = {
    let c = {
        let c = {
            a + b
        }
        a + c
    }
    b + c
}

a = {
    let a = 12
    a + c
}

{ (b = b + 1) b } = 1
#=>
{
    let temp = Nothing
    {
        b = b + 1
        temp = (ref b)
    }
    (@ temp) = 1
}

let a-ref = (ref a)
{ ... (@ a-ref) } = 1
#=>
{
    let temp = Nothing
    {
        ...
        temp = (ref (@ a-ref))
    }
    (@ temp) = 1
}


{ (b = b + 1) b } = { (b = b + 1) (b + 1) }
#=>
{
    let temp = Nothing
    {
        b = b + 1
        temp = (ref b)
    }
    let temp2 = Nothing
    {
        b = b + 1
        temp2 = b + 1
    }
    (@ temp) = temp2
}

# let d = { (let a = 12) (a + b) } + { (let d = 100) (d + d) }