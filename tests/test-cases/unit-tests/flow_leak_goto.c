#pragma safety enable


void free(void* ptr);
void* malloc(int size);

void f(int condition)
{
    int* p = malloc(sizeof(int));

    if (condition)
    {
        goto end;
    }

    free(p);
end:
}
#pragma cake diagnostic check "-Wmissing-destructor"

