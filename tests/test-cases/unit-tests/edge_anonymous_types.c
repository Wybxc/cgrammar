// Anonymous struct and union members (C11)
struct Outer {
    int id;
    
    // Anonymous struct
    struct {
        int x;
        int y;
    };
    
    // Anonymous union
    union {
        int i_value;
        float f_value;
        char c_value;
    };
    
    // Named struct for comparison
    struct Inner {
        int a;
        int b;
    } inner;
};

void test_anonymous() {
    struct Outer o;
    
    // Access anonymous struct members directly
    o.x = 10;
    o.y = 20;
    
    // Access anonymous union members directly
    o.i_value = 42;
    o.f_value = 3.14f;
    
    // Access named struct members
    o.inner.a = 5;
}

// Anonymous union in file scope
union {
    int global_int;
    float global_float;
};

// Nested anonymous
struct Nested {
    struct {
        struct {
            int deep_value;
        };
    };
};
