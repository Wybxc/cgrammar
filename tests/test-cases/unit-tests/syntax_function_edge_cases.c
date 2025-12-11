// Function Declaration Edge Cases
// Old-style K&R function declarations
int old_style();

// Empty parameter list vs void
void func1(void);
void func2();

// Variadic functions
void variadic1(int x, ...);
void variadic2(const char *fmt, ...);

// Function with array parameters
void array_param(int arr[]);
void array_param2(int arr[10]);
void array_param3(int arr[*]);

// Function with VLA parameters
void vla_param(int n, int arr[n]);
void vla_param2(int rows, int cols, int matrix[rows][cols]);

// Static array parameters (C99)
void static_array(int arr[static 10]);
void static_array2(int arr[const 10]);
void static_array3(int arr[static const 10]);

// Function pointers as parameters
void callback_param(void (*callback)(int));
void callback_param2(int (*compare)(const void*, const void*));

// Inline functions
inline void inline_func(void);
inline int inline_with_return(void) { return 0; }

// Complex return types
int (*complex_return(void))[10];
