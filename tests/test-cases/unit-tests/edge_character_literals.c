// Character Literal Edge Cases
// Basic character literals
char c1 = 'a';
char c2 = 'Z';
char c3 = '0';

// Escape sequences
char newline = '\n';
char tab = '\t';
char backslash = '\\';
char quote = '\'';
char return_char = '\r';
char null_char = '\0';

// Octal escape sequences
char octal1 = '\0';
char octal2 = '\101';  // 'A'
char octal3 = '\177';

// Hex escape sequences
char hex1 = '\x00';
char hex2 = '\x41';  // 'A'
char hex3 = '\xFF';

// Wide character literals
wchar_t wc1 = L'a';
wchar_t wc2 = L'中';
wchar_t wc3 = L'\n';

// UTF-8 character literal (C11)
char utf8_char = u8'a';

// UTF-16 character literal (C11)
char16_t utf16_char = u'a';
char16_t utf16_unicode = u'€';

// UTF-32 character literal (C11)
char32_t utf32_char = U'a';
char32_t utf32_unicode = U'😀';

// Special characters
char bell = '\a';
char backspace = '\b';
char form_feed = '\f';
char vertical_tab = '\v';
char question = '\?';

// Universal character names
char32_t ucn1 = U'\u0041';  // 'A'
char32_t ucn2 = U'\U0001F600';  // emoji
