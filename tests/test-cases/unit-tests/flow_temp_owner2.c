#pragma safety enable

void* malloc(unsigned long size);
void free(void* ptr);

int main()
{
    void* p = malloc(1);

//left object must be an owner reference.
#pragma cake diagnostic check "-Wtemp-owner"

    free(p);
#pragma cake diagnostic check "-Wnon-owner-to-owner-move"
}



