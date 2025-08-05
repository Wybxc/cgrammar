#pragma safety enable


void* malloc(unsigned long size);
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
  struct Y  *pY;
};

int main() {
   struct X * x = malloc(sizeof * x);
   static_state(x, "null | not-null ");

   static_state(x->p1, "uninitialized");
   static_state(x->i, "uninitialized");
   static_state(x->pY, "uninitialized");
   free(x);
}

