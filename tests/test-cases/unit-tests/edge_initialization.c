// Initialization Edge Cases
// Zero initialization
int zero_int = 0;
int zero_array[10] = {0};
int partial_array[10] = {1, 2, 3};

// Aggregate initialization
struct Point {
    int x, y;
};

struct Point p1 = {1, 2};
struct Point p2 = {.x = 1, .y = 2};
struct Point p3 = {.y = 2, .x = 1};

// Array initialization
int arr1[] = {1, 2, 3, 4, 5};
int arr2[5] = {1, 2, 3};
int arr3[5] = {[0] = 1, [4] = 5};

// String initialization
char str1[] = "hello";
char str2[10] = "hello";
char str3[] = {'h', 'e', 'l', 'l', 'o', '\0'};

// Union initialization
union Data {
    int i;
    float f;
    char c;
};

union Data u1 = {42};
union Data u2 = {.f = 3.14f};

// Nested initialization
struct Outer {
    int x;
    struct Point p;
    int arr[3];
};

struct Outer outer1 = {1, {2, 3}, {4, 5, 6}};
struct Outer outer2 = {
    .x = 1,
    .p = {.x = 2, .y = 3},
    .arr = {4, 5, 6}
};

// Static initialization
static int static_var = 42;
static int static_array[5] = {1, 2, 3, 4, 5};
