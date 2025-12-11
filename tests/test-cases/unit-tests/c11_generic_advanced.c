// C11 _Generic - Advanced Cases
#include <stddef.h>

#define type_name(x) _Generic((x), \
    int: "int", \
    float: "float", \
    double: "double", \
    char: "char", \
    char*: "string", \
    default: "unknown")

void test_generic_basic() {
    int x = 0;
    float y = 0.0f;
    const char *result1 = type_name(x);
    const char *result2 = type_name(y);
}

// Generic with function selection
int func_int(int x) { return x; }
float func_float(float x) { return x; }
double func_double(double x) { return x; }

#define FUNC(x) _Generic((x), \
    int: func_int, \
    float: func_float, \
    double: func_double)

void test_generic_function() {
    int i = FUNC(10)(5);
    float f = FUNC(1.0f)(2.5f);
}

// Generic with pointers
void test_generic_pointers() {
    int *ip;
    float *fp;
    _Generic(ip, int*: 1, float*: 2, default: 0);
    _Generic(fp, int*: 1, float*: 2, default: 0);
}

// Nested generic
void test_generic_nested(int x) {
    _Generic(x,
        int: _Generic(x, int: 1, default: 2),
        default: 0);
}
