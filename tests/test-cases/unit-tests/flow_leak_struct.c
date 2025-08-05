#pragma safety enable



char* strdup(const char* s);
void free(void* p);

struct X
{
    char* text;
};

void f(int a)
{
    struct X x = { 0 };
    x.text = strdup("a");
}

#pragma cake diagnostic check "-Wmissing-destructor"

