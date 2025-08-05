#pragma safety enable


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

struct X f();
void destroy(struct X *  x);

int main()
{
    struct X x;
    x = f();
    static_state(x.text, "not-null ");
    static_state(x.p1, "not-null ");
    static_state(x.i, "zero | not-zero");
    static_state(x.pY, "null | not-null");
    static_state(x.pY->p0, "not-null ");
    destroy(&x);
}
