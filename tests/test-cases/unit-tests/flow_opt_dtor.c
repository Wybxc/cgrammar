#pragma safety enable

void free(void* p);
char* strdup(const char* s);

struct Y {
    char* text;
};

struct X {

    struct Y* pY;
};

void f(struct X* pX)
{
    if (pX)
    {
        free(pX->pY->text);
        pX->pY->text = strdup("a");
    }
}
