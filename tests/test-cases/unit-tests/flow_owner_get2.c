
#pragma safety enable


int*  get();
void dtor(int* p);

void f(int a)
{
    int* p = 0;
    p = get();
    dtor(p);
}
