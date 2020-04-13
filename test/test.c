#include "specs_runtime.h"
int _USER_MAIN()
{
    {
        int foo = 10;
        int bar = 12;
        {
            int foo = ( foo + 1 );
            bar     = ( bar + foo );
        };
        int baz       = ( foo + bar );
        int* foo__ref = ( &foo );
        ( *foo__ref ) = foo;
    }
}