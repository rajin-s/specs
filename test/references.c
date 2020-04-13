#include "specs_runtime.h"
int _USER_MAIN()
{
    {
        int a             = 123;
        int* a__ref       = ( &a );
        int b             = 321;
        ( *a__ref )       = b;
        int c             = 0;
        a__ref            = ( &c );
        ( *a__ref )       = 999;
        int** a__ref__ref = ( &a__ref );
        ( *a__ref__ref )  = ( &b );
        int d             = 0;
        ( *( &d ) )       = 1;
        int result        = ( a + ( *( *a__ref__ref ) ) );
        1;
    }
}