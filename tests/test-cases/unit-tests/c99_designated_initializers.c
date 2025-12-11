// C99 Designated Initializers - Extended Cases
struct Complex {
    int a;
    double b;
    char c;
    int arr[5];
};

void test_designated_struct() {
    struct Complex c1 = {.a = 10, .b = 20.5, .c = 'x'};
    struct Complex c2 = {.c = 'y', .a = 5};
    struct Complex c3 = {.arr = {1, 2, 3}, .a = 100};
    struct Complex c4 = {.arr[2] = 42};
}

void test_designated_array() {
    int arr1[10] = {[0] = 1, [9] = 10};
    int arr2[5] = {[2] = 5, [4] = 10, [1] = 3};
    int arr3[] = {[0] = 1, [5] = 6};
}

void test_designated_nested() {
    struct Complex arr[3] = {
        [0] = {.a = 1, .b = 2.0},
        [2] = {.a = 3, .b = 4.0}
    };
}

void test_designated_mixed() {
    struct Complex c = {
        .a = 10,
        .arr = {[1] = 5, [3] = 7}
    };
}

void test_designated_range() {
    int arr[100] = {[0 ... 9] = 1, [10 ... 19] = 2};
}
