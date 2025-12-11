// Floating-Point Literal Edge Cases
// Basic float literals
float f1 = 0.0f;
float f2 = 1.0F;
float f3 = 3.14f;

// Double literals
double d1 = 0.0;
double d2 = 1.0;
double d3 = 3.14159;

// Long double literals
long double ld1 = 0.0L;
long double ld2 = 3.14159L;

// Scientific notation
double sci1 = 1e10;
double sci2 = 1.5e-10;
float sci3 = 3.14e2f;
double sci4 = 1E10;

// Hexadecimal floating-point (C99)
double hex1 = 0x1.0p0;
double hex2 = 0x1.921fb54442d18p+1;  // pi
float hex3 = 0x1.0p-126f;

// Without integer part
float frac1 = .5f;
double frac2 = .123;

// Without fractional part
double int_part1 = 1.;
float int_part2 = 42.f;

// Digit separators in floats (C23)
double sep1 = 1'000.5;
double sep2 = 3.141'592'653;
float sep3 = 1'234.567'8f;

// Very small and large values
double small = 1e-308;
double large = 1e308;
float small_f = 1e-38f;
float large_f = 1e38f;
