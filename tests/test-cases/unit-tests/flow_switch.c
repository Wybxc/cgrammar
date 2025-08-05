#pragma safety enable



void* make();
void free( void* p);

void f(int condition)
{
    void* p = make();


    switch (condition)
    {
        case 1:
        {
            free(p);
        }
        break;
        case 2:
        {
            free(p);
        }
        break;

        default:
            free(p);
            break;
    }
}