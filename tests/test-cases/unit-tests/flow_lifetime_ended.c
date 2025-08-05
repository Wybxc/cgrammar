#pragma safety enable

struct X {
    int i;
};

void f2()
{
    struct X* p = 0;
    {
        struct X x = { 0 };
        p = &x;
    }
    if (p->i) {}

#pragma cake diagnostic check "-Wlifetime-ended"

}