use cgrammar::*;
use chumsky::prelude::*;
use rstest::rstest;

// =============================================================================
// Test identifier parser
// =============================================================================

#[rstest]
#[case("hello", "hello")]
#[case("_var", "_var")]
#[case("var123", "var123")]
#[case("_123abc", "_123abc")]
#[case("camelCase", "camelCase")]
#[case("snake_case", "snake_case")]
#[case("UPPER_CASE", "UPPER_CASE")]
fn test_identifier_valid(#[case] input: &str, #[case] expected: &str) {
    let parser = identifier();
    let result = parser.parse(input).unwrap();
    assert_eq!(result.0, expected);
}

#[rstest]
#[case("123abc")] // Cannot start with a digit
#[case("")] // Empty string
fn test_identifier_invalid(#[case] input: &str) {
    let parser = identifier();
    let result = parser.parse(input);
    assert!(result.has_errors());
}

// =============================================================================
// Test encoding prefix parser
// =============================================================================

#[rstest]
#[case("u8", EncodingPrefix::U8)]
#[case("u", EncodingPrefix::U)]
#[case("U", EncodingPrefix::CapitalU)]
#[case("L", EncodingPrefix::L)]
fn test_encoding_prefix_valid(#[case] input: &str, #[case] expected: EncodingPrefix) {
    let parser = encoding_prefix();
    let result = parser.parse(input).unwrap();
    assert_eq!(result, expected);
}

#[rstest]
#[case("x")]
#[case("l")]
#[case("u16")]
#[case("")]
fn test_encoding_prefix_invalid(#[case] input: &str) {
    let parser = encoding_prefix();
    let result = parser.parse(input);
    assert!(result.has_errors());
}

// =============================================================================
// Test escape sequence parser
// =============================================================================

#[rstest]
#[case(r"\'", '\'')]
#[case(r#"\""#, '"')]
#[case(r"\?", '?')]
#[case(r"\\", '\\')]
#[case(r"\a", '\x07')]
#[case(r"\b", '\x08')]
#[case(r"\f", '\x0C')]
#[case(r"\n", '\n')]
#[case(r"\r", '\r')]
#[case(r"\t", '\t')]
#[case(r"\v", '\x0B')]
fn test_escape_sequence_valid(#[case] input: &str, #[case] expected: char) {
    let parser = escape_sequence();
    let result = parser.parse(input).unwrap();
    assert_eq!(result, expected);
}

#[rstest]
#[case(r"\u123")] // Insufficient Unicode sequence digits
#[case(r"\U1234567")] // Insufficient Unicode sequence digits
fn test_escape_sequence_invalid(#[case] input: &str) {
    let parser = escape_sequence();
    let result = parser.parse(input);
    assert!(result.has_errors());
}

// =============================================================================
// Test string literal parser
// =============================================================================

#[rstest]
#[case(r#""hello""#, None, "hello")]
#[case(r#"u8"hello""#, Some(EncodingPrefix::U8), "hello")]
#[case(r#"u"hello""#, Some(EncodingPrefix::U), "hello")]
#[case(r#"U"hello""#, Some(EncodingPrefix::CapitalU), "hello")]
#[case(r#"L"hello""#, Some(EncodingPrefix::L), "hello")]
#[case(r#""hello\nworld""#, None, "hello\nworld")]
#[case(r#""hello\"world""#, None, "hello\"world")]
#[case(r#""""#, None, "")]
fn test_string_literal_valid(
    #[case] input: &str,
    #[case] expected_prefix: Option<EncodingPrefix>,
    #[case] expected_value: &str,
) {
    let parser = string_literal();
    let result = parser.parse(input).unwrap();
    assert_eq!(result.encoding_prefix, expected_prefix);
    assert_eq!(result.value, expected_value);
}

#[rstest]
#[case(r#""hello"#)] // Missing closing quote
#[case(r#"hello""#)] // Missing opening quote
#[case(r#""hello"world""#)] // Content beyond string range
fn test_string_literal_invalid(#[case] input: &str) {
    let parser = string_literal();
    let result = parser.parse(input);
    assert!(result.has_errors());
}

