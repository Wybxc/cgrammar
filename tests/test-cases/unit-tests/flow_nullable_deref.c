#pragma safety enable


void* malloc(int i);
void free(void*);

struct X {
    char* name;
};

int main()
{
    struct X* p = malloc(sizeof(struct X));
    if (p)
    {
        p->name = malloc(1);
    }
    else
    {
        //p->name = malloc(1);
        //#pragma cake diagnostic check "-Wanalyzer-null-dereference"
    }
    free(p->name);
#pragma cake diagnostic check "-Wanalyzer-null-dereference"
#pragma cake diagnostic check "-Wanalyzer-maybe-uninitialized"

    free(p);
#pragma cake diagnostic check "-Wmissing-destructor"

}
