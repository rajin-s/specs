Source Text:
	fn Foo -> int
	    x : int
	    y : int
	{
	    x * (x + y)
	}
	
	let a = 100
	let b = (Foo 10 30)
	
	let c = (a + b) - b

S-Expression Result:
	{fn Foo -> int x : int y : int {x * (x + y)} let a = 100 let b = (Foo 10 30) let c = (a + b) - b}

Preprocessor Result:
	{(fn Foo -> int <[x int] [y int]> {(x * (x + y))}) (let a = 100) (let b = (Foo 10 30)) (let c = ((a + b) - b))}

Parse Result:
	{(fn Foo <[x int] [y int]> -> int {([op *] ~ [var x] ([op +] ~ [var x] [var y]))}) (let [a int] = [int 100]) (let [b unknown] = ([var Foo] ~ [int 10] [int 30])) (let [c unknown] = ([op -] ~ ([op +] ~ [var a] [var b]) [var b]))}

# Pass InferTypes
	{(fn Foo <[x int] [y int]> -> int {([op *] ~ [var x] ([op +] ~ [var x] [var y]))}) (let [a int] = [int 100]) (let [b int] = ([var Foo] ~ [int 10] [int 30])) (let [c int] = ([op -] ~ ([op +] ~ [var a] [var b]) [var b]))}

# Pass CheckTypes
	{(fn Foo <[x int] [y int]> -> int {([op *] ~ [var x] ([op +] ~ [var x] [var y]))}) (let [a int] = [int 100]) (let [b int] = ([var Foo] ~ [int 10] [int 30])) (let [c int] = ([op -] ~ ([op +] ~ [var a] [var b]) [var b]))}

# Pass FlattenNames
	{(fn Foo <[x int] [y int]> -> int {([op *] ~ [var x] ([op +] ~ [var x] [var y]))}) (let [a int] = [int 100]) (let [b int] = ([var Foo] ~ [int 10] [int 30])) (let [c int] = ([op -] ~ ([op +] ~ [var a] [var b]) [var b]))}

# Pass FlattenDefinitions
	{(fn Foo <[x int] [y int]> -> int {([op *] ~ [var x] ([op +] ~ [var x] [var y]))}) <<<fn Foo>>> (let [a int] = [int 100]) (let [b int] = ([var Foo] ~ [int 10] [int 30])) (let [c int] = ([op -] ~ ([op +] ~ [var a] [var b]) [var b]))}

# Pass FlattenOperands
	{(fn Foo <[x int] [y int]> -> int {([op *] ~ [var x] ([op +] ~ [var x] [var y]))}) <<<fn Foo>>> (let [a int] = [int 100]) (let [b int] = ([var Foo] ~ [int 10] [int 30])) (let [c int] = ([op -] ~ ([op +] ~ [var a] [var b]) [var b]))}

# Pass FlattenBindings
	{(fn Foo <[x int] [y int]> -> int {([op *] ~ [var x] ([op +] ~ [var x] [var y]))}) <<<fn Foo>>> (let [a int] = [int 100]) (let [b int] = ([var Foo] ~ [int 10] [int 30])) (let [c int] = ([op -] ~ ([op +] ~ [var a] [var b]) [var b]))}

# Pass ExplicateMain
	{!(fn Foo <[x int] [y int]> -> int {([op *] ~ [var x] ([op +] ~ [var x] [var y]))}) [nothing] [nothing] [nothing] [nothing] (fn __SpecsMain__ <> -> int {<<<fn Foo>>> (let [a int] = [int 100]) (let [b int] = ([var Foo] ~ [int 10] [int 30])) (let [c int] = ([op -] ~ ([op +] ~ [var a] [var b]) [var b]))})!}

# Pass ExplicateReturns
	{!(fn Foo <[x int] [y int]> -> int {([op return] ~ ([op *] ~ [var x] ([op +] ~ [var x] [var y])))}) [nothing] [nothing] [nothing] [nothing] (fn __SpecsMain__ <> -> int {<<<fn Foo>>> (let [a int] = [int 100]) (let [b int] = ([var Foo] ~ [int 10] [int 30])) (let [c int] = ([op -] ~ ([op +] ~ [var a] [var b]) [var b]))})!}

# Pass CConvertNames
	{!(fn Foo <[x int] [y int]> -> int {([op return] ~ ([op *] ~ [var x] ([op +] ~ [var x] [var y])))}) [nothing] [nothing] [nothing] [nothing] (fn __SpecsMain__ <> -> int {<<<fn Foo>>> (let [a int] = [int 100]) (let [b int] = ([var Foo] ~ [int 10] [int 30])) (let [c int] = ([op -] ~ ([op +] ~ [var a] [var b]) [var b]))})!}

# Pass CConvert
	 int Foo(int x, int y){ return (x * (x + y)); } int __SpecsMain__(){ /* fn Foo */ int a = 100; int b = Foo(10, 30); int c = (a + b) - b; } 

Compile Result:

#include "specs_runtime.h"

 int Foo(int x, int y){ return (x * (x + y)); } int __SpecsMain__(){ /* fn Foo */ int a = 100; int b = Foo(10, 30); int c = (a + b) - b; } 
