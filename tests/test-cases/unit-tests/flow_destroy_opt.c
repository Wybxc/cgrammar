#pragma safety enable


void free(void * p);
struct X {
    char * text;
};

void x_destroy(struct X *  x) {
    free(x->text);
}

int main() {
    struct X x = {};
    x_destroy(&x);
}
