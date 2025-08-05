#pragma safety enable



void free(void* p);

struct X {
    char* name;
};

void x_delete(struct X* p)
{
    if (p) {
        //free(p->name);
        free(p);
    }
}

#pragma cake diagnostic check "-Wmissing-destructor"

