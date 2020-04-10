<<< ADDITION
    EXAMPLES >>>
let value-1 = 1
let value-2 = value-1 + 1

let value-3 = {
    let result = value-1 + value-2
    result
}

let value-4 = {
    let value-4 = value-1 + value-1 + value-1
    value-4 + 1
}

<<< REFERENCE
    EXAMPLES >>>

let int-value = 1               # int int__value = 1;
let int-ref   = (ref int-value) # int* int__ref = &int__value;
(deref int-ref) = 2             # *int__ref = 2