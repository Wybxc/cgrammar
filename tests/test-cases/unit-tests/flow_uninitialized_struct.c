#pragma safety enable


struct X {
    char* text;
};

void x_destroy(struct X*  p);


int main() {
    struct X x;
    x_destroy(&x);
}

//flow analyze
#pragma cake diagnostic check "-Wanalyzer-maybe-uninitialized"
