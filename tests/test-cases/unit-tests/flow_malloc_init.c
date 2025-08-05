#pragma safety enable


struct X
{
    int i;
    void *p;
};
void *malloc(int i, int sz);
void free(void *p);

int main()
{
    struct X *p = malloc(1, 1);
    static_state(p, "null | not-null ");
    if (p)
    {
        static_state(p->i, "uninitialized");
        static_state(p->p, "uninitialized");
    }
    free(p);
}
