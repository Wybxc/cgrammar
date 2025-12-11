// Integer Literal Edge Cases
// Decimal literals
int dec1 = 0;
int dec2 = 123;
long dec3 = 123L;
long long dec4 = 123LL;
unsigned dec5 = 123U;
unsigned long dec6 = 123UL;
unsigned long long dec7 = 123ULL;

// Octal literals
int oct1 = 0;
int oct2 = 0123;
int oct3 = 0777;

// Hexadecimal literals
int hex1 = 0x0;
int hex2 = 0xFF;
int hex3 = 0xDEADBEEF;
int hex4 = 0XABCDEF;
long hex5 = 0xFFFFFFFFL;

// Binary literals (C23)
int bin1 = 0b0;
int bin2 = 0b1010;
int bin3 = 0b11111111;
int bin4 = 0B10101010;

// Digit separators (C23)
int sep1 = 1'000'000;
int sep2 = 0xFF'FF'FF'FF;
int sep3 = 0b1010'1010;

// Various suffixes
long l1 = 123l;
long l2 = 123L;
long long ll1 = 123ll;
long long ll2 = 123LL;
unsigned u1 = 123u;
unsigned u2 = 123U;
unsigned long ul1 = 123ul;
unsigned long ul2 = 123UL;

// Large values
long long large = 9223372036854775807LL;
unsigned long long ularge = 18446744073709551615ULL;
