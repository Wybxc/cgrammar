// C99 Boolean Type
#include <stdbool.h>

_Bool bool_var;
bool bool_var2;

void test_bool_constants() {
    _Bool b1 = 0;
    _Bool b2 = 1;
    bool b3 = true;
    bool b4 = false;
}

void test_bool_expressions() {
    bool result1 = (5 > 3);
    bool result2 = (2 + 2 == 4);
    bool result3 = (10 < 5);
}

void test_bool_operations() {
    bool a = true;
    bool b = false;
    
    bool and_result = a && b;
    bool or_result = a || b;
    bool not_result = !a;
}

bool is_even(int n) {
    return (n % 2) == 0;
}

bool is_positive(int n) {
    return n > 0;
}

void test_bool_conversions() {
    int x = 42;
    bool b = x;  // Non-zero converts to true
    
    bool zero = 0;
    bool non_zero = 100;
}

struct BoolStruct {
    bool flag1;
    bool flag2;
    _Bool flag3;
};

bool bool_array[10];
