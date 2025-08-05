#pragma safety enable;

struct X
 {
   char * p;
   char * p2;
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
 #pragma cake diagnostic check "-Wanalyzer-null-dereference"
