#pragma safety enable


int rand();
void free(void* ptr);

struct X { char* name; };

void x_destroy(struct X*  p);
struct X f();

void f()
{
    {
        struct X x = {0};

        if (rand())
        {
            x = f();
        }
        else
        {
            x = f();
        }
        x_destroy(&x);
    }
}
