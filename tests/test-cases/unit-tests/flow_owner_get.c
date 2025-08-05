#pragma safety enable


int*  get();

void f()
{
    int* p = 0;
    p = get();
}


#pragma cake diagnostic check "-Wmissing-destructor"
