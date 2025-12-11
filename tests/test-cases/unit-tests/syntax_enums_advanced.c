// Advanced Enum Cases
enum SimpleEnum {
    FIRST,
    SECOND,
    THIRD
};

// Enum with explicit values
enum ExplicitEnum {
    E_ZERO = 0,
    E_ONE = 1,
    E_TEN = 10,
    E_HUNDRED = 100
};

// Enum with expressions
enum ExprEnum {
    EX_A = 1,
    EX_B = EX_A + 1,
    EX_C = EX_B * 2,
    EX_D = (1 << 5)
};

// Enum with negative values
enum SignedEnum {
    NEG = -1,
    ZERO = 0,
    POS = 1
};

// Trailing comma
enum TrailingComma {
    TC_ONE,
    TC_TWO,
    TC_THREE,
};

// Anonymous enum
enum {
    ANON_ONE,
    ANON_TWO
};

// Enum forward declaration
enum ForwardEnum;

// Enum with typedef
typedef enum {
    TD_FIRST,
    TD_SECOND
} TypedefEnum;

// Enum with very large values
enum LargeValues {
    LARGE_A = 0x7FFFFFFF,
    LARGE_B = 0xFFFFFFFF,
};
