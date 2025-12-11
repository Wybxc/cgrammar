// C99 Mixed Declarations and Statements
void test_mixed_decl() {
    int a = 1;
    a++;
    
    // Declaration after statement (C99)
    int b = a * 2;
    b += 10;
    
    // More statements
    int c = b + a;
    
    // For loop with declaration
    for (int i = 0; i < 10; i++) {
        int j = i * 2;
        j++;
    }
    
    // Another declaration
    int d = c + b + a;
}

void test_nested_blocks() {
    int x = 1;
    {
        int y = 2;
        x++;
        int z = x + y;
    }
    x++;
    int w = x;
}

void test_switch_declarations() {
    int x = 5;
    
    switch (x) {
        case 1: {
            int a = 10;
            break;
        }
        case 2: {
            int b = 20;
            break;
        }
    }
}
