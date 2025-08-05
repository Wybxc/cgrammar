#pragma safety enable

void free(void *p);

void f(int* p)
{
   int * p2 = p;
   static_state(p, "null, moved");
   if (p)
   {
     static_state(p, "moved");
   }
   free(p2);
}
