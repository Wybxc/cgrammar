// C99 Imaginary Numbers (optional feature)
#include <complex.h>

_Imaginary float if1;
_Imaginary double id1;
float _Imaginary if2;
double _Imaginary id2;

void test_imaginary() {
    _Imaginary float i1 = 2.0f * I;
    _Imaginary double i2 = 3.0 * I;
}

// Imaginary in arrays
_Imaginary float imag_array[5];

// Imaginary in struct
struct ImaginaryData {
    _Imaginary float imag_f;
    _Imaginary double imag_d;
};
