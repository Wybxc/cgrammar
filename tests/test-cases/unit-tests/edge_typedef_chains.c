// Typedef Chain Edge Cases
typedef int int_t;
typedef int_t int_t2;
typedef int_t2 int_t3;

// Pointer typedefs
typedef int *int_ptr_t;
typedef int_ptr_t *int_ptr_ptr_t;

// Array typedefs
typedef int int_array_t[10];
typedef int_array_t int_matrix_t[5];

// Function pointer typedefs
typedef int (*func_ptr_t)(int, int);
typedef func_ptr_t (*func_ptr_ptr_t)(void);

// Struct typedefs
typedef struct Point {
    int x, y;
} Point_t;

typedef Point_t *PointPtr_t;

// Anonymous struct typedef
typedef struct {
    int id;
    char name[32];
} Record_t;

// Union typedefs
typedef union {
    int i;
    float f;
} Value_t;

// Enum typedefs
typedef enum {
    RED, GREEN, BLUE
} Color_t;

// Complex typedef combinations
typedef const int const_int_t;
typedef volatile int volatile_int_t;
typedef const volatile int cv_int_t;

typedef int (*compare_func_t)(const void *, const void *);
typedef compare_func_t (*compare_factory_t)(void);

// Typedef with qualifiers (storage class not valid with typedef)
typedef const int const_typedef_int_t;
