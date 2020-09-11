fn Foo -> int
    x : int
{
    x + 1
}

let x = (Foo 10)
let y = (Foo x)

fn FactSum -> int
    x : int
{
    fn Helper [x int] [acc int] -> int
    {
        if x == 0
            then acc
            else (Helper (x - 1) (acc + x))
    }
    
    (Helper x 0)
}

fn FactMul -> int
    x : int
{
    fn Helper [x int] [acc int] -> int
    {
        if x =/= 0
            then (Helper (x - 1) (acc * x))
            else acc
    }
    
    (Helper x 1)
}

let n-1 = (FactSum 4)
let m-2 = (FactMul n-1)

let p =
    {
        n-1 <- n-1 + 1 
        
        {
            m-2 <- m-2 + 1
            m-2 + 1
        }
        -> m-2
        
        n-1 + m-2
    }

p <- (p + 1)

if p > 12
    and p < 32
    and p / 2 == 7
then
    {
        let q = - p
        q <- q * 2
        
        q
    }
else if p < 0 xor p < 123
then
    {
        p
    }
else
    {
        0
    }
    

if (Foo 10) > 10
then
    {
        (Foo 11)
    }
else
    {
        (Foo 9)
    }