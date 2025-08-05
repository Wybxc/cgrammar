#pragma safety enable


void* malloc(unsigned size);
void free(void* ptr);

struct X {
    char* name;
};

struct X* F(int i)
{

    struct X* p1 = 0;
    try
    {
        if (i == 1)
        {
            p1 = malloc(sizeof * p1);
            while (0){}
        }
        else if (i == 3)
        {
            p1 = malloc(sizeof * p1);
        }
    }
    catch
    {
    }

    return p1;  //p1.name not initialized
}

#pragma cake diagnostic check "-Wanalyzer-maybe-uninitialized]"
#pragma cake diagnostic check "-Wanalyzer-maybe-uninitialized]"

//We have two error message here, because one is generated when we read p1
//the other one is generated because the returned object may access initialized objects.


