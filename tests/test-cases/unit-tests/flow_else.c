#pragma safety enable


void* malloc(int sz);

void f(int i) {
    if (i) {
    }
    else {
        int* p3 = malloc(1);
    }
}
#pragma cake diagnostic check "-Wmissing-destructor"

