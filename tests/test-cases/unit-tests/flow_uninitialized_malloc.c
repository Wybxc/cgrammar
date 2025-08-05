#pragma safety enable


void* malloc(int i);
void free(void* p);

struct X {
    char* name;
};

void x_delete(struct X* p)
{
    if (p) {
        free(p->name);
        free(p);
    }
}

int main() {
    struct X* p = malloc(sizeof * p);

    x_delete(p);

    //p.name is uninitialized
    #pragma cake diagnostic check "-Wanalyzer-maybe-uninitialized"
}