// =============================================================================
// Test integer suffix parser
// =============================================================================

#[rstest]
#[case("u", IntegerSuffix::Unsigned)]
#[case("U", IntegerSuffix::Unsigned)]
#[case("l", IntegerSuffix::Long)]
#[case("L", IntegerSuffix::Long)]
#[case("ll", IntegerSuffix::LongLong)]
#[case("LL", IntegerSuffix::LongLong)]
#[case("ul", IntegerSuffix::UnsignedLong)]
#[case("UL", IntegerSuffix::UnsignedLong)]
#[case("lu", IntegerSuffix::UnsignedLong)]
#[case("LU", IntegerSuffix::UnsignedLong)]
#[case("ull", IntegerSuffix::UnsignedLongLong)]
#[case("ULL", IntegerSuffix::UnsignedLongLong)]
#[case("llu", IntegerSuffix::UnsignedLongLong)]
#[case("LLU", IntegerSuffix::UnsignedLongLong)]
fn test_integer_suffix_valid(#[case] input: &str, #[case] expected: IntegerSuffix) {
    let parser = integer_suffix();
    let result = parser.parse(input).unwrap();
    assert_eq!(result, expected);
}

#[rstest]
#[case("x")]
#[case("lll")]
#[case("uuu")]
#[case("")]
fn test_integer_suffix_invalid(#[case] input: &str) {
    let parser = integer_suffix();
    let result = parser.parse(input);
    assert!(result.has_errors());
}

// =============================================================================
// Test decimal constant parser
// =============================================================================

#[rstest]
#[case("123", 123)]
#[case("456789", 456789)]
#[case("1'234", 1234)] // Digit separator
#[case("9'876'543", 9876543)] // Multiple digit separators
fn test_decimal_constant_valid(#[case] input: &str, #[case] expected: i128) {
    let parser = decimal_constant();
    let result = parser.parse(input).unwrap();
    assert_eq!(result, expected);
}

#[rstest]
#[case("01")] // Octal (starts with 0 but not 0)
#[case("")] // Empty string
#[case("abc")] // Non-digits
#[case("0")] // This should be handled by octal_constant
fn test_decimal_constant_invalid(#[case] input: &str) {
    let parser = decimal_constant();
    let result = parser.parse(input);
    assert!(result.has_errors());
}

// =============================================================================
// Test octal constant parser
// =============================================================================

#[rstest]
#[case("0", 0)]
#[case("01", 1)]
#[case("0123", 83)]
#[case("0'123", 83)] // Digit separator
#[case("076'543", 32099)] // Multiple digit separators
fn test_octal_constant_valid(#[case] input: &str, #[case] expected: i128) {
    let parser = octal_constant();
    let result = parser.parse(input).unwrap();
    assert_eq!(result, expected);
}

#[rstest]
#[case("18")] // Contains invalid octal digit
#[case("09")] // Contains invalid octal digit
#[case("")] // Empty string
#[case("abc")] // Non-digits
fn test_octal_constant_invalid(#[case] input: &str) {
    let parser = octal_constant();
    let result = parser.parse(input);
    assert!(result.has_errors());
}

// =============================================================================
// Test hexadecimal constant parser
// =============================================================================

#[rstest]
#[case("0x123", 0x123)]
#[case("0X123", 0x123)]
#[case("0xabc", 0xabc)]
#[case("0XDEF", 0xDEF)]
#[case("0x1'23", 0x123)] // Digit separator
#[case("0xa'b'c", 0xabc)] // Multiple digit separators
#[case("0x0", 0x0)]
fn test_hexadecimal_constant_valid(#[case] input: &str, #[case] expected: i128) {
    let parser = hexadecimal_constant();
    let result = parser.parse(input).unwrap();
    assert_eq!(result, expected);
}

