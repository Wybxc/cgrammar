#pragma safety enable


char * f() {
    char * p = 0;
    return p; /*implicit move*/
}