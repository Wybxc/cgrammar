// Long identifier names (testing parser limits)
int very_long_identifier_name_that_goes_on_and_on_and_on_and_on_and_on;
int another_very_long_identifier_name_with_many_underscores_in_between_words;

struct VeryLongStructNameThatTestsParserLimitsForIdentifiers {
    int very_long_member_name_inside_struct;
    int another_very_long_member_name;
};

void very_long_function_name_that_tests_parser_limits(
    int very_long_parameter_name_one,
    int very_long_parameter_name_two,
    int very_long_parameter_name_three
);

// Long string literals
const char *long_string = "This is a very long string literal that contains many characters and words to test how the parser handles long string literals that might span multiple lines when concatenated like this one does";

// Long macro names in usage
#define VERY_LONG_MACRO_NAME_FOR_TESTING 42
int x = VERY_LONG_MACRO_NAME_FOR_TESTING;
