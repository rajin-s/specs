module Helpers
{
    module Addition
    {
        fn Add [x int] [y int] -> int
        {
            x + y
        }

        fn Add1 [x int] -> int
        {
            x + 1
        }

        fn Add10 [x int] -> int
        {
            x + 10
        }
    }
}

(Helpers\Addition\Add 1 2)

use Helpers
let one-plus-two = (Addition\Add 1 2)

use Helpers\Addition
let one = (Add1 0)