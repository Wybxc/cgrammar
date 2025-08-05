#pragma safety enable


void * calloc(int i, int sz);
void free( void * p);

struct X { int i; };
struct Y { struct X * p; };

int main() {
   struct Y y = {0};
   struct X * p = calloc(1, sizeof(struct X));
   if (p){
     y.p = p;
   }
  free(y.p);
}

