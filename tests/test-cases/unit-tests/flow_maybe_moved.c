#pragma safety enable

void free(void* p);
char* strdup(const char* s);

struct X {
    char* text;
};

struct X* make();

void f(int condition)
{
    struct X* p1 = make();


    {
        struct X* p2 = make();

        struct X* p = nullptr;
        if (condition)
        {
            p = p1;
        }
        else
        {
            p = p2;
        }

        free(p->text);
        p->text = strdup("c");

        free(p->text);
        free(p);
    }
#pragma cake diagnostic check "-Wmissing-destructor"

}
#pragma cake diagnostic check "-Wmissing-destructor"
