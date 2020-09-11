#include "specs_runtime.h"

 int Foo(int x, int y){ return (x * (x + y)); } int __SpecsMain__(){ /* fn Foo */ int a = 100; int b = Foo(10, 30); int c = (a + b) - b; } 