#pragma safety enable



void free(void* ptr);
void* malloc(int size);

void f(int c)
{
    int* p = malloc(sizeof(int));
    if (c) {
        free(p);
    }
}

#pragma cake diagnostic check "-Wmissing-destructor"
