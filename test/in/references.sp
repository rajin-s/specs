let a = 123
let aRef = (ref a)

let b = 321
(deref aRef) = b

let c = 0
aRef = (ref c)

(deref aRef) = 999

let aRefRef = (ref aRef)
(deref aRefRef) = (ref b)

let d = 0
(deref (ref d)) = 1

let result = a + (deref (deref aRefRef))
result + (deref aRef)