#! Groups idea rejected

type Foo
{
    # Data is ONLY private and only top-level
    data
    {
        self.foo   : int
        self.value : int
    }

    # Groups can be public or private
    #   - Unnamed groups represent top-level names
    public group  {}
    private group {}


    # Groups can be named
    #   - Accessed like instance.group.member
    public group helper
    {
        # Group members simply have their names transformed
        #   => self.helper\Hello
        fn self.Hello { ... }
    }

    public group get
    {
        # Shorthand accessor functions make data 'visible'
        #   => fn self.get\value
        (get self.value)
    }

    public group get-ref
    {
        # The same member name can be used in different groups
        #   => fn self.get-ref\value
        (get ref self.value)
    }

    public group get-mut
    {
        (get mut-ref self.value)
    }
}

let foo = (Foo ...) # value = 1
let x   = foo.get.value
let y   = foo.get-ref.value
let z   = foo.get-mut.value

z <- 2

# foo.value => 2
# x         => 1
# y         => (ref -> 2)
# z         => (ref -> 2)