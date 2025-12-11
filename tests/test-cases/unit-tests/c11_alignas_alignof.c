// C11 Alignment Specifiers
#include <stdalign.h>

_Alignas(16) int aligned_int;
_Alignas(double) char aligned_char;
_Alignas(32) struct AlignedStruct {
    int x;
    int y;
};

void test_alignas() {
    _Alignas(64) int x;
    _Alignas(128) char buffer[256];
}

void test_alignof() {
    int a = _Alignof(int);
    int b = _Alignof(double);
    int c = _Alignof(struct AlignedStruct);
    int d = alignof(long);
}

// Alignment with arrays
_Alignas(32) int aligned_array[10];

// Alignment with typedef
typedef _Alignas(16) int aligned_int_t;
