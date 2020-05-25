#include "specs_runtime.h"


// Function Declarations

int Add__10(int x);
bool Is__Even(int x);
bool Is__Seven(int x);
int Add(int a, int b);
int Factorial(int n);
int _Factorial__Accumulator(int n, int acc);
int _Factorial__Multiply(int a, int b, int acc);
int _Factorial__Multiply__Accumulator(int a, int b, int acc);
int Fibonacci(int n);
int _Fibonacci__Accumulator(int n, int prev, int acc);
int DotSum(int A__x, int A__y, int B__x, int B__y);

// Function Definitions

int Add__10(int x)
{
	return (x + 10);
}
bool Is__Even(int x)
{
	if (x == 0)
	{
		return true;
	}
	else
	{
		if (x < 0)
		{
			return false;
		}
		else
		{
			return Is__Even((x + -2));
		}
	}
}
bool Is__Seven(int x)
{
	if (x == 6)
	{
		return false;
	}
	else
	{
		if (x == 8)
		{
			return false;
		}
		else
		{
			if (Is__Even(x))
			{
				return false;
			}
			else
			{
				return (2 == (x + -5));
			}
		}
	}
}
int Add(int a, int b)
{
	return (a + b);
}
int Factorial(int n)
{
	return _Factorial__Accumulator(n, 1);
	/* function _Factorial/Accumulator */
	/* function _Factorial/Multiply */
}
int _Factorial__Accumulator(int n, int acc)
{
	if (n <= 1)
	{
		return acc;
	}
	else
	{
		return _Factorial__Accumulator((n + -1), _Factorial__Multiply(n, acc, 0));
	}
}
int _Factorial__Multiply(int a, int b, int acc)
{
	/* function _Factorial/Multiply/Accumulator */
	return _Factorial__Multiply__Accumulator(a, b, 0);
}
int _Factorial__Multiply__Accumulator(int a, int b, int acc)
{
	if (b == 0)
	{
		return acc;
	}
	else
	{
		return _Factorial__Multiply__Accumulator(a, (b + -1), (a + acc));
	}
}
int Fibonacci(int n)
{
	return _Fibonacci__Accumulator(n, 1, 1);
	/* function _Fibonacci/Accumulator */
}
int _Fibonacci__Accumulator(int n, int prev, int acc)
{
	if (n == 0)
	{
		return acc;
	}
	else
	{
		return _Fibonacci__Accumulator((n + -1), acc, (prev + acc));
	}
}
int DotSum(int A__x, int A__y, int B__x, int B__y)
{
	return (A__x + B__x + A__y + B__y);
}

// User Program
int _USER_MAIN()
{
	/* function Add-10 */
	int x = Add__10(1);
	int y = Add__10(3);
	int z = Add__10(Add__10(4));
	/* function Is-Even */
	bool seven__is__even = Is__Even(7);
	/* function Is-Seven */
	bool seven__is__seven = Is__Seven(7);
	bool foo = Is__Seven(Add__10(-3));
	/* function Add */
	Add__10(Add(5, 1));
	/* function Factorial */
	/* function Fibonacci */
	(Factorial(5) + Fibonacci(5));
	/* function DotSum */
	DotSum(1, 4, 5, 2);
	return DotSum(DotSum(1, 2, 3, 4), DotSum(5, 6, 7, 8), DotSum(9, 10, 11, 12), DotSum(13, 14, 15, 16));
}