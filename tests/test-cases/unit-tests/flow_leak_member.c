#pragma safety enable


void* calloc(int n, int size);

struct X {
    char* name;
};

void* f1()
{
    struct X* p = calloc(1, sizeof(struct X));
    if (p)
    {
        p->name = calloc(1,2);
    }
    return p;
}
#pragma cake diagnostic check "-Wmissing-destructor"
