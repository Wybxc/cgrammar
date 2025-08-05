#pragma safety enable


void * malloc(int i);
void free( void * p);

struct X {
  char * name;
};

int main() {
   struct X * p = malloc(sizeof * p);
   if (p) {
     p->name = malloc(10);
     free(p->name);
   }
   free(p);
}
