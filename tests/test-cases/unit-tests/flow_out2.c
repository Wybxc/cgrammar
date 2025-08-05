#pragma safety enable


void* malloc(unsigned long size);
void free(void* ptr);

struct Y {
    char* p0;
    int* p2;
    double i2;
};

struct X {
    char* text;
    int* p1;
    int i;
    struct Y* pY;
};

void f(const struct X* p);
void destroy(struct X *  p);

int main()
{
    struct X x = {0};
    f(&x);

    static_state(x.p1, "null ");
    static_state(x.i, "zero");
    static_state(x.pY, "null ");

    destroy(&x);
}
