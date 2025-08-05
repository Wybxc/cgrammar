#pragma safety enable



int* make1();
int* make2();
void free(void * p);


void f(int condition)
{
  int * p = 0;
  static_state(p, "null");

  if (condition)
  {
       static_state(p, "null");
       p = make1();
       static_state(p, "not-null ");
       free(p);
       p = make2();
       static_state(p, "null | not-null ");
  }
  else
  {
    static_state(p, "null");
  }
  free(p);
}