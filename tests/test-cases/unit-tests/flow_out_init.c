#pragma safety enable


void  free(void* p);
char* strdup(const char* s);

struct X {
    char* s;
};
void init(struct X* px)
{
    static_state(px, "not-null");
    static_state(px->s, "uninitialized");
    px->s = strdup("a");
}

int main() {
    struct X x;
    init(&x);
    free(x.s);
}
