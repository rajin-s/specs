#include "specs_runtime.h"

/* Program Body */
int _specs__UserMain()
{
	int* a = 123;
	int * aRef = (&a);
	int* b = 321;
	(*aRef) = b;
	int* c = 0;
	aRef = (&c);
	(*aRef) = 999;
	int ** aRefRef = (&aRef);
	(*aRefRef) = (&b);
	int* d = 0;
	(*(&d)) = 1;
	int* result = (a + (**aRefRef));
	return (result + (*aRef));
}
