#pragma safety enable


struct X {
    char* p;
};
void x_destroy(struct X*  p);
void f(struct X* x)
{
    x_destroy(x);
}

#pragma cake diagnostic check "-Wmust-use-address-of"
