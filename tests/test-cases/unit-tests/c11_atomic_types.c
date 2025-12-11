// C11 Atomic Types
_Atomic int atomic_int;
_Atomic _Bool atomic_bool;
_Atomic(int) atomic_int2;
_Atomic(long) atomic_long;

struct AtomicStruct {
    _Atomic int counter;
    _Atomic _Bool flag;
};

void test_atomic_operations() {
    _Atomic int x = 0;
    _Atomic(int) y = 10;
}

void test_atomic_pointer() {
    _Atomic(int*) ptr;
    _Atomic char* atomic_ptr;
}

// Atomic type qualifiers
void test_atomic_qualified() {
    const _Atomic int const_atomic;
    volatile _Atomic int volatile_atomic;
}
