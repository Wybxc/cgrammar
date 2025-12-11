// C11 Static Assertions
#include <assert.h>

_Static_assert(sizeof(int) >= 4, "int must be at least 4 bytes");
_Static_assert(sizeof(char) == 1, "char must be 1 byte");

struct TestStruct {
    int a;
    char b;
};

_Static_assert(sizeof(struct TestStruct) >= 5, "struct size check");

void test_static_assert_function() {
    _Static_assert(1, "always true");
    _Static_assert(sizeof(long) >= sizeof(int), "long >= int");
}

// Static assert with expressions
_Static_assert(2 + 2 == 4, "math check");
_Static_assert((1 << 10) == 1024, "bit shift check");

// Static assert without message (C23)
_Static_assert(1);
