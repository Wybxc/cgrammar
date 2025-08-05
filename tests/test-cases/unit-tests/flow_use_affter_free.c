#pragma safety enable


char* strdup(const char* s);
void* malloc(unsigned size);

void free(void* ptr);

struct X {
    char* name;
};

void x_destroy(struct X*  p) {
    free(p->name);
}

void x_print(struct X* p)
{
    //printf("%s", p->name);
}

int main() {
    struct X x = { 0 };
    x.name = strdup("a");
    x_destroy(&x);
    x_print(&x);
    #pragma cake diagnostic check "-Wanalyzer-maybe-uninitialized"
}
#pragma cake diagnostic check "-Wmissing-destructor"

