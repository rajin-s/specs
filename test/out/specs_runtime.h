#include "stdio.h"
#include "stdlib.h"

typedef int bool;
#define true 1
#define false 0

int __SpecsMain__();

int main( int argc, char** argv )
{
    int result = __SpecsMain__();
    printf( "out: %d\n", result );
}

void* _specs__Allocate(size_t size)
{
    return malloc(size);
}