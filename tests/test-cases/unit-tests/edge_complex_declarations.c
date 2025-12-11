// Edge Cases - Complex Declarations
// Function pointer arrays
void (*func_ptr_array[10])(int, int);

// Array of pointers to functions
int (*arr_func_ptr[5])(void);

// Pointer to array of function pointers
void (**ptr_to_arr_func_ptr[10])(void);

// Function returning pointer to function
int (*get_function(void))(int);

// Function taking function pointer
void process(int (*callback)(int, int));

// Complex typedef chains
typedef int *int_ptr;
typedef int_ptr *int_ptr_ptr;
typedef int_ptr_ptr int_ptr_ptr_array[10];

// Multi-dimensional arrays
int matrix[3][4][5];
int (*ptr_to_matrix)[4][5];

// Volatile and const combinations
const volatile int cv_int;
volatile const int vc_int;
const int * const const_ptr_const_int;
volatile int * const const_ptr_volatile_int;

// Restrict pointers (C99)
void func(int * restrict ptr1, int * restrict ptr2);
