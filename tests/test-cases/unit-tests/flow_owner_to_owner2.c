#pragma safety enable


void destroy(char* x);
char   *  get();

int main()
{
  destroy(get());
}
