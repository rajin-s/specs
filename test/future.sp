
# let a = Complex->new(...) # Complex* a = Complex__new(...);
# let b = a                 # Complex* b = a
# let c = { ... b }         # Complex* _temp; {...; _temp = b;} Complex* c = _temp
#                           # _Complex__free(c);

# let new-thing = {
#     let a = Foo->Bar()
#     (a->Set-Member 100)
#     a
# }

# let new-thing = Foo->Bar()
# {
#     (new-thing->Set-Member 100)
# }

<<< FUNCTION
    EXAMPLES >>>

# # Single parameter
# fn Add-1
#     [x Int]
#     -> Int
# {
#     x + 1
# }

# # Multiple parameter
# fn Add-2
#     [x Int]
#     [y Int]
#     -> Int
# {
#     x + y
# }

# # Pass static functions
# fn Transform-Int
#     [x Int]
#     [f (Int -> Int)]
#     -> Int
# {
#     (f x)
# }

# # Anonymous functions / binding
# let add-3 = fn [x Int] { x + 3 }

# # Function application
# (Transform-Int 6 add-3)

<<< HIGHER ORDER FUNCTION
    EXAMPLES (potential future plan) >>>

# fn Add-N
#     [n Int]
#     -> (Int -> Int)
# {
#     fn [x Int]
#     {
#         x + n
#     }
# }

# fn Apply
#     [f (Int -> Int)]
#     [x Int]
#     -> Int
# {
#     (f x)
# }

# let add-1 = (Add-N 1)
# let add-2 = (Add-N 2)
# let add-3 = (Add-N 3)

# (Apply add-1 1)
# (add-2 (add-3 2))

<<< C

    typedef struct {
        int (*function_ptr)(_Add__N_Closure* int);
        int n;
    } _Add__N_Closure;

    _Add__N_Closure Add__N(int n) {
        return { &_Apply_Add__N_Closure, n };
    }

    int _Apply_Add__N_Closure(void* closure, int x)
    {
        int n = ((_Add__N_Closure*) closure)->n;
        return x + n;
    }

    int Apply(_closure* f, int x)
    {
        int (*f_function_ptr)(int) = _closure[0];
        return f_function_ptr(x);
    }

    let add__1 = Add__N(1);
    let add__2 = Add__N(2);
    let add__3 = Add__N(3);

    Apply(&add__1, 1);

    int (*add__2_function_ptr)(int) = (&add__2)[0];
    int (*add__3_function_ptr)(int) = (&add__3)[0];
    add__2_function_ptr(add__3_function_ptr(2));
>>>

<<< STRUCT
    EXAMPLES >>>

# struct Foo
# {
#     [value-1 Int]
#     [value-2 Int]

#     public
#     {
#         fn self->Bar
#             [x Int]
#             -> Int
#         {
#             (self->BarInternal (x + self->value-1))
#         }
#     }

#     fn self->Bar-Internal
#         [x Int]
#         -> Int
#     {
#         x + self->value-2
#     }

#     fn Long-Name-Static-Function
#         [x Int]
#     {
#         (System->Console->Print "hello, `x` is my value")
#     }
# }