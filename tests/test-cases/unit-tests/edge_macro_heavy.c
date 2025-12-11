// Edge Cases - Macro-Heavy Code
#define CONCAT(a, b) a##b
#define STRINGIFY(x) #x
#define EXPAND(x) x

#define MAX(a, b) ((a) > (b) ? (a) : (b))
#define MIN(a, b) ((a) < (b) ? (a) : (b))
#define CLAMP(x, min, max) MIN(MAX(x, min), max)

#define ARRAY_SIZE(arr) (sizeof(arr) / sizeof((arr)[0]))

#define FOREACH(i, n) for(int i = 0; i < (n); i++)

void test_macros() {
    int x = 10, y = 20;
    int max = MAX(x, y);
    int min = MIN(x, y);
    int clamped = CLAMP(15, 0, 100);
    
    int arr[10];
    int size = ARRAY_SIZE(arr);
    
    FOREACH(i, 10) {
        arr[i] = i;
    }
}

// Multi-line macros
#define COMPLEX_MACRO(x, y) \
    do { \
        int temp = (x); \
        (x) = (y); \
        (y) = temp; \
    } while(0)

// Variadic macros
#define DEBUG(fmt, ...) printf(fmt, ##__VA_ARGS__)

void test_variadic() {
    DEBUG("Hello\n");
    DEBUG("Value: %d\n", 42);
    DEBUG("Two values: %d, %d\n", 1, 2);
}
