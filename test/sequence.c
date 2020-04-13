#include "specs_runtime.h"
int _USER_MAIN()
{
    {
        {
            int a = 1;
            int b = 2;
        };
        int _seqresult_1;
        {
            int b        = 1;
            _seqresult_1 = ( ( b + b ) + 3 );
        };
        int a = _seqresult_1;
        ;
        int _seqresult_2;
        {
            int b        = 1;
            _seqresult_2 = ( ( b + a ) + b );
        };
        int b = _seqresult_2;
        ;
        int _seqresult_5;
        {
            int _seqresult_4;
            {
                int _seqresult_3;
                {
                    _seqresult_3 = ( a + b );
                };
                int c = _seqresult_3;
                ;
                _seqresult_4 = ( a + c );
            };
            int c = _seqresult_4;
            ;
            _seqresult_5 = ( b + c );
        };
        int c = _seqresult_5;
        ;
        int _seqresult_6;
        {
            int a        = 12;
            _seqresult_6 = ( a + c );
        };
        a = _seqresult_6;
        ;

        return a;
    }
}