#[rstest]
#[case("0x")] // Missing hexadecimal digits
#[case("0X")] // Missing hexadecimal digits
#[case("123")] // Not hexadecimal format
#[case("")] // Empty string
fn test_hexadecimal_constant_invalid(#[case] input: &str) {
    let parser = hexadecimal_constant();
    let result = parser.parse(input);
    assert!(result.has_errors());
}

// =============================================================================
// Test binary constant parser
// =============================================================================

#[rstest]
#[case("0b101", 0b101)]
#[case("0B101", 0b101)]
#[case("0b0", 0b0)]
#[case("0b1'01", 0b101)] // Digit separator
#[case("0b1'0'1", 0b101)] // Multiple digit separators
fn test_binary_constant_valid(#[case] input: &str, #[case] expected: i128) {
    let parser = binary_constant();
    let result = parser.parse(input).unwrap();
    assert_eq!(result, expected);
}

#[rstest]
#[case("0b")] // Missing binary digits
#[case("0B")] // Missing binary digits
#[case("0b123")] // Contains invalid binary digit
#[case("101")] // Not binary format
#[case("")] // Empty string
fn test_binary_constant_invalid(#[case] input: &str) {
    let parser = binary_constant();
    let result = parser.parse(input);
    assert!(result.has_errors());
}

// =============================================================================
// Test integer constant parser
// =============================================================================

#[rstest]
#[case("123", 123, None)]
#[case("123u", 123, Some(IntegerSuffix::Unsigned))]
#[case("0x123", 0x123, None)]
#[case("0b101", 0b101, None)]
#[case("0123", 83, None)]
#[case("456L", 456, Some(IntegerSuffix::Long))]
#[case("0xFF", 0xFF, None)]
fn test_integer_constant_valid(
    #[case] input: &str,
    #[case] expected_value: i128,
    #[case] expected_suffix: Option<IntegerSuffix>,
) {
    let parser = integer_constant();
    let result = parser.parse(input).unwrap();
    assert_eq!(result.value, expected_value);
    assert_eq!(result.suffix, expected_suffix);
}

// =============================================================================
// Test floating suffix parser
// =============================================================================

#[rstest]
#[case("f", FloatingSuffix::F)]
#[case("F", FloatingSuffix::F)]
#[case("l", FloatingSuffix::L)]
#[case("L", FloatingSuffix::L)]
#[case("df", FloatingSuffix::DF)]
#[case("DF", FloatingSuffix::DF)]
#[case("dd", FloatingSuffix::DD)]
#[case("DD", FloatingSuffix::DD)]
#[case("dl", FloatingSuffix::DL)]
#[case("DL", FloatingSuffix::DL)]
fn test_floating_suffix_valid(#[case] input: &str, #[case] expected: FloatingSuffix) {
    let parser = floating_suffix();
    let result = parser.parse(input).unwrap();
    assert_eq!(result, expected);
}

// =============================================================================
// Test floating constant parser
// =============================================================================

#[rstest]
#[case("123.456", 123.456, None)]
#[case("123.456f", 123.456, Some(FloatingSuffix::F))]
#[case("123.456L", 123.456, Some(FloatingSuffix::L))]
#[case("1.23e10", 1.23e10, None)]
#[case("1.23E-10", 1.23E-10, None)]
#[case("1.23e+10", 1.23e+10, None)]
fn test_floating_constant_valid(
    #[case] input: &str,
    #[case] expected_value: f64,
    #[case] expected_suffix: Option<FloatingSuffix>,
) {
    let parser = floating_constant();
    let result = parser.parse(input).unwrap();
    assert_eq!(result.value, expected_value);
    assert_eq!(result.suffix, expected_suffix);
}

// =============================================================================
// Test character constant parser
// =============================================================================

