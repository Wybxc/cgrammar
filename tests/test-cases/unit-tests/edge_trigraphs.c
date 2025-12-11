// Trigraph sequences (legacy, removed in C23 but still in parser context)
// These are typically handled by preprocessor but shown for completeness

void test_basic_syntax() {
    // Regular array syntax
    int arr[10];
    arr[0] = 1;
    
    // Braces
    if (1) {
        int x = 0;
    }
    
    // Bitwise operations
    int a = 5;
    int b = a ^ 3;
    int c = ~a;
    int d = a | b;
}

// Comments work normally
/* This is a comment */
// This is also a comment
