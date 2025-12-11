// C99 Compound Literals
#include <stddef.h>

struct Point {
    int x, y;
};

void test_compound_literal_struct() {
    struct Point p = (struct Point){1, 2};
    struct Point *ptr = &(struct Point){3, 4};
}

void test_compound_literal_array() {
    int *arr = (int[]){1, 2, 3, 4, 5};
    int sum = ((int[]){1, 2, 3})[0] + ((int[]){4, 5, 6})[1];
}

void test_compound_literal_nested() {
    struct Point points[] = {
        (struct Point){0, 0},
        (struct Point){1, 1},
        (struct Point){2, 2}
    };
}

void test_compound_literal_const() {
    const struct Point *p = &(const struct Point){10, 20};
}

// Compound literal in function call
void process_point(struct Point p);
void test_compound_literal_call() {
    process_point((struct Point){5, 10});
}
