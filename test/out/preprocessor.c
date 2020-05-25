#include "specs_runtime.h"


/* Type Declarations */



/* Type Definitions */



/* Function Declarations */



/* Function Definitions */



/* Program Body */
int _specs__UserMain()
{
	bool* a = true;
	bool* b = false;
	bool* c = (100 < 200);
	bool* d = (a == b);
	bool* e = (c != d);
	if (a && c && d && e == d)
	{
		return a;
	}
	else
	{
		if (b)
		{
			return b;
		}
		else
		{
			return (c != d);
		}
	}
}
