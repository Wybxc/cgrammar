// Inline assembly (common extension)
// Note: Syntax varies by compiler, shown for parser awareness

void test_asm_basic() {
    // GCC-style inline assembly
    __asm__("nop");
    __asm__ __volatile__("nop");
}

void test_asm_with_constraints() {
    int input = 42;
    int output;
    
    // Extended asm with constraints
    __asm__("movl %1, %0" : "=r"(output) : "r"(input));
}

void test_asm_keyword() {
    // Some compilers support 'asm' keyword
    asm("nop");
}

// Function-level asm
void naked_function(void) __attribute__((naked));
