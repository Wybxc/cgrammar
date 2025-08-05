
#pragma safety enable

void* calloc(int n, unsigned long size);
void free(void* ptr);

struct Y {
    int i;
};
struct X {
    struct Y* pY;
};

void f(struct Y* p);
int main()
{

    struct X* p = calloc(1, sizeof * p);
    if (p)
    {
        p->pY = calloc(1, sizeof(struct Y));
        if (p->pY)
        {
            f(p->pY);
            p->pY->i = 1;
//          ^^^^^ still not null
        }
        free(p->pY);
        free(p);
    }
}

