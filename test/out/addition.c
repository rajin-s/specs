#include "specs_runtime.h"


/* Type Declarations */



/* Type Definitions */



/* Function Declarations */



/* Function Definitions */



/* Program Body */
int _specs__UserMain()
{
	int* one = 1;
	int* two = (one + one);
	int* three = (one + two);
	
		int* _xseq_1;
		{
			int* four = 0;
			{
				four = (four + one);
				four = (four + one);
				four = (four + one);
				four = (four + one);
			}
			_xseq_1 = four;
		}
		int* four = _xseq_1;
	
}
