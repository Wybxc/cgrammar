#pragma safety enable
void f(int  * p)
{
  static_state(p, "null | not-null");
}
