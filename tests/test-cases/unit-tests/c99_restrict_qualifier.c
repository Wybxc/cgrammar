// C99 restrict qualifier
void copy_array(int * restrict dest, const int * restrict src, int n);

void test_restrict() {
    int arr1[10];
    int arr2[10];
    copy_array(arr2, arr1, 10);
}

// restrict with pointers to pointers
void process(int ** restrict pp);

// restrict in function parameters
void func(int * restrict p1, int * restrict p2, int * restrict p3);

// restrict with const
void read_data(const int * restrict data, int size);

// restrict in typedef
typedef int * restrict int_restrict_ptr;

// restrict with struct members
struct Buffer {
    int * restrict data;
    int size;
};

// restrict in array parameters
void process_matrix(int (* restrict matrix)[10], int rows);

// Multiple restrict pointers
void swap(int * restrict a, int * restrict b) {
    int temp = *a;
    *a = *b;
    *b = temp;
}
