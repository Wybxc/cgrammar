
#pragma safety enable


void * malloc(int i);
void free( void *  p);

struct X {
  char * name;
};

int main() {
   struct X * p = malloc(sizeof * p);
   free(p);
}
