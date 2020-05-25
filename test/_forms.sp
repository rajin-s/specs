# E : expression
# T : type
# s : symbol
# ~ : operator

# TE : type definition expression

# Expressions (E)

    <<< Atomic >>>

    142     # Integer
    true    # Boolean
    foo     # Variable

    <<< Operators >>>

    (E ~ E)     # Infix Binary Operator
                #   + - * / ^
                #   and or xor
                #   @

    (~ E)       # Prefix Unary Operator
                #   -
                #   not
                #   create

    <<< Operator-Like >>>

    (E = E)     # Assignment
    (E . s)     # Access

    (ref E)     # Reference
                #   ref mut-ref
    (deref E)   # Dereference

    <<< Function Application >>>

    (E E*)      # Prefix Function Operator

    <<< Binding >>>

    (let s = E) # Let binding

    <<< Control Flow >>>

    (if E then E)   # If-Then
    (if E           # If-Then-Else
        then E
        else E)

    (when               # When
        { <E => E>* })  #
    (when               # When-Else
        { <E => E>* }   #
        else E)         #   (expanded to if statements in parser)

    <<< Functions >>>

    (fn s               { E* }) # Function on no arguments, no return type
    (fn s          -> T { E* }) # Function on no arguments, with return type
    (fn s <[s T]*>      { E* }) # Function on arguments, no return type
    (fn s <[s T]*> -> T { E* }) # Function on arguments, with return type

    <<< Types >>>

    (type N TE*)    # Type

    <<< Other >>>

    { E* }  # Sequence

# Type Definition Expressions (TE)

    <data { [N T]* }>   # Data members
    <public  { ME* }>   # Public Methods/Accessors
    <private { ME* }>   # Private Methods/Accessors

    # Names (N)
        s           # Static Member Name
        (self . s)  # Instance Member Name
    
    # Method/Accessor Expressions (ME)

        <<< Acessors >>>

        (read N)        # Add reading permissions for a member
        (read-write N)  # Add reading and writing permissions for a member
                        #   note: only useful in public blocks

        <<< Functions >>>
        
        (fn N             { E* })   # Function on no arguments, no return type
        (fn N        -> T { E* })   # Function on no arguments, with return type
        (fn N [s T]*      { E* })   # Function on arguments, no return type
        (fn N [s T]* -> T { E* })   # Function on arguments, with return type