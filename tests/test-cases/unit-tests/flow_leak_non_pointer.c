#pragma safety enable



struct X {
    int i;
};
void f() {
    const struct X x = { 0 };
}

#pragma cake diagnostic check "-Wmissing-destructor"

