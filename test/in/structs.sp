type Foo
{
    data
    {
        self.a : int
        self.b : int

        GlobalValue : int
    }

    public
    {
        (read-write self.a)
        (read       self.b)

        (read-write GlobalValue)
    }

    public
    {
        fn New [a int] [b int] -> Foo
        {
            let new = (create Foo)
            {
                new.a = a
                new.b = b
            }

            new
        }

        fn GetGlobalValue -> int
        {
            Foo.GlobalValue
        }

        fn self.GetSum -> int
        {
            self.a + self.b
        }
    }
}

Foo.GlobalValue = 100
let x = (Foo.GetGlobalValue)

let instance = (Foo.New 1 2)
let y = (instance.GetSum)

x + y

type Vector2
{
    is PassByValue
    
    data
    {
        self.x : int
        self.y : int
    }

    public
    {
        (read-write self.x)
        (read-write self.y)

        fn New [x int] [y int] -> Vector2
        {
            let new = (create Vector2)
            new.x = x
            new.y = y

            new
        }

        fn self.Add [other Vector2] -> Vector2
        {
            let result = (create Vector2)
            result.x = self.x + other.x
            result.y = self.y + other.y

            result
        }
    }
}

let A = (Vector2.New 1 2)
let B = (A.Add (Vector2.New 3 4))

A.x + B.x + A.y + B.y