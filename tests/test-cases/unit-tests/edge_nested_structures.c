// Edge Cases - Deeply Nested Structures
struct Level1 {
    int x;
    struct Level2 {
        int y;
        struct Level3 {
            int z;
            struct Level4 {
                int w;
            } l4;
        } l3;
    } l2;
};

// Nested unions
union OuterUnion {
    int i;
    union InnerUnion {
        float f;
        double d;
    } inner;
};

// Anonymous structs and unions
struct Container {
    int id;
    union {
        int i_value;
        float f_value;
        struct {
            int x, y;
        };
    };
};

// Struct with bit fields
struct BitFields {
    unsigned int flag1 : 1;
    unsigned int flag2 : 1;
    unsigned int value : 6;
    unsigned int : 0;  // padding
    unsigned int next : 8;
};

// Self-referential struct
struct Node {
    int data;
    struct Node *next;
    struct Node *prev;
};
