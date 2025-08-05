#pragma safety enable

struct X {
    char * text;
};

struct Y {
    struct X x;
};

struct Y f();
int main()
{
    struct Y y = f();
}



