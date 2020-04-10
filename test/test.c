#include "specs_runtime.h"
int _USER_MAIN()
{
    {
        int value__1 = 1;
        int value__2 = ( value__1 + 1 );
        int _bindseq_1;
        {
            int result = ( value__1 + value__2 );
            _bindseq_1 = result;
        };
        int value__3 = _bindseq_1;
        ;
        int _bindseq_2;
        {
            int value__4 = ( ( value__1 + value__1 ) + value__1 );
            _bindseq_2   = ( value__4 + 1 );
        };
        int value__4 = _bindseq_2;
        ;
    }
}