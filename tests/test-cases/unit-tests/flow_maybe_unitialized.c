#pragma safety enable


void* malloc(unsigned long size);

struct X {
    char* text;
};

void x_delete(struct X*  p);

int main()
{
    struct X* p = malloc(sizeof(struct X));
    x_delete(p);
#pragma cake diagnostic check "-Wanalyzer-maybe-uninitialized"
}

