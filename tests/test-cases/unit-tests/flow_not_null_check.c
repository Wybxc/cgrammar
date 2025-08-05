#pragma safety enable

char* f();

void f()
{
    const char* s1 = f();
    if (s1 != 0)
    {
        static_state(s1, "not-null");
    }
    else
    {
        static_state(s1, "null");
    }
}