#pragma safety enable


void F(int i);
int make();
int main()
{
    F(make());
}
#pragma cake diagnostic check "-Wtemp-owner"

