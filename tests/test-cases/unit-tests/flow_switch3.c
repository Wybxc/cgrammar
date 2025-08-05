#pragma safety enable



void * calloc(int n , int i);

char* f(int i)
{
    char* p = calloc(1,2);
    switch (i) {
        case 1: break;
    }
    return p;
}