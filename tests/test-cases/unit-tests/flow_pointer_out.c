#pragma safety enable


void* malloc(unsigned long size);
void free(void* ptr);

struct X {
    char* text;
};

void f(struct X* p1, struct X** p2)
{
    *p2 = p1;
}

int main()
{
    struct X* p1 = malloc(sizeof * p1);
    if (p1)
    {
        p1->text = 0;
        struct X* p2 = 0;
        f(p1, &p2);

        free(p2->text);
#pragma cake diagnostic check "-Wanalyzer-null-dereference"

        free(p2);
    }
}
