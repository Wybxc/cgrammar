// C99 Variable Length Arrays - Edge Cases
void test_vla_basic(int n) {
    int arr[n];
    int matrix[n][n];
}

void test_vla_expression(int n, int m) {
    int arr[n + m];
    int arr2[n * 2];
    int arr3[n > 10 ? n : 10];
}

void test_vla_pointer(int n) {
    int (*ptr)[n];
    int (*matrix)[n][n];
}

void test_vla_sizeof(int n) {
    int arr[n];
    sizeof(arr);
    sizeof(*arr);
}

// VLA in function parameter
void process_array(int n, int arr[n]);
void process_matrix(int rows, int cols, int matrix[rows][cols]);

// VLA with typedef
void test_vla_typedef(int n) {
    typedef int vla_type[n];
    vla_type arr;
}
