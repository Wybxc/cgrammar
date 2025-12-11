// C23 Enhanced Enumerations with fixed underlying type
enum Color : int {
    RED = 0,
    GREEN = 1,
    BLUE = 2
};

enum LargeValues : long long {
    LARGE_A = 1000000000000LL,
    LARGE_B = 2000000000000LL
};

enum SmallFlags : unsigned char {
    FLAG_A = 0x01,
    FLAG_B = 0x02,
    FLAG_C = 0x04,
    FLAG_D = 0x08
};

// Enum with explicit unsigned type
enum Status : unsigned int {
    STATUS_OK = 0,
    STATUS_ERROR = 1,
    STATUS_PENDING = 2
};

// Forward declaration with underlying type
enum ForwardEnum : int;

void test_enum_types() {
    enum Color c = RED;
    enum LargeValues lv = LARGE_A;
    enum SmallFlags sf = FLAG_A;
}
