// Control Flow Edge Cases
void test_nested_loops() {
    for (int i = 0; i < 10; i++) {
        for (int j = 0; j < 10; j++) {
            for (int k = 0; k < 10; k++) {
                if (i == j && j == k) {
                    continue;
                }
            }
        }
    }
}

void test_break_continue() {
    while (1) {
        break;
    }
    
    for (;;) {
        break;
    }
    
    do {
        break;
    } while (1);
    
    for (int i = 0; i < 10; i++) {
        if (i % 2) continue;
        if (i > 5) break;
    }
}

void test_switch_complex(int x) {
    switch (x) {
        case 1:
        case 2:
        case 3:
            break;
        case 4 ... 10:  // Range case (GCC extension)
            break;
        default:
            break;
    }
}

void test_goto() {
    int i = 0;
start:
    i++;
    if (i < 10) goto start;
    
end:
    return;
}

void test_nested_switch(int x, int y) {
    switch (x) {
        case 1:
            switch (y) {
                case 10:
                    break;
                case 20:
                    break;
            }
            break;
        case 2:
            break;
    }
}

void test_fall_through(int x) {
    switch (x) {
        case 1:
            x++;
            // fallthrough
        case 2:
            x += 2;
            // fallthrough
        case 3:
            x += 3;
            break;
    }
}
