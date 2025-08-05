#pragma safety enable


void free( void* ptr);
void* malloc(int size);

int main()
{
    int* p = malloc(sizeof(int));
    if (p != 0)
    {
       free(p);
    }
}

