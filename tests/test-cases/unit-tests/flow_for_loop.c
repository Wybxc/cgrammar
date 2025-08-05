#pragma safety enable


void* malloc(unsigned long size);
void free(void* ptr);

int main() {
   void * p = 0;
   for (int i=0; i < 2; i++) {
     p = malloc(1); /*leak*/
   }
   #pragma cake diagnostic check "-Wmissing-destructor"
   free(p);
}