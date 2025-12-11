// C23 nullptr
#include <stddef.h>

void test_nullptr_basic() {
    int *ptr = nullptr;
    char *str = nullptr;
}

void test_nullptr_comparison() {
    int *p1 = nullptr;
    if (p1 == nullptr) {
        // handle null
    }
}

void test_nullptr_vs_null() {
    int *p1 = nullptr;
    int *p2 = NULL;
    int *p3 = 0;
}

void accept_pointer(int *ptr);

void test_nullptr_argument() {
    accept_pointer(nullptr);
}

struct Data {
    int *ptr;
};

void test_nullptr_struct() {
    struct Data d = { nullptr };
}
