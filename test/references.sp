let a = 123
let a-ref = (ref a)

let b = 321
(@ a-ref) = b

let c = 0
a-ref = (ref c)

(@ a-ref) = 999

let a-ref-ref = (ref a-ref)
(@ a-ref-ref) = (ref b)

let d = 0
(@ (ref d)) = 1

let result = a + (@ (@ a-ref-ref))
1