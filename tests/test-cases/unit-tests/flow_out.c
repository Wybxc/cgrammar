#pragma safety enable


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

void init(struct X * p);
void destroy(struct X *  p);

int main() {
   struct X x;
   init(&x);

   static_state(x.p1, "not-null ");
   static_state(x.i, "zero | not-zero");
   static_state(x.pY, "not-null");
   static_state(x.pY->p0, "not-null ");
   static_state(x.pY->p2, "not-null ");
   static_state(x.pY->i2, "zero | not-zero");
   destroy(&x);
}

