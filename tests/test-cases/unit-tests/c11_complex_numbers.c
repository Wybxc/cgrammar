// C99/C11 Complex Numbers
#include <complex.h>

_Complex float cf;
_Complex double cd;
_Complex long double cld;

float _Complex cf2;
double _Complex cd2;

// Using complex.h macros
float complex fc;
double complex dc;

void test_complex() {
    _Complex float z1 = 1.0f + 2.0f * I;
    _Complex double z2 = 3.0 + 4.0 * I;
    
    // Complex arithmetic
    _Complex float z3 = z1 + z1;
    _Complex float z4 = z1 * z1;
}

// Complex arrays
_Complex double complex_array[10];

// Complex in struct
struct ComplexPair {
    _Complex float a;
    _Complex double b;
};

// Function returning complex
_Complex double get_complex(void);

// Function taking complex parameters
void process_complex(_Complex float z);
