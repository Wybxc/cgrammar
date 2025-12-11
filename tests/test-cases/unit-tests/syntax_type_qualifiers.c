// Type Qualifier Edge Cases
const int const_int = 42;
volatile int volatile_int;
restrict int * restrict restrict_ptr;

// Multiple qualifiers
const volatile int cv_int;
volatile const int vc_int;

// Qualified pointers
const int *ptr_to_const;
int * const const_ptr;
const int * const const_ptr_to_const;

// Restrict with const and volatile
int * restrict const restrict_const_ptr;
const int * restrict restrict_ptr_to_const;

// Arrays with qualifiers
const int const_array[10];
volatile int volatile_array[10];

// Function parameters with qualifiers
void func1(const int x);
void func2(volatile int *ptr);
void func3(int * restrict ptr1, int * restrict ptr2);

// Struct members with qualifiers
struct QualifiedMembers {
    const int const_member;
    volatile int volatile_member;
};

// Typedef with qualifiers
typedef const int const_int_t;
typedef volatile int volatile_int_t;

// Qualified function pointers
void (*const const_func_ptr)(void);
const int (*func_ptr_const_return)(void);
