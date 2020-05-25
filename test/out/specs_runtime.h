#include "stdio.h"
#include "stdlib.h"

typedef int bool;
#define true 1
#define false 0

int _specs__UserMain();

int main( int argc, char** argv )
{
    int result = _specs__UserMain();
    printf( "out: %d\n", result );
}

void* _specs__Allocate(size_t size)
{
    return malloc(size);
}