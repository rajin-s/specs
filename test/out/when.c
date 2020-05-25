#include "specs_runtime.h"

// Function Declarations

bool foo(int x, int y);
bool some__predicate(int x);
// Function Definitions

bool foo(int x, int y)
{
	return ((x < y) && (x > 1));
}
bool some__predicate(int x)
{
	return (x == 1);
}
// User Program
int _USER_MAIN()
{
	int bar = 100;
	int baz = 6;
	int value = 1;
	if (foo(bar, baz))
	{
		return 1;
	}
	else
	{
		if (some__predicate(value))
		{
			
				int _xcond_1;
				{
					int foo = 123;
					int bar = 321;
					if (foo == bar)
					{
						_xcond_1 = 3;
					}
					else
					{
						int * _xcond_2 = (&_xcond_1);
						if (foo > bar)
						{
							(*_xcond_2) = 4;
						}
						else
						{
							(*_xcond_2) = 5;
						}
					}
				}
				return _xcond_1;
			
		}
		else
		{
			return 255;
		}
	}
	/* function foo */
	/* function some-predicate */
}