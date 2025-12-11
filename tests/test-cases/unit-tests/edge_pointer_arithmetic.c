// Pointer Arithmetic Edge Cases
void test_pointer_arithmetic() {
    int arr[10];
    int *p = arr;
    
    // Basic pointer arithmetic
    p++;
    p--;
    p += 5;
    p -= 3;
    
    // Pointer difference
    int *p1 = &arr[0];
    int *p2 = &arr[5];
    long diff = p2 - p1;
    
    // Pointer comparison
    if (p1 < p2) {}
    if (p1 <= p2) {}
    if (p1 > p2) {}
    if (p1 >= p2) {}
    if (p1 == p2) {}
    if (p1 != p2) {}
}

void test_array_subscript() {
    int arr[10];
    int *p = arr;
    
    // Array subscript
    int x = arr[5];
    int y = p[5];
    int z = 5[arr];  // Valid in C!
}

void test_multidimensional() {
    int matrix[3][4];
    int (*p)[4] = matrix;
    
    int x = matrix[1][2];
    int y = p[1][2];
    int z = (*(p + 1))[2];
    int w = *(*(p + 1) + 2);
}

void test_pointer_to_pointer() {
    int x = 42;
    int *p = &x;
    int **pp = &p;
    int ***ppp = &pp;
    
    int val = ***ppp;
}

void test_void_pointer() {
    int x = 42;
    void *vp = &x;
    int *ip = (int*)vp;
}
