#pragma safety enable


void* malloc(unsigned size);
void free(void* ptr);


struct X {  char *name; };
struct Y { struct X x; };

void f(struct Y * y, struct X *  p)
{
    free(y->x.name);
    y->x = *p;
}
