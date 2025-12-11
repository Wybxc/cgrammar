// C23 Decimal Floating-Point Types
_Decimal32 d32_var;
_Decimal64 d64_var;
_Decimal128 d128_var;

void test_decimal32() {
    _Decimal32 x = 0.0DF;
    _Decimal32 y = 1.5DF;
}

void test_decimal64() {
    _Decimal64 x = 0.0DD;
    _Decimal64 y = 3.14159DD;
}

void test_decimal128() {
    _Decimal128 x = 0.0DL;
    _Decimal128 y = 2.718281828DL;
}

struct DecimalData {
    _Decimal32 small;
    _Decimal64 medium;
    _Decimal128 large;
};

// Decimal arrays
_Decimal64 prices[10];

// Decimal pointers
_Decimal32 *ptr32;
_Decimal64 *ptr64;
_Decimal128 *ptr128;
