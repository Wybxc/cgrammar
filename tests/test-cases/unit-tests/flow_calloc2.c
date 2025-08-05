#pragma safety enable



struct X { int i; void* p; };
void* calloc(int i, int sz);
void free(void* p);

int main()
{
    struct X* p = calloc(1, 1);
    static_state(p, "null | not-null ");
    if (p)
    {
    static_state(p->i, "zero");
    static_state(p->p, "null");
    }
    free(p);
}
