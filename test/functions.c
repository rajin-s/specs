#include "specs_runtime.h"

// Function Declarations

int Add__10(int x);
bool Is__Even(int x);
bool Is__Seven(int x);
int Add(int a, int b);
int Foo();

// Function Definitions

int Add__10(int x) { return((x + 10)); }
bool Is__Even(int x) { if ((x == 0)) {return(true);} else {if ((x < 0)) {return(false);} else {return(Is__Even((x + -2)));};}; }
bool Is__Seven(int x) { if ((x == 6)) {return(false);} else {if ((x == 8)) {return(false);} else {if (Is__Even(x)) {return(false);} else {return((2 == (x + -5)));};};}; }
int Add(int a, int b) { return((a + b)); }

// Program

int _USER_MAIN(){ { /* function Add-10 */;int x = Add__10(1);int y = Add__10(3);int z = Add__10(Add__10(4));/* function Is-Even */;bool seven__is__even = Is__Even(7);/* function Is-Seven */;bool seven__is__seven = Is__Seven(7);bool foo = Is__Seven(Add__10(-3));/* function Add */;return(Add__10(Add(5, 1)));/* function Foo */; }; }