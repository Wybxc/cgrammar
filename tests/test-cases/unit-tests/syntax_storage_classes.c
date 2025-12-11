// Storage Class Specifiers
extern int extern_var;
static int static_var;
auto int auto_var;
register int register_var;

// Combinations with type qualifiers
extern const int extern_const;
static volatile int static_volatile;

// Function storage classes
extern void extern_func(void);
static void static_func(void);
inline void inline_func(void);

// Static in block scope
void test_static_local() {
    static int counter = 0;
    static const int const_counter = 0;
}

// Extern in block scope
void test_extern_local() {
    extern int global_var;
    extern void external_function(void);
}

// Register with auto
void test_register() {
    register int fast_var;
    register char *fast_ptr;
}

// Inline definitions
inline int inline_definition(int x) {
    return x * 2;
}

extern inline int extern_inline_func(void);
static inline int static_inline_func(void) { return 42; }
