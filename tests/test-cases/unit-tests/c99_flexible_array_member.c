// C99 Flexible Array Members
struct Buffer {
    int size;
    char data[];
};

struct Matrix {
    int rows;
    int cols;
    int data[];
};

struct String {
    int length;
    char str[];
};

// Flexible array must be last member
struct Valid {
    int a;
    double b;
    int flex[];
};
