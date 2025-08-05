#pragma safety enable


void* f();
void free(void* p);
int main() {
    void* p = f();
    if (p)
    {
        free(p);
        p = f();
    }
}
#pragma cake diagnostic check "-Wmissing-destructor"
