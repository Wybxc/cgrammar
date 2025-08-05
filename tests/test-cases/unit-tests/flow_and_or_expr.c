#pragma safety enable


struct X {
    int i;
};

void f(struct X * p)
{
      if (p && p->i || p->i >2)
      {
#pragma cake diagnostic check "-Wanalyzer-null-dereference"
      }
      else
      {
      }
}

