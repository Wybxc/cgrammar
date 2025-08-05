#pragma safety enable


void destroy(int i);

int main()
{
    int i = 0;
    int v = i;
    destroy(v);
    #pragma cake diagnostic check "-Wnon-owner-to-owner-move"
}
#pragma cake diagnostic check "-Wmissing-destructor"


