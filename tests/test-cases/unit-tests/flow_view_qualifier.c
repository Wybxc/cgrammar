#pragma safety enable


struct X {
  char * name;
};

struct X f();

int main()
{
  f();
}
