// C11 Anonymous Struct/Union Members
struct Point3D {
    union {
        struct {
            float x, y, z;
        };
        float coords[3];
    };
};

void test_point3d() {
    struct Point3D p;
    p.x = 1.0f;
    p.y = 2.0f;
    p.z = 3.0f;
    
    // Or access as array
    p.coords[0] = 1.0f;
}

// Multiple anonymous members
struct Data {
    int id;
    
    struct {
        char name[32];
        int age;
    };
    
    union {
        long long_value;
        double double_value;
    };
    
    struct {
        int flags;
    };
};

// Nested anonymous
struct Complex {
    struct {
        union {
            int i;
            float f;
        };
        int type;
    };
};
