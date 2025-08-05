#pragma safety enable


void free(void*) { }
#pragma cake diagnostic check "-Wmissing-destructor"
