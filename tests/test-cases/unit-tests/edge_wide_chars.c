// Wide character and string literals
#include <wchar.h>
#include <uchar.h>

wchar_t wc = L'A';
wchar_t wc2 = L'中';
wchar_t wc3 = L'\n';

const wchar_t *ws = L"Hello";
const wchar_t *ws2 = L"World";
const wchar_t *ws_concat = L"Hello" L" " L"World";

// UTF-16 (C11)
char16_t c16 = u'A';
char16_t c16_2 = u'€';
const char16_t *s16 = u"UTF-16 string";

// UTF-32 (C11)
char32_t c32 = U'A';
char32_t c32_2 = U'😀';
const char32_t *s32 = U"UTF-32 string";

// UTF-8 (C11)
const char *utf8 = u8"UTF-8 string";
const char *utf8_emoji = u8"Hello 😀 World";

// Mixed in array
wchar_t wide_array[] = L"test";
char16_t utf16_array[] = u"test";
char32_t utf32_array[] = U"test";

// Wide character escape sequences
wchar_t wc_escape1 = L'\n';
wchar_t wc_escape2 = L'\x41';
wchar_t wc_escape3 = L'\u0041';
wchar_t wc_escape4 = L'\U00000041';
