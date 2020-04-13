#include "specs_runtime.h"
int _USER_MAIN()
{
    {
        int one   = 1;
        int two   = ( one + one );
        int three = ( one + two );
        int _seqresult_1;
        {
            int four = 0;
            {
                four = ( four + one );
                four = ( four + one );
                four = ( four + one );
                four = ( four + one );
            };
            _seqresult_1 = four;
        };
        int four = _seqresult_1;
        ;
    }
}