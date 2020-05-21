# Simple-typed arrays
{
    # Fixed-length array literals
    let someArray = [1 2 3 4 5 6 7]
        #=> specs_array__int someArray = specs_array__int_New(7, {1, 2, 3, 4, 5, 6, 7});

    # For integral types, index operator returns a value???
    let foo = someArray at 0
        #=> int foo = someArray[0];

    # Getting a reference to an element requires the ref operator
    let bar   = ref (someArray at 1)
    deref bar = 100
    (someArray at 1)
        #=> int bar = &someArray[1];
        #   *bar = 100;
        #   return someArray[1];
}

# Complex-typed arrays
{
    struct Complex  { ... }

    let complexArray = [
        (Complex.New ...)
        (Complex.New ...)
        (Complex.New ...)
        (Complex.New ...)
    ]
    #=> Complex complexArray[] = { ... };

    # For non-integral types, index operator returns a reference
    # => so (X @ i) becomes (ref (X @ i)) are equivalent
    let complexElement = complexArray @ 0
    let complexElementRef = ref (complexArray @ 0)
        #=> Complex * complexElement = &complexArray[0];
        #   Complex * complexElementRef = &complexArray[0];
}

# Iteration
{
    let N = -40

    for each i in (0 to 10 by 2) { <<<...>>> }
    for each i in (1 to 10)      { <<<...>>> }
    for each i in (0 to N)       { <<<...>>> }

    let countUp   = (0 to 10)
    let countDown = (reverse countUp)

    fn IsMultipleOfSeven [x int] -> bool { x % 7 == 0 }
    let countSevens = (filter (0 to 100) IsMultipleOfSeven)

    let array = [...]
    for each element in arr { <<<...>>> }
    for each [i element] in (enumerate arr) { <<<...>>> }
    for each [i element] in (reverse (enumerate arr)) { <<<...>>> }

    let arrayIterator1 = (enumerate arr)
    let arrayIterator2 = (reverse (enumerate arr))
    let arrayIterator3 = (reverse (filter (enumerate arr) SomePredicate))

    for each [i element] in arrayIterator1 { <<<...>>> }
    for each [i element] in arrayIterator2 { <<<...>>> }
    for each [i element] in arrayIterator3 { <<<...>>> }

    for each pair in (enumerate arr)
    {
        let i       = pair at 0
        let element = pair at 1
    }

    for each element in (consume arr) { <<<...>>> }
}
{
    struct Foo
    {
        public
        {
            [x int]
            [y int]

            fn self.Combine
            {
                self.x = self.x + self.y
                self.y = 0
            }
        }
    }

    let fooArray = [(Foo.New ...) ...]

    let length = fooArray.length

    for each i from 0 to length - 1
    {
        let foo = fooArray at i
        (foo.Combine)
        
        foo.y = (fooArray at i + 1) . y
    }

    for each foo in fooArray
    {
        let x = foo.x
        let y = foo.y
        
        (foo.Combine)

        let combined = foo.x
        if combined =/= x + y
        {
            (Program.Exit 1)
        }
    }

    <<<
        for (int _i = 0; _i < foo.length; _i++)
        {
            Foo * foo = &fooArray[_i];

            int x = foo->x;
            int y = foo->y;

            _Foo__Combine(foo);

            int combined = foo->x;
            if (combined != (x + y))
            {
                exit(1);
            }
        }
    >>>

    for each foo in fooArray
    {
        if foo.y =/= 0
        {
            (Program.Exit 1)
        }
    }
}