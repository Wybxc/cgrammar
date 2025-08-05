#pragma safety enable


char* f() {
    char* p = 0;
    return p;

//cannot return a owner to non owner
#pragma cake diagnostic check "-E1280"

}
