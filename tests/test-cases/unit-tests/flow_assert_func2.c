#pragma safety enable

void free(void* p);
char* strdup(const char* s);


struct X {

    char* text;
};
#define NULL ((void*)0)

struct X makex();
void clear(struct X* p);
void f(struct X* pX)
{
    struct X x = makex();
    clear(&x);
    assert(x.text == 0);
}
