#include "stdio.h"
#include "stdlib.h"
#include "string.h"

typedef unsigned int bool;
#define FALSE 0
#define TRUE 1

typedef struct
{
    int n;
    int end;
    int step;
} _Specs_NumericIterator;
inline _Specs_NumericIterator _Specs_NumericIterator__New( int start, int end, int step )
{
    _Specs_NumericIterator new_iterator = { start, end, step };
    return new_iterator;
}
inline int _Specs_NumericIterator__Next( _Specs_NumericIterator* iterator )
{
    int result = iterator->n;
    iterator->n += 1;
    return result;
}
inline bool _Specs_NumericIterator__HasNext( _Specs_NumericIterator* iterator )
{
    if ( iterator->n >= iterator->end )
    {
        return FALSE;
    }

    return TRUE;
}

typedef struct
{
    int length;
    int* data;
} _Specs_Array__Int;
inline _Specs_Array__Int _Specs_Array__Int__New( int length )
{
    int* data                   = malloc( sizeof( int ) * length );
    _Specs_Array__Int new_array = { length, data };
    return new_array;
}
inline void _Specs_array__Int__Free( _Specs_Array__Int* array )
{
    free( array->data );
}

typedef struct
{
    int n;
    _Specs_Array__Int* array;
} _Specs_ArrayIterator__Int;
inline _Specs_ArrayIterator__Int _Specs_ArrayIterator__Int__New( _Specs_Array__Int* array )
{
    _Specs_ArrayIterator__Int new_iterator = { 0, array };
    return new_iterator;
}
inline int* _Specs_ArrayIterator__Int__Next( _Specs_ArrayIterator__Int* iterator )
{
    int* result = &iterator->array->data[iterator->n];
    iterator->n += 1;

    return result;
}
inline bool _Specs_ArrayIterator__Int__HasNext( _Specs_ArrayIterator__Int* iterator )
{
    return iterator->n < iterator->array->length;
}

typedef struct
{
    _Specs_ArrayIterator__Int* forward;
} _Specs_ArrayIterator__Int_Reversed;
inline _Specs_ArrayIterator__Int_Reversed _Specs_ArrayIterator__Int_Reversed__New( _Specs_ArrayIterator__Int* forward )
{
    _Specs_ArrayIterator__Int_Reversed new_iterator = { forward };
    return new_iterator;
}
inline int* _Specs_ArrayIterator__Int_Reversed__Next( _Specs_ArrayIterator__Int_Reversed* iterator )
{
    int index   = iterator->forward->array->length - iterator->forward->n - 1;
    int* result = &iterator->forward->array->data[index];

    iterator->forward->n += 1;

    return result;
}
inline bool _Specs_ArrayIterator__Int_Reversed__HasNext( _Specs_ArrayIterator__Int_Reversed* iterator )
{
    return _Specs_ArrayIterator__Int__HasNext( iterator->forward );
}

int main()
{
    _Specs_NumericIterator iterator = _Specs_NumericIterator__New( 0, 10, 2 );
    while ( _Specs_NumericIterator__HasNext( &iterator ) )
    {
        int i = _Specs_NumericIterator__Next( &iterator );
        printf( "%d\n", i );
    }

    _Specs_Array__Int arr = _Specs_Array__Int__New( 5 );
    {
        arr.data[0] = 100;
        arr.data[1] = 200;
        arr.data[2] = 300;
        arr.data[3] = 400;
        arr.data[4] = 500;
    }

    _Specs_ArrayIterator__Int arr_iterator = _Specs_ArrayIterator__Int__New( &arr );
    while ( _Specs_ArrayIterator__Int__HasNext( &arr_iterator ) )
    {
        int* element = _Specs_ArrayIterator__Int__Next( &arr_iterator );
        printf( "%d\n", *element );
    }

    arr_iterator = _Specs_ArrayIterator__Int__New( &arr );

    _Specs_ArrayIterator__Int_Reversed rev_iterator = _Specs_ArrayIterator__Int_Reversed__New( &arr_iterator );
    while ( _Specs_ArrayIterator__Int_Reversed__HasNext( &rev_iterator ) )
    {
        int* element = _Specs_ArrayIterator__Int_Reversed__Next( &rev_iterator );
        printf( "%d\n", *element );
    }

    _Specs_array__Int__Free( &arr );

    return 0;
}