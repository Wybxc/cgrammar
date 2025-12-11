// C23 _BitInt Types
_BitInt(8) small_int;
_BitInt(16) medium_int;
_BitInt(32) normal_int;
_BitInt(64) large_int;
_BitInt(128) huge_int;

// Unsigned bit-precise integers
unsigned _BitInt(8) u_small;
unsigned _BitInt(256) u_huge;

void test_bitint() {
    _BitInt(7) x = 0;
    _BitInt(15) y = 100;
    unsigned _BitInt(12) z = 4095;
}

struct BitIntStruct {
    _BitInt(5) small;
    _BitInt(20) medium;
    unsigned _BitInt(9) flags;
};

// BitInt in arrays
_BitInt(10) bit_array[5];

// BitInt pointers
_BitInt(17) *ptr;
unsigned _BitInt(33) *uptr;
