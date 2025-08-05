
#pragma safety enable


struct Y {
    int i;
};

struct X {
    char * name;
};

int main()
{
    struct Y y1 = {};
    struct Y y2 = {};
    y1 = y2; //ok

    struct X x1 = {};
    struct X x2 = {};
    x1 = x2; //ok

}