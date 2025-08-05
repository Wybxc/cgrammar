#pragma safety enable


void* malloc(unsigned size);
void free(void* ptr);


void f(int i)
{
  void * p1 = malloc(1);
  switch(i)
  {
      case 1:
      {
          void * p2 = malloc(1);
          free(p2);
      }
      break;

      case 2:
      {
          void * p3 = malloc(1);
            free(p3);
      }
      break;
  }

  free(p1);

}
