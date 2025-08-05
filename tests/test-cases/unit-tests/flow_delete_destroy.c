
#pragma safety enable



void free( void* ptr);
void* malloc(int size);
struct X { char * text; };

void x_destroy(struct X*  p)
{
    free(p->text);
}

void x_delete(struct X* p)
{
    if (p)
    {
        x_destroy(p);
        free(p);
    }
}