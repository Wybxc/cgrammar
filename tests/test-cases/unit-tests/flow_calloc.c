#pragma safety enable


void* calloc(unsigned long n , unsigned long size);
void free(void* ptr);

struct Y {
  char * p0;
  int * p2;
  double i2;
};

struct X {
  char * text;
  int * p1;
  int i;
  struct Y  * pY;
};

int main() {
   struct X * x = calloc(1,sizeof * x);
   static_state(x, "null | not-null ");

   static_state(x->p1, "null ");
   static_state(x->i, "zero");
   static_state(x->pY, "null");
   static_state(x->pY->p0, "");
   static_state(x->pY->p2, "");
   static_state(x->pY->i2, "");
   free(x);
}

