// Edge Cases - Complex Expressions
void test_precedence() {
    int a, b, c, d, e;
    
    // Complex arithmetic
    int x = a + b * c - d / e;
    int y = (a + b) * (c - d) / e;
    
    // Bitwise operations
    int z = a & b | c ^ d;
    int w = (a << 2) | (b >> 3);
    
    // Logical operations
    int p = a && b || c && d;
    int q = !a || !b && !c;
}

void test_ternary() {
    int a, b, c, d;
    
    // Nested ternary
    int x = a ? b : c ? d : 0;
    int y = a ? (b ? c : d) : (c ? d : 0);
    
    // Ternary in ternary
    int z = (a > b) ? (c > d ? c : d) : (a > d ? a : d);
}

void test_comma_operator() {
    int a, b, c;
    int x = (a = 1, b = 2, c = 3);
    int y = (a++, b++, c++, a + b + c);
}

void test_casts() {
    int i = 42;
    float f = (float)i;
    double d = (double)(int)(float)i;
    void *ptr = (void*)(long)(int*)0;
}

void test_sizeof_expressions() {
    int arr[10];
    int x = sizeof(int) + sizeof(arr) + sizeof(arr[0]);
    int y = sizeof(int*) * sizeof(char*);
    int z = sizeof sizeof(int);
}
