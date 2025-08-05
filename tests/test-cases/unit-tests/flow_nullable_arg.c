#pragma safety enable


void f(int * p);
int main()
{
   int * p = 0;
   p = ((void *) 0);
   f(0);
   f((void *) 0);
   f(nullptr);
}
