#pragma safety enable;

struct X
 {
   char * p;
 };

 char* strdup(const char *s);
 void free(void* p);

 int main() {
     const char* p0 = strdup("a");
     struct X x = {
         .p = p0
     };
     free(x.p);
 }
