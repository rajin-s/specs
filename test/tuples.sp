# Tuples are fixed-length arrays of potentially different types
# => ? converted to anonymous struct { int v0; bool b1; }
let pair = [123 true]

# Homogeneous-typed tuples are fixed length arrays
# => converted to anonymous struct { int count; int vs[2]; }
let pairOfInts = [1 2]

fn UsePair
    pair : [int bool]
    -> int
{
    # Tuples are indexed by integer values
    # => invalid indices result in compile-time errors
    if pair @ 1 then
    {
        (pair @ 0) + 100
    }
    else
    {
        (pair @ 0) + 1
    }
}

fn UseArray
    array : [int]
    -> int
{
    # Arrays have an implicit length member (read-only)
    # => this may or may not be known at compile-time
    if array\length > 0
    {
        # Arrays are indexed by integer values
        # => invalid indices result in
        #      - compile-time errors if the length of the array is known
        #      - run-time errors otherwise
        array @ 0
    }
    else
    {
        0
    }
}

fn UseTupleArray
    tuples : (ref [[bool int int]])
    -> int
{
    let result = 0

    # Iteration over an array
    # => converts to for (int i = 0; i < array.length; i++) {...}
    for tuple in tuples
    {
        when
        {
            tuple @ 0 =>
            {
                result = result + (tuple @ 1) + (tuple @ 2)
            }
            tuple @ 1 > 100 =>
            {
                result = result + 100
            }

        }
        else
        {
            result = result + 1
        }
    }
}