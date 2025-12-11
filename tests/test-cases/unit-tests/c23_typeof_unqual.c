// C23 typeof and typeof_unqual
void test_typeof() {
    int x = 10;
    typeof(x) y = 20;
    typeof(int) z = 30;
}

void test_typeof_unqual() {
    const int x = 10;
    typeof_unqual(x) y = 20;  // y is int, not const int
    
    volatile int v = 5;
    typeof_unqual(v) w = 10;  // w is int, not volatile int
}

void test_typeof_complex() {
    int arr[5];
    typeof(arr) arr2;  // arr2 is int[5]
    
    int *ptr;
    typeof(ptr) ptr2;  // ptr2 is int*
}

struct Point {
    int x, y;
};

void test_typeof_struct() {
    struct Point p1 = {1, 2};
    typeof(p1) p2 = {3, 4};
    typeof(struct Point) p3 = {5, 6};
}

// typeof with expressions
void test_typeof_expressions() {
    typeof(1 + 2) result1;
    typeof(1.0 + 2.0) result2;
}
