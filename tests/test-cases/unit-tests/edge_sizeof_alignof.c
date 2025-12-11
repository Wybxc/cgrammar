// sizeof and alignof Edge Cases
#include <stddef.h>

void test_sizeof_types() {
    // Basic types
    size_t s1 = sizeof(char);
    size_t s2 = sizeof(int);
    size_t s3 = sizeof(long);
    size_t s4 = sizeof(long long);
    size_t s5 = sizeof(float);
    size_t s6 = sizeof(double);
    size_t s7 = sizeof(void*);
}

void test_sizeof_expressions() {
    int x = 42;
    size_t s1 = sizeof(x);
    size_t s2 = sizeof x;  // Without parentheses
    size_t s3 = sizeof(x + 1);
    size_t s4 = sizeof(int*);
}

void test_sizeof_arrays() {
    int arr[10];
    size_t s1 = sizeof(arr);
    size_t s2 = sizeof(arr[0]);
    size_t count = sizeof(arr) / sizeof(arr[0]);
}

struct TestStruct {
    int a;
    char b;
    double c;
};

void test_sizeof_structs() {
    size_t s1 = sizeof(struct TestStruct);
    struct TestStruct ts;
    size_t s2 = sizeof(ts);
    size_t s3 = sizeof ts;
}

void test_sizeof_nested() {
    // Nested sizeof
    size_t s1 = sizeof(sizeof(int));
    size_t s2 = sizeof(sizeof(sizeof(int)));
}

void test_alignof() {
    size_t a1 = _Alignof(int);
    size_t a2 = _Alignof(double);
    size_t a3 = _Alignof(struct TestStruct);
    size_t a4 = _Alignof(char);
}

void test_sizeof_vla(int n) {
    int vla[n];
    size_t s = sizeof(vla);  // Evaluated at runtime
}
