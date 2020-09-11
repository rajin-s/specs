# Every function generates 3 things
fn F [arg1 T1] [arg2 T2] ... -> Tr
{
    ... var1 ... var2 ...
}
    # Closure type with
    #   - Unwrap/Apply function pointer
    #   - Closure variables
    struct F_closure
    {
        apply : (F_Closure* T1 T2 ... -> Tr)

        var1 : U1
        var2 : U2 ...
    }

    # Unwrap/Apply function
    #   - Unwraps closure variables from the closure
    #   - Passes closure variables to the static function
    fn F_ApplyClosure [_closure F_closure*] [arg1 T1] [arg2 T2] ... -> Tr
    {
        (F arg1 arg2 ... _closure.var1 _closure.var2 ...)
    }

    # Static function
    #   - All closure variables are moved to function arguments
    fn F [arg1 T1] [arg2 T2] ... [var1 U1] [var2 U2] ... -> Tr
    {
        ...
    }


# For a declared static function
fn F [arg1 T1] [arg2 T2] ... -> Tr
{
    ...
}
    # Closure contains no data
    struct F_closure
    {
        apply : (F_closure* T1 T2 ... -> Tr)
    }

    # Wrapper trivially applies function
    fn F_ApplyClosure [_closure F_Closure*] [arg1 T1] [arg2 T2] ... -> Tr
    {
        (F arg1 arg2 ...)
    }

    # Function body is left unchanged
    fn F [arg1 T1] [arg2 T2] ... -> Tr
    {
        ...
    }


# At the call site, the function type of the operator can have three modes

# Static call to a function
(F arg1 arg2 ...)
    # No closure needed, just call the function
    (F arg1 arg2 ...)

# Dynamic call to a 'known' closure
#   - At compile-time, we know what function the closure refers to AND the layout of the closure type
#   - No indirection, only cost is associated with closure storage (still non-trivial)
#   - note: for closures with no variables, this is the same as a static function call
let f = F
let g = (GetClosure 1 2 3)
(f arg1 arg2 ...)
(g arg1 arg2 ...)
    # A closure is generated, but we can inline the unwrap/apply function
    #   - note: the closure might still be used later in an unknown context
    let f = (F_closure {})
    let g = (GetClosure 1 2 3)
    (F arg1 arg2 ...)
    (GetClosure/InnerFunction arg1 arg2 ... g.var1 g.var2 ...)
    

# Dynamic call to an 'unknown' closure
#   - At compile-time, we don't know what function the closure refers to or the layout of the closure type,
#     so it must be applied through the unwrap/apply function (stored inside the closure)
#   - 2 layers of indirection (ptr call -> static call), plus storage overhead
let f = someFunctionArgument
let g = if true then F else G
(f arg1 arg2 ...)
(g arg1 arg2 ...)
    # A closure is generated, and we can't inline unwrapping or application
    let f = someFunctionArgument
    let g = if true the (F_closure {}) else (G_closure {})
    (f.function g arg1 arg2 ...)
    (g.function g arg1 arg2 ...)

    # note: it could be possible to inline the unwrap/apply function if the closure type layout is identical
    #       AND the original function pointer is also stored inside the closure (which seems reasonable)
    #   - 1 layer of indirection (ptr call), plus storage overhead
    let g = if true then (F_closure { var1 }) else (G_closure { var1 })
    (g.static_function arg1 arg2 ... g.var1)