// C99 Inline Functions
inline int add(int a, int b) {
    return a + b;
}

inline static int static_inline(int x) {
    return x * 2;
}

extern inline int extern_inline(int x);

inline int extern_inline(int x) {
    return x * 3;
}

// Inline function with multiple statements
inline int max(int a, int b) {
    if (a > b) {
        return a;
    } else {
        return b;
    }
}

// Inline function calling another inline
inline int process(int x) {
    return add(max(x, 0), 10);
}

// Inline with various return points
inline int clamp(int value, int min, int max) {
    if (value < min) return min;
    if (value > max) return max;
    return value;
}

// Inline with loops
inline int sum_range(int n) {
    int total = 0;
    for (int i = 0; i <= n; i++) {
        total += i;
    }
    return total;
}
