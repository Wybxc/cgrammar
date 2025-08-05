#pragma safety enable


void * calloc(int i, int n);
void free(void * p);

int main() {
    int * p1 = 0;
    int * p2 = calloc(1, sizeof(int));

    if (p2 == 0) {
        return 1;
    }
    else
    {
      p1 = p2;
    }
    static_state(p2, "moved");
    free(p1);
    return 0;
}