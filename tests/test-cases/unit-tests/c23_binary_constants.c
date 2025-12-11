// C23 Binary Constants - Extended
int bin1 = 0b0;
int bin2 = 0b1;
int bin3 = 0b10;
int bin4 = 0b11;
int bin5 = 0b100;
int bin6 = 0b1111;
int bin7 = 0b10101010;
int bin8 = 0b11111111;

// Binary with suffixes
long bin_long = 0b101010L;
unsigned bin_unsigned = 0b1010U;
long long bin_ll = 0b11001100LL;
unsigned long long bin_ull = 0b10101010ULL;

// Binary with digit separators
int bin_sep1 = 0b1010'1010;
int bin_sep2 = 0b1111'0000'1111'0000;
int bin_sep3 = 0b1'0'1'0'1'0'1'0;

// Large binary values
long bin_large = 0b11111111'11111111'11111111'11111111L;
unsigned long long bin_huge = 0b1111111111111111'1111111111111111'1111111111111111'1111111111111111ULL;

// Binary in expressions
int bin_expr1 = 0b1010 + 0b0101;
int bin_expr2 = 0b1111 & 0b1010;
int bin_expr3 = 0b0011 | 0b1100;
int bin_expr4 = 0b1010 ^ 0b0101;

// Binary in array sizes
int bin_array[0b100];

// Binary in case labels
void test_binary_switch(int x) {
    switch (x) {
        case 0b0001:
            break;
        case 0b0010:
            break;
        case 0b0100:
            break;
        case 0b1000:
            break;
    }
}
