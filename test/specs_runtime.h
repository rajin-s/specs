#include "stdio.h"

typedef int bool;
#define true 1
#define false 0

int _USER_MAIN();

int main( int argc, char** argv )
{
    int result = _USER_MAIN();
    printf( "out: %d\n", result );
}