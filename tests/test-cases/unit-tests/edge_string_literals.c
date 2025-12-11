// String Literal Edge Cases
// Basic string literals
const char *str1 = "hello";
const char *str2 = "world";

// String concatenation
const char *concat = "hello" " " "world";
const char *multiline = "This is a "
                        "multi-line "
                        "string";

// Escape sequences
const char *escapes = "tab\there\nnewline\nquote\"slash\\";
const char *hex = "\x41\x42\x43";
const char *octal = "\101\102\103";

// Wide strings
const wchar_t *wide = L"wide string";
const wchar_t *wide_concat = L"hello" L" " L"world";

// UTF-8, UTF-16, UTF-32 strings (C11)
const char *utf8 = u8"UTF-8 string";
const char16_t *utf16 = u"UTF-16 string";
const char32_t *utf32 = U"UTF-32 string";

// Empty string
const char *empty = "";

// String with only escape sequences
const char *only_escapes = "\n\t\r";

// Raw string-like (using preprocessor)
const char *path = "C:\\Users\\path\\to\\file";
