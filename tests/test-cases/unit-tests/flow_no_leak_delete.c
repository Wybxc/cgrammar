#pragma safety enable


void free( void * p);
struct X {
  char * text;
};
void x_delete( struct X * p)
{
    if (p)
    {
      free(p->text);
      free(p);
    }
}
