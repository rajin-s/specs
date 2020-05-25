#include "specs_runtime.h"
int _USER_MAIN()
{
    {
        int _xcond_1;
        if ( true )
        {
            _xcond_1 = 1;
        }
        else
        {
            _xcond_1 = 2;
        };
        int x = _xcond_1;
        ;
        int y = 100;
        {
            int* _xcond_2 = ( &y );
            if ( false )
            {
                ( *_xcond_2 ) = 3;
            }
            else
            {
                ( *_xcond_2 ) = 4;
            };
        };
        int a = 100;
        int b = 100;
        {
            int* _xcond_3;
            if ( false )
            {
                int* _xseq_1;
                int _xcond_4;
                if ( true )
                {
                    _xcond_4 = a;
                }
                else
                {
                    _xcond_4 = b;
                };
                _xseq_1 = ( &_xcond_4 );
                ;
                _xcond_3 = _xseq_1;
                ;
            }
            else
            {
                int* _xseq_2;
                int _xcond_5;
                if ( true )
                {
                    _xcond_5 = b;
                }
                else
                {
                    _xcond_5 = a;
                };
                _xseq_2 = ( &_xcond_5 );
                ;
                _xcond_3 = _xseq_2;
                ;
            };
            ( *_xcond_3 ) = 200;
        };
        {
            int _xcond_6;
            if ( true )
            {
                _xcond_6 = 1;
            }
            else
            {
                _xcond_6 = 2;
            };
            int _xcond_7;
            if ( false )
            {
                _xcond_7 = 2;
            }
            else
            {
                _xcond_7 = 1;
            };
            FOO( _xcond_6, _xcond_7 );
        };
        {
            int _xcond_8;
            if ( true )
            {
                {
                    int* _xcond_9 = ( &_xcond_8 );
                    if ( false )
                    {
                        ( *_xcond_9 ) = 1;
                    }
                    else
                    {
                        ( *_xcond_9 ) = 2;
                    };
                };
            }
            else
            {
                {
                    int* _xcond_10 = ( &_xcond_8 );
                    if ( true )
                    {
                        ( *_xcond_10 ) = 3;
                    }
                    else
                    {
                        ( *_xcond_10 ) = 4;
                    };
                };
            };
            FOO( 1, _xcond_8 );
        };
        {
            bool _xcond_13 = true;
            int _xcond_11  = 1;
            int _xcond_12  = 2;
            int _xcond_14;
            if ( _xcond_13 )
            {
                _xcond_14 = FOO( _xcond_11, _xcond_12 );
            }
            else
            {
                _xcond_14 = BAR( _xcond_11, _xcond_12 );
            };
            _xcond_14;
        };
        if ( true )
        {
            {
                bool _xcond_19 = ( FOO( 1, 2 ) > 100 );
                int _xcond_16;
                if ( ( BAR( 3, 4 ) < 0 ) )
                {
                    _xcond_16 = 10;
                }
                else
                {
                    _xcond_16 = 20;
                };
                int _xcond_15 = _xcond_16;
                ;
                int _xcond_18;
                if ( ( BAR( FOO( 5, 6 ), 7 ) > 999 ) )
                {
                    _xcond_18 = 0;
                }
                else
                {
                    _xcond_18 = 1;
                };
                int _xcond_17 = _xcond_18;
                ;
                int _xcond_20;
                if ( _xcond_19 )
                {
                    _xcond_20 = FOO( _xcond_15, _xcond_17 );
                }
                else
                {
                    _xcond_20 = BAR( _xcond_15, _xcond_17 );
                };
                _xcond_20;
            };
        }
        else
        {
            2;
        };
        {
            bool _xcond_23 = ( 1 > 2 );
            bool _xcond_21 = true;
            bool _xcond_22 = false;
            bool _xcond_24;
            if ( _xcond_23 )
            {
                _xcond_24 = ( _xcond_21 && _xcond_22 );
            }
            else
            {
                _xcond_24 = ( _xcond_21 ^ _xcond_22 );
            };
            _xcond_24;
        };
    }
}