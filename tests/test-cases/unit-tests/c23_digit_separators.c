// C23 Digit Separators - Extended
// Decimal with separators
int dec1 = 1'000;
int dec2 = 1'000'000;
int dec3 = 1'234'567'890;
long dec4 = 9'223'372'036'854'775'807L;

// Hex with separators
int hex1 = 0xFF'FF;
int hex2 = 0xDEAD'BEEF;
int hex3 = 0x0123'4567'89AB'CDEF;

// Binary with separators
int bin1 = 0b1111'0000;
int bin2 = 0b1010'1010'1010'1010;
int bin3 = 0b1111'1111'1111'1111'1111'1111'1111'1111;

// Octal with separators
int oct1 = 0123'456'777;

// Float with separators
float f1 = 1'000.5f;
float f2 = 3.141'592'653f;
double d1 = 1'234'567.890'123;
double d2 = 1e1'00;

// Arbitrary separator positions
int weird1 = 1'2'3'4'5'6;
int weird2 = 0x1'2'3'4;
int weird3 = 0b1'0'1'0;

// Separators with suffixes
long sep_long = 1'000'000L;
unsigned long sep_ul = 1'234'567UL;
long long sep_ll = 1'000'000'000'000LL;

// Separators in scientific notation
double sci1 = 1.234'567e10;
double sci2 = 1e1'00;
