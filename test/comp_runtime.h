#include "stdio.h"

int _USER_MAIN();

int main( int argc, char** argv )
{
    int result = _USER_MAIN();
    printf( "out: %d\n", result );
}