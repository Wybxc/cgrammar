#pragma safety enable


void free(void* p);
struct X
{
    int i;
    void* p;
};
void f(struct X* p);
int main()
{
    struct X x = { 0 };
    static_state(x.p, "null");
    f(&x);
    static_state(x.p, "null | not-null");
    free(x.p);
}
