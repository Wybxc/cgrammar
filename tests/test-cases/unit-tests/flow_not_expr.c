#pragma safety enable

char* f();

void f()
{
    const char* s1 = f();
    if (!s1)
    {
        static_state(s1, "null");
    }
    else
    {
        static_state(s1, "not-null");
    }
}