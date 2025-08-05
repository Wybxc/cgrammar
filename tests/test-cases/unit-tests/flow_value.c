#pragma safety enable



bool f()
{
    int  * p =0;
    return p == 0;
    #pragma cake diagnostic check "-Wflow-not-null"
}