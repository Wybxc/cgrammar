#pragma safety enable


struct X {
  void * text;
};

void x_change(struct X* list);
void x_destroy(struct X*  p);

int main()
{
  struct X x = {};
  x_change(&x);
  static_debug(x);
}
//memory pointed by 'x.text' was not released.
#pragma cake diagnostic check "-Wmissing-destructor"

