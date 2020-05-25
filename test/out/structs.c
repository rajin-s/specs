#include "specs_runtime.h"

/* Type Declarations */

struct Foo;
struct Vector2;


/* Type Definitions */

typedef struct Foo
{
	int* a;
	int* b;
} Foo;

struct
{
	int* GlobalValue;
} Foo__static;
typedef struct Vector2
{
	int* x;
	int* y;
} Vector2;



/* Function Declarations */

Foo* Foo__New(int* a, int* b);
int* Foo__GetGlobalValue();
int* Foo__GetSum(Foo * self);
Vector2* Vector2__New(int* x, int* y);
Vector2* Vector2__Add(Vector2 * self, Vector2* other);


/* Function Definitions */

Foo* Foo__New(int* a, int* b)
{
	Foo* new = _specs__Allocate(sizeof(Foo));
	{
		(new.a) = a;
		(new.b) = b;
	}
	return new;
}
int* Foo__GetGlobalValue()
{
	(Foo__static.GlobalValue);
}
int* Foo__GetSum(Foo * self)
{
	return ((self->a) + (self->b));
}
Vector2* Vector2__New(int* x, int* y)
{
	Vector2* new = _specs__Allocate(sizeof(Vector2));
	(new.x) = x;
	(new.y) = y;
	return new;
}
Vector2* Vector2__Add(Vector2 * self, Vector2* other)
{
	Vector2* result = _specs__Allocate(sizeof(Vector2));
	(result.x) = ((self->x) + (other.x));
	(result.y) = ((self->y) + (other.y));
	return result;
}


/* Program Body */
int _specs__UserMain()
{
	/* type Foo */;
	(Foo__static.GlobalValue) = 100;
	int* x = (Foo__static.GetGlobalValue)();
	Foo* instance = (Foo__static.New)(1, 2);
	int* y = (instance.GetSum)();
	(x + y);
	/* type Vector2 */;
	Vector2* A = (Vector2__static.New)(1, 2);
	Vector2* B = (A.Add)((Vector2__static.New)(3, 4));
	return ((A.x) + (B.x) + (A.y) + (B.y));
}
