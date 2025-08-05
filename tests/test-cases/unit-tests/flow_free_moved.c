#pragma safety enable

void* calloc(unsigned long n, unsigned long size);
void free(void* ptr);


int main() {
   int * p = calloc(1, sizeof(int));
   int *p2 = p;    //MOVED
   free(p2);
   free(p); //MOVED
#pragma cake diagnostic check "-Wusing-moved-object"
}