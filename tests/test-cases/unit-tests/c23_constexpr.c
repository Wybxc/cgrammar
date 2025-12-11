// C23 constexpr
constexpr int const_value = 42;
constexpr double pi = 3.14159;

constexpr int array_size = 10;
int array[array_size];

// constexpr with expressions
constexpr int expr1 = 2 + 2;
constexpr int expr2 = 10 * 20;
constexpr int expr3 = (1 << 10);

// constexpr in struct
struct Constants {
    constexpr int max_size = 100;
    constexpr double ratio = 1.5;
};

void test_constexpr() {
    constexpr int local = 5;
    int arr[local];
}

// constexpr with sizeof
constexpr int int_size = sizeof(int);
constexpr int ptr_size = sizeof(void*);

// constexpr with conditional
constexpr int value = (sizeof(int) == 4) ? 32 : 64;
