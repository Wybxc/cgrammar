#pragma safety enable



void* calloc(int n ,unsigned size);

char* f(int i)
{
    char* p = calloc(1,2);
    switch (i)
    {
        case 1:
            break;
        case 2:
            break;
    }

    return p;
}
