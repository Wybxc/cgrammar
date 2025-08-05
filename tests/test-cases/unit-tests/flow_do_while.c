#pragma safety enable



void* malloc(unsigned size);
void free(void*  ptr);

int main() {
   void * p = malloc(1);
   do{
      free(p);
   }
   while(0);
}
