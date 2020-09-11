#include <stdio.h>
#include <stdlib.h>

typedef enum
{
    __TypeID_Dog,
    __TypeID_Cat,
    __TypeID_Person,
    __TypeID_Rock,
} __TypeID;

typedef struct
{
    __TypeID _;
    int dogID;
} Dog;
char* GetSpecies__Animal__Dog( Dog* a )
{
    return "canis familiaris";
}
void SayHello__Pet__Dog( Dog* p )
{
    printf( "woof\n" );
}

typedef struct
{
    __TypeID _;
    int catID;
} Cat;
char* GetSpecies__Animal__Cat( Cat* a )
{
    return "felis catus";
}
void SayHello__Pet__Cat( Cat* p )
{
    printf( "meow\n" );
}

typedef struct
{
    __TypeID _;
    char* name;
} Person;
char* GetSpecies__Animal__Person( Person* a )
{
    return "homo spaiens";
}

typedef struct
{
    __TypeID _;
    char* name;

    enum
    {
        Rock__Sedimentary,
        Rock__Igneous,
        Rock__Metamorphic
    } rockType;
} Rock;
void SayHello__Pet__Rock( Rock* p )
{
    printf( "...\n" );
}

typedef union
{
    struct
    {
        __TypeID __type;
    } __as_animal;

    Dog __as_dog;
    Cat __as_cat;
    Person __as_person;
} Animal;
inline char* GetSpecies__Animal( Animal* a )
{
    switch ( a->__as_animal.__type )
    {
        case __TypeID_Dog:
            return GetSpecies__Animal__Dog( &a->__as_dog );
            break;
        case __TypeID_Cat:
            return GetSpecies__Animal__Cat( &a->__as_cat );
            break;
        case __TypeID_Person:
            return GetSpecies__Animal__Person( &a->__as_person );
            break;

        default:
            exit(1);
            break;
    }
}

typedef union
{
    struct
    {
        __TypeID __type;
    } __as_pet;

    Dog __as_dog;
    Cat __as_cat;
    Rock __as_rock;
} Pet;
inline void SayHello__Pet( Pet* p )
{
    switch ( p->__as_pet.__type )
    {
        case __TypeID_Dog:
            SayHello__Pet__Dog( &p->__as_dog );
            break;
        case __TypeID_Cat:
            SayHello__Pet__Cat( &p->__as_cat );
            break;
        case __TypeID_Rock:
            SayHello__Pet__Rock( &p->__as_rock );
            break;

        default:
            exit(1);
            break;
    }
}

void PrintDog( Dog* d )
{
    printf( "Dog id=%d\n", d->dogID );
}

#define SHOW(e, f) printf(#e " = " f "\n", e);

int main()
{
    SHOW(sizeof(Dog),    "%llu");
    SHOW(sizeof(Cat),    "%llu");
    SHOW(sizeof(Person), "%llu");
    SHOW(sizeof(Rock),   "%llu");
    SHOW(sizeof(Animal), "%llu");
    SHOW(sizeof(Pet),    "%llu");

    Dog d = { __TypeID_Dog, 100 };
    PrintDog( &d );

    SayHello__Pet( (Pet*) &d );
    
    char* spec = GetSpecies__Animal( (Animal*) &d );
    printf( "%s\n", spec );

    return 0;
}