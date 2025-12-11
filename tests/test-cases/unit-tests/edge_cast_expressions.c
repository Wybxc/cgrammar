// Cast Expression Edge Cases
void test_basic_casts() {
    int i = 42;
    float f = (float)i;
    double d = (double)i;
    char c = (char)i;
}

void test_pointer_casts() {
    int x = 42;
    void *vp = (void*)&x;
    int *ip = (int*)vp;
    char *cp = (char*)&x;
}

void test_cast_chains() {
    int i = 42;
    double d = (double)(float)(long)i;
    void *p = (void*)(long)(int)i;
}

void test_const_casts() {
    const int ci = 42;
    int *p = (int*)&ci;  // Casting away const
    
    volatile int vi = 10;
    int *p2 = (int*)&vi;  // Casting away volatile
}

void test_function_pointer_casts() {
    void (*fp1)(void);
    int (*fp2)(int);
    fp1 = (void(*)(void))fp2;
}

void test_array_to_pointer_cast() {
    int arr[10];
    int *p = (int*)arr;
}

struct A { int x; };
struct B { float y; };

void test_struct_casts() {
    struct A a = {42};
    struct B *bp = (struct B*)&a;
}

void test_cast_in_expressions() {
    int result = (int)3.14 + (int)2.71;
    int shifted = ((int)1.5) << 2;
}

void test_cast_with_sizeof() {
    int size = (int)sizeof(double);
}
