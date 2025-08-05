#pragma safety enable


char * strdup(const char* s);
void free(void * p);

struct X {
  char *name;
};

struct X make()
{
  struct X x = {0};
  x.name = strdup("text");
  return x;
}
