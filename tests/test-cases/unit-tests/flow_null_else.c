#pragma safety enable


void* malloc(unsigned long size);
void free(void* ptr);

void f5()
{
    void * p = malloc(1);

    if (p) {
       free(p);
       return;
    }

    static_state(p, "null ");
}
