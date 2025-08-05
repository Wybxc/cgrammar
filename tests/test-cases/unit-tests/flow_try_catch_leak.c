#pragma safety enable


void* malloc(int i);
void free(void* p);
int rand();

int main()
{
    char* s = malloc(1);
    try
    {
        if (rand())
        {
            free(s);
        }
        else
        {
            static_debug(s);
            throw;
        }
    }
    catch
    {
    }
}
#pragma cake diagnostic check "-Wmissing-destructor"
