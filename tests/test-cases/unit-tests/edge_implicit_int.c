// Implicit int (legacy C, removed in C99)
// Modern C requires explicit types

// Explicit types (C99+)
int function_with_int(void);
int variable = 42;

// Function with explicit return type
int main(void) {
    return 0;
}

// Explicit int in declarations
int a, b, c;
static int static_var;
extern int extern_var;

// Old style function declarations need explicit types
int old_style_func();

// Modern prototype
int modern_func(int x, int y);
