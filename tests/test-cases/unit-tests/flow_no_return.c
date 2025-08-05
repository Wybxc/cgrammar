#pragma safety enable

void free(void* p);
char* strdup(const char* s);
[[noreturn]] void exit( int exit_code );

void f()
{
    char * s = strdup("a");

    if (s == nullptr)
    {
        exit(1);
    }

    static_state(s, "not-null");
    free(s);
}