#[rstest]
#[case("'a'", None, "a")]
#[case("u8'a'", Some(EncodingPrefix::U8), "a")]
#[case("u'a'", Some(EncodingPrefix::U), "a")]
#[case("U'a'", Some(EncodingPrefix::CapitalU), "a")]
#[case("L'a'", Some(EncodingPrefix::L), "a")]
#[case(r"'\n'", None, "\n")]
#[case(r"'\''", None, "'")]
#[case("'abc'", None, "abc")] // Multi-character constant
fn test_character_constant_valid(
    #[case] input: &str,
    #[case] expected_prefix: Option<EncodingPrefix>,
    #[case] expected_value: &str,
) {
    let parser = character_constant();
    let result = parser.parse(input).unwrap();
    assert_eq!(result.encoding_prefix, expected_prefix);
    assert_eq!(result.value, expected_value);
}

#[rstest]
#[case("'a")] // Missing closing quote
#[case("a'")] // Missing opening quote
#[case("''")] // Empty character constant
fn test_character_constant_invalid(#[case] input: &str) {
    let parser = character_constant();
    let result = parser.parse(input);
    assert!(result.has_errors());
}

// =============================================================================
// Test predefined constant parser
// =============================================================================

#[rstest]
#[case("false", PredefinedConstant::False)]
#[case("true", PredefinedConstant::True)]
#[case("nullptr", PredefinedConstant::Nullptr)]
fn test_predefined_constant_valid(#[case] input: &str, #[case] expected: PredefinedConstant) {
    let parser = predefined_constant();
    let result = parser.parse(input).unwrap();
    assert_eq!(result, expected);
}

#[rstest]
#[case("False")]
#[case("TRUE")]
#[case("null")]
#[case("NULL")]
fn test_predefined_constant_invalid(#[case] input: &str) {
    let parser = predefined_constant();
    let result = parser.parse(input);
    assert!(result.has_errors());
}

// =============================================================================
// Test constant parser
// =============================================================================

