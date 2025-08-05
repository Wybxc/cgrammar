#pragma safety enable


char * create();
void f(char * p);

int main()
{
    f(create());
}

