// C23 Empty Initializer
struct Point {
    int x;
    int y;
};

void test_empty_init() {
    // Empty initializer (C23)
    struct Point p = {};
    int arr[5] = {};
}

struct Nested {
    int a;
    struct Point p;
    int arr[3];
};

void test_nested_empty() {
    struct Nested n = {};
}

union Data {
    int i;
    float f;
};

void test_union_empty() {
    union Data d = {};
}

// Arrays
void test_array_empty() {
    int arr1[10] = {};
    double arr2[5] = {};
    char arr3[100] = {};
}

// With typedef
typedef struct {
    int id;
    char name[32];
} Record;

void test_typedef_empty() {
    Record r = {};
}
