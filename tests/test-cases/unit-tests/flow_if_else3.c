#pragma safety enable

void free(void *p);
char * strdup(const char* s);

struct X {
  char *text;
};

struct X * make();

void f(int condition)
{
    struct X * p = nullptr;
    if (condition)
    {
        p = make();
    }
    else
    {
        p = make();
    }

    free(p->text);
    p->text = strdup("c");

    free(p->text);
    free(p);
}
