// Statement Expression Edge Cases (GNU extension, common in C)
void test_labels_and_cases() {
    int x = 0;
    
    // Label at end of block
    {
        x = 1;
    label_end:;
    }
    
    // Multiple labels on same statement
label1:
label2:
label3:
    x = 2;
    
    // Case labels
    switch(x) {
        case 0:
        case 1:
        case 2:
            break;
    }
}

void test_block_scope() {
    // Nested blocks
    {
        int x = 1;
        {
            int x = 2;
            {
                int x = 3;
            }
        }
    }
}

void test_declaration_statements() {
    int a = 1, b = 2, c = 3;
    int *p = &a, **pp = &p;
    
    // Declaration after statements (C99)
    a = b + c;
    int d = a * 2;
    
    // For loop declarations (C99)
    for (int i = 0; i < 10; i++) {
        int j = i * 2;
    }
}

void test_mixed_declarations() {
    int x;
    x = 10;
    int y = 20;
    y += x;
    int z = x + y;
}
