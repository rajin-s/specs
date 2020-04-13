if foo {A} then
else if bar {B} then
else {C}

#=>

(if foo then {A} else (if bar then {B} else {C}))

<<< ~ ~ ~ >>>

let some-cond = true
let x = (if some-cond then 12 else 13)

#=>

let some-cond = true
let x = Nothing
if some-cond then { x = 12 }
else { x = 13 }

<<< ~ ~ ~ >>>

(+
    (if some-cond 1 else 2)
    100
)

#=>

let _temp = Nothing
if some-cond (_temp = 1) else (_temp = 2)
(+ _temp 100)

<<< ~ ~ ~ >>>

let foo = some-cond and {...}

#=>

let foo = (if some-cond (if {...} true) else false)