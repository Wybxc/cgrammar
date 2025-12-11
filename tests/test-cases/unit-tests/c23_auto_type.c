// C23 auto type (type inference)
void test_auto_basic() {
    auto x = 42;          // int
    auto y = 3.14;        // double
    auto z = 3.14f;       // float
}

void test_auto_pointers() {
    int value = 10;
    auto ptr = &value;    // int*
    auto str = "hello";   // char*
}

void test_auto_arrays() {
    int arr[5] = {1, 2, 3, 4, 5};
    auto p = arr;         // int*
}

struct Point {
    int x, y;
};

void test_auto_struct() {
    struct Point p = {1, 2};
    auto p2 = p;          // struct Point
}

void test_auto_const() {
    const int ci = 42;
    auto x = ci;          // int (not const int in C23)
}

void test_auto_expressions() {
    auto sum = 1 + 2;     // int
    auto product = 2.0 * 3.0;  // double
}

// auto with compound literals
void test_auto_compound() {
    auto p = (struct Point){1, 2};
}
