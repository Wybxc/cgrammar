#pragma safety enable;

struct X
 {
   char * p;
 };

 char* strdup(const char *s);

 int main() {

     struct X x = {
         .p = strdup("a")
     };
 }
#pragma cake diagnostic check "-Wmissing-destructor"
