#include "specs_runtime.h"
int _USER_MAIN() 
{
{ bool a = true;bool b = false;bool c = (100 < 200);bool d = (a == b);bool e = (c != d);if ((((a && c) && d) && (e == d))) {{ a; }};bool q = (a == (1 != 2)); }
}