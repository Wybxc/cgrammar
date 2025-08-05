#pragma safety enable


void free(void * p);
struct X;
struct X f();
struct X { char * p; };
int main()
{
    struct X x = 1 ? f() : f();
    free(x.p);
}