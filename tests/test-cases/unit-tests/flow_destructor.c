#pragma safety enable


void free( void * p);

struct X {
  char * name;
};

void x_destroy( struct X *  p);

struct Y {
  struct X x;
};

void y_destroy(struct Y *  p) {
   x_destroy(&p->x);
}
