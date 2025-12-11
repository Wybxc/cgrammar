// C23 unreachable() macro
#include <stddef.h>

void test_unreachable(int x) {
    switch (x) {
        case 1:
            return;
        case 2:
            return;
        default:
            // unreachable();
            return;  // Using return for now
    }
}

int get_value(int type) {
    if (type == 0) {
        return 42;
    } else if (type == 1) {
        return 100;
    } else {
        // Should never reach here
        // unreachable();
        return -1;  // Using return for compatibility
    }
}

void handle_enum(enum Status { OK, ERROR } status) {
    switch (status) {
        case OK:
            break;
        case ERROR:
            break;
    }
    // If we reach here, something is wrong
}
