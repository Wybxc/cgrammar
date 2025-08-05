#pragma safety enable



void free(void* p);
struct X
{
    int i;
    void* p;
};
void f(const struct X* p);
int main()
{
    struct X x = { 0 };
    f(&x);
    static_state(x.p, "null");
}
