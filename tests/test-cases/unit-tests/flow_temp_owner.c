#pragma safety enable


void* malloc(unsigned long size);
void free(void* ptr);

struct X {
    char* name;
};

int main()
{
    struct X* p = (struct X*) malloc(1);
}

//flow analyze
#pragma cake diagnostic check "-Wtemp-owner"


