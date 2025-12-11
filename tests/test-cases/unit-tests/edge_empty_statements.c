// Edge Cases - Empty Statements
void test_empty_statements() {
    ;
    ;;
    ;;;
}

void test_empty_blocks() {
    {}
    {{}}
    {{{}}}
}

void test_empty_control() {
    if (1);
    
    while (0);
    
    for (;;);
    
    do ; while(0);
}

void test_empty_switch() {
    int x = 0;
    switch(x) {
    }
    
    switch(x) {
        case 1:;
        case 2:;
    }
}

void test_labels() {
label1:;
label2:;
    goto label1;
}
