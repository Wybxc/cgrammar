// Dollar sign in identifiers (compiler extension)
// Note: Not standard C, but commonly supported

int regular_identifier = 42;

// Some compilers allow these (as extension):
// int identifier_with_$_dollar = 10;
// int $start_with_dollar = 20;
// int another$dollar$sign = 30;

// For standard compliance, using regular identifiers
int identifier_with_underscore = 10;
int start_with_letter = 20;
int another_underscore_sign = 30;

struct RegularStruct {
    int member1;
    int member2;
};

void regular_function(void) {
    int local_var = 100;
}