#[rstest]
#[case("123", |result: Constant| matches!(result, Constant::Integer(_)))]
#[case("123.456", |result: Constant| matches!(result, Constant::Floating(_)))]
#[case("'a'", |result: Constant| matches!(result, Constant::Character(_)))]
#[case("true", |result: Constant| matches!(result, Constant::Predefined(PredefinedConstant::True)))]
#[case("false", |result: Constant| matches!(result, Constant::Predefined(PredefinedConstant::False)))]
#[case("nullptr", |result: Constant| matches!(result, Constant::Predefined(PredefinedConstant::Nullptr)))]
#[case("MY_ENUM", |result: Constant| matches!(result, Constant::Enumeration(_)))]
fn test_constant_valid<F>(#[case] input: &str, #[case] validator: F)
where
    F: Fn(Constant) -> bool,
{
    let parser = constant();
    let result = parser.parse(input).unwrap();
    assert!(validator(result));
}

// =============================================================================
// Test simple punctuator parser
// =============================================================================

#[rstest]
#[case("++", Punctuator::Increment)]
#[case("--", Punctuator::Decrement)]
#[case("<<", Punctuator::LeftShift)]
#[case(">>", Punctuator::RightShift)]
#[case("<=", Punctuator::LessEqual)]
#[case(">=", Punctuator::GreaterEqual)]
#[case("==", Punctuator::Equal)]
#[case("!=", Punctuator::NotEqual)]
#[case("&&", Punctuator::LogicalAnd)]
#[case("||", Punctuator::LogicalOr)]
#[case("->", Punctuator::Arrow)]
#[case("::", Punctuator::Scope)]
#[case("...", Punctuator::Ellipsis)]
#[case("*=", Punctuator::MulAssign)]
#[case("/=", Punctuator::DivAssign)]
#[case("%=", Punctuator::ModAssign)]
#[case("+=", Punctuator::AddAssign)]
#[case("-=", Punctuator::SubAssign)]
#[case("<<=", Punctuator::LeftShiftAssign)]
#[case(">>=", Punctuator::RightShiftAssign)]
#[case("&=", Punctuator::AndAssign)]
#[case("^=", Punctuator::XorAssign)]
#[case("|=", Punctuator::OrAssign)]
#[case("##", Punctuator::HashHash)]
#[case(".", Punctuator::Dot)]
#[case("&", Punctuator::Ampersand)]
#[case("*", Punctuator::Star)]
#[case("+", Punctuator::Plus)]
#[case("-", Punctuator::Minus)]
#[case("~", Punctuator::Tilde)]
#[case("!", Punctuator::Bang)]
#[case("/", Punctuator::Slash)]
#[case("%", Punctuator::Percent)]
#[case("<", Punctuator::Less)]
#[case(">", Punctuator::Greater)]
#[case("^", Punctuator::Caret)]
#[case("|", Punctuator::Pipe)]
#[case("?", Punctuator::Question)]
#[case(":", Punctuator::Colon)]
#[case(";", Punctuator::Semicolon)]
#[case("=", Punctuator::Assign)]
#[case(",", Punctuator::Comma)]
#[case("#", Punctuator::Hash)]
fn test_simple_punctuator_valid(#[case] input: &str, #[case] expected: Punctuator) {
    let parser = simple_punctuator();
    let result = parser.parse(input).unwrap();
    assert_eq!(result, expected);
}

// =============================================================================
// Test unknown token parser
// =============================================================================

#[rstest]
#[case("@", "@")]
#[case("$", "$")]
#[case("\\", "\\")]
#[case("`", "`")]
#[case("€", "€")]
fn test_unknown_token_valid(#[case] input: &str, #[case] expected: &str) {
    let parser = unknown_token();
    let result = parser.parse(input).unwrap();
    assert_eq!(result, expected);
}

// =============================================================================
// Test balanced token sequence parser
// =============================================================================

#[rstest]
#[case("()", |result: BalancedTokenSequence| {
    result.tokens.len() == 1 && matches!(result.tokens[0], BalancedToken::Parenthesized(_))
})]
#[case("[]", |result: BalancedTokenSequence| {
    result.tokens.len() == 1 && matches!(result.tokens[0], BalancedToken::Bracketed(_))
})]
#[case("{}", |result: BalancedTokenSequence| {
    result.tokens.len() == 1 && matches!(result.tokens[0], BalancedToken::Braced(_))
})]
#[case("hello", |result: BalancedTokenSequence| {
    result.tokens.len() == 1 && matches!(result.tokens[0], BalancedToken::Identifier(_))
})]
#[case(r#""string""#, |result: BalancedTokenSequence| {
    result.tokens.len() == 1 && matches!(result.tokens[0], BalancedToken::StringLiteral(_))
})]
#[case("123", |result: BalancedTokenSequence| {
    result.tokens.len() == 1 && matches!(result.tokens[0], BalancedToken::Constant(_))
})]
#[case("+", |result: BalancedTokenSequence| {
    result.tokens.len() == 1 && matches!(result.tokens[0], BalancedToken::Punctuator(_))
})]
#[case("hello + world", |result: BalancedTokenSequence| {
    result.tokens.len() == 3
})]
#[case("(hello world)", |result: BalancedTokenSequence| {
    result.tokens.len() == 1 && matches!(result.tokens[0], BalancedToken::Parenthesized(_))
})]
#[case("((nested))", |result: BalancedTokenSequence| {
    result.tokens.len() == 1
})]
fn test_balanced_token_sequence_valid<F>(#[case] input: &str, #[case] validator: F)
where
    F: Fn(BalancedTokenSequence) -> bool,
{
    let parser = balanced_token_sequence();
    let result = parser.parse(input).unwrap();
    assert!(validator(result));
}

#[rstest]
#[case("(unclosed")] // Unmatched parentheses
#[case("unclosed)")] // Unmatched parentheses
#[case("[unclosed")] // Unmatched square brackets
#[case("unclosed]")] // Unmatched square brackets
#[case("{unclosed")] // Unmatched braces
#[case("unclosed}")] // Unmatched braces
fn test_balanced_token_sequence_invalid(#[case] input: &str) {
    let parser = balanced_token_sequence();
    let result = parser.parse(input);
    assert!(result.has_errors());
}
