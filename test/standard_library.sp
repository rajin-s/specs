use Specs\Collections

# Create owned, resizable lists from fixed-length arrays
# => the list now owns the original array, and can re-allocate/free it as needed
let someCoolInts = ((List int)\New [1 2 3])

# Bind instantiated template structs at compile-time
let ListOfInts = (List int)
let moreCoolInts = (ListOfInts\New [4 5 6])
{
    (moreCoolInts\Add 7)
    (moreCoolInts\Add 8)
}

# The original lists are now owned by the parent list
let ListOfListsOfInts = (List ListOfInts)
let someCoolLists = (ListOfListsOfInts\New [someCoolInts moreCoolInts])

let dict = ((Dictionary int int)\New)
{
    (dict\Add 1 0)
    (dict\Add 2 1)
    (dict\Add 4 2)

    # Exploit the fact that A : B is universally converted to [A B]
    (dict\AddMany [
        8  : 3
        16 : 4
        32 : 5
        64 : 6
    ])
}

# (dict\Get k) returns an optional value
# => TBD how to handle union types
let log-1  = unwrap? (dict\Get 1)
let log-4  = unwrap? (dict\Get 4)
let log-64 = unwrap? (dict\Get 64)