#pragma safety enable


int* f();

int main()
{
    int* p1 = f();
    int* p2 = f();

    if (p1 && p2)
    {
        static_state(p1, "not-null");
        static_state(p2, "not-null");
    }
    else
    {
    }
}