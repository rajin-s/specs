let x = (if C1
    then (if C2 then T2 else E2)
    else (if C3 then T3 else E3)
)
    # Single-level transformation -> Incorrect
    let temp = { }
    if C1
        then temp = (if C2 then T2 else E2)
        else temp = (if C3 then T3 else E3)
    let x = temp

    # Propogating transformation -> Correct
    let temp = { }
    if C1
        then (if C2
                then temp = T2
                else temp = E2)
        else (if C3
                then temp = T3
                else temp = E3)
    let x = temp

        let x = (if ...)
                ######## -> this branch WILL assign to a temporary

                (if C1 then ... else ...)
                    ##      ############ -> these branches will be assigning to a temporary
                    ##
                    ## -> this branch will not assign to a temporary

                    (if C2 then T2 else E2)
                        ##      ########## -> these branches will be assigning to a temporary
                        ##
                        ## -> this branch will not assign to a temporary
                    
                    (if C3 then T3 else E3)
                        ##      ########## -> these branches will be assigning to a temporary
                        ##
                        ## -> this branch will not assign to a temporary