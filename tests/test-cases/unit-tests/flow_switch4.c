#pragma safety enable


void* calloc(unsigned n, unsigned size);
void free(void* ptr);

struct X {
    char* name;
};

struct X* F(int i)
{
    struct X* p1 = 0;

    switch (i)
    {
        case 1:
            struct X* p2 = calloc(1, sizeof * p2);
            if (p2)
            {
              static_set(*p2, "zero");
                p1 = p2;
            }
            break;
        case 2:
            break;
    }

    return p1;
}