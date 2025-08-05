use crate::ast::*;
use chumsky::prelude::*;

/// Parse an identifier
pub fn identifier<'a>() -> impl Parser<'a, &'a str, Identifier> + Clone {
    text::ident().map(|value: &str| Identifier(value.into()))
}

/// Parse an encoding prefix for string/character literals
pub fn encoding_prefix<'a>() -> impl Parser<'a, &'a str, EncodingPrefix> + Clone {
    choice((
        just("u8").map(|_| EncodingPrefix::U8),
        just("u").map(|_| EncodingPrefix::U),
        just("U").map(|_| EncodingPrefix::CapitalU),
        just("L").map(|_| EncodingPrefix::L),
    ))
}

/// Parse escape sequences in strings and characters
pub fn escape_sequence<'a>() -> impl Parser<'a, &'a str, char> + Clone {
    just('\\').ignore_then(choice((
        // Simple escape sequences
        just('\'').map(|_| '\''),
        just('"').map(|_| '"'),
        just('?').map(|_| '?'),
        just('\\').map(|_| '\\'),
        just('a').map(|_| '\x07'),
        just('b').map(|_| '\x08'),
        just('f').map(|_| '\x0C'),
        just('n').map(|_| '\n'),
        just('r').map(|_| '\r'),
        just('t').map(|_| '\t'),
        just('v').map(|_| '\x0B'),
        // Octal escape sequence (\ooo)
        one_of('0'..='7')
            .repeated()
            .at_least(1)
            .at_most(3)
            .collect::<String>()
            .map(|digits| char::from_u32(u32::from_str_radix(&digits, 8).unwrap()).unwrap()),
        // Hexadecimal escape sequence (\xhh)
        just('x')
            .ignore_then(
                one_of('0'..='9')
                    .or(one_of('a'..='f'))
                    .or(one_of('A'..='F'))
                    .repeated()
                    .at_least(1)
                    .collect::<String>(),
            )
            .map(|digits| char::from_u32(u32::from_str_radix(&digits, 16).unwrap()).unwrap()),
        // Universal character names (\uxxxx or \Uxxxxxxxx)
        just('u')
            .ignore_then(
                one_of('0'..='9')
                    .or(one_of('a'..='f'))
                    .or(one_of('A'..='F'))
                    .repeated()
                    .exactly(4)
                    .collect::<String>(),
            )
            .map(|digits| char::from_u32(u32::from_str_radix(&digits, 16).unwrap()).unwrap()),
        just('U')
            .ignore_then(
                one_of('0'..='9')
                    .or(one_of('a'..='f'))
                    .or(one_of('A'..='F'))
                    .repeated()
                    .exactly(8)
                    .collect::<String>(),
            )
            .map(|digits| char::from_u32(u32::from_str_radix(&digits, 16).unwrap()).unwrap()),
        // Fallback: just return the character itself
        any(),
    )))
}

/// Parse a string literal with full C23 support
pub fn string_literal<'a>() -> impl Parser<'a, &'a str, StringLiteral> + Clone {
    let prefix = encoding_prefix().or_not();

    let content = escape_sequence()
        .or(none_of("\"\\"))
        .repeated()
        .collect::<String>();

    prefix
        .then(content.delimited_by(just('"'), just('"')))
        .map(|(encoding_prefix, value)| StringLiteral {
            encoding_prefix,
            value,
        })
}

/// Parse integer suffixes
pub fn integer_suffix<'a>() -> impl Parser<'a, &'a str, IntegerSuffix> + Clone {
    choice((
        // Unsigned + Long variations
        just("ull")
            .or(just("ULL"))
            .map(|_| IntegerSuffix::UnsignedLongLong),
        just("ul")
            .or(just("UL"))
            .map(|_| IntegerSuffix::UnsignedLong),
        just("llu")
            .or(just("LLU"))
            .map(|_| IntegerSuffix::UnsignedLongLong),
        just("lu")
            .or(just("LU"))
            .map(|_| IntegerSuffix::UnsignedLong),
        // Long variations
        just("ll").or(just("LL")).map(|_| IntegerSuffix::LongLong),
        just("l").or(just("L")).map(|_| IntegerSuffix::Long),
        // Unsigned
        just("u").or(just("U")).map(|_| IntegerSuffix::Unsigned),
        // Bit-precise suffixes
        just("uwb")
            .or(just("UWB"))
            .map(|_| IntegerSuffix::UnsignedBitPrecise),
        just("wb").or(just("WB")).map(|_| IntegerSuffix::BitPrecise),
    ))
}

/// Parse decimal integer constant (no leading zero except for 0 itself)
pub fn decimal_constant<'a>() -> impl Parser<'a, &'a str, i128> + Clone {
    regex(r"[1-9]('?[0-9]+)*")
        .map(|s: &str| s.replace("'", ""))
        .from_str::<i128>()
        .unwrapped()
}

/// Parse octal integer constant (starts with 0)
pub fn octal_constant<'a>() -> impl Parser<'a, &'a str, i128> + Clone {
    regex(r"0('?[0-7]+)*")
        .map(|s: &str| s.replace("'", ""))
        .map(|s| i128::from_str_radix(&s, 8))
        .unwrapped()
}

/// Parse hexadecimal integer constant
pub fn hexadecimal_constant<'a>() -> impl Parser<'a, &'a str, i128> + Clone {
    choice((just("0x"), just("0X")))
        .ignore_then(regex(r"[0-9a-fA-F]('?[0-9a-fA-F]+)*"))
        .map(|s: &str| s.replace("'", ""))
        .map(|s| i128::from_str_radix(&s, 16))
        .unwrapped()
}

/// Parse binary integer constant
pub fn binary_constant<'a>() -> impl Parser<'a, &'a str, i128> + Clone {
    choice((just("0b"), just("0B")))
        .ignore_then(regex(r"[01]('?[01]+)*"))
        .map(|s: &str| s.replace("'", ""))
        .map(|digits| i128::from_str_radix(&digits, 2))
        .unwrapped()
}

/// Parse an integer constant with full C23 support
pub fn integer_constant<'a>() -> impl Parser<'a, &'a str, IntegerConstant> + Clone {
    choice((
        hexadecimal_constant(),
        binary_constant(),
        octal_constant(),
        decimal_constant(),
    ))
    .then(integer_suffix().or_not())
    .map(|(value, suffix)| IntegerConstant { value, suffix })
}

/// Parse floating point suffixes
pub fn floating_suffix<'a>() -> impl Parser<'a, &'a str, FloatingSuffix> + Clone {
    choice((
        just("df").or(just("DF")).map(|_| FloatingSuffix::DF),
        just("dd").or(just("DD")).map(|_| FloatingSuffix::DD),
        just("dl").or(just("DL")).map(|_| FloatingSuffix::DL),
        just("f").or(just("F")).map(|_| FloatingSuffix::F),
        just("l").or(just("L")).map(|_| FloatingSuffix::L),
    ))
}

/// Parse a decimal floating constant
pub fn decimal_floating_constant<'a>() -> impl Parser<'a, &'a str, f64> + Clone {
    regex(r"[0-9]+('[0-9]+)*\.[0-9]+('[0-9]+)*([eE][+-]?[0-9]+('[0-9]+)*)?")
        .map(|s: &str| s.replace("'", ""))
        .from_str::<f64>()
        .unwrapped()
}

/// Parse a hexadecimal floating constant
pub fn hexadecimal_floating_constant<'a>() -> impl Parser<'a, &'a str, f64> + Clone {
    regex(r"0[xX][0-9a-fA-F]+('[0-9a-fA-F]+)*\.[0-9a-fA-F]+('[0-9a-fA-F]+)*([pP][+-]?[0-9]+('[0-9]+)*)?")
        .map(|s: &str| s.replace("'", ""))
        .map(|s| hexf_parse::parse_hexf64(&s, false))
        .unwrapped()
}

/// Parse a floating constant
pub fn floating_constant<'a>() -> impl Parser<'a, &'a str, FloatingConstant> + Clone {
    choice((decimal_floating_constant(), hexadecimal_floating_constant()))
        .then(floating_suffix().or_not())
        .map(|(value, suffix)| FloatingConstant { value, suffix })
}

/// Parse a character constant
pub fn character_constant<'a>() -> impl Parser<'a, &'a str, CharacterConstant> + Clone {
    let prefix = encoding_prefix().or_not();

    let c_char = choice((escape_sequence(), none_of("'\\")));

    let content = c_char.repeated().at_least(1).collect::<String>();

    prefix
        .then(content.delimited_by(just('\''), just('\'')))
        .map(|(encoding_prefix, value)| CharacterConstant {
            encoding_prefix,
            value,
        })
}

/// Parse predefined constants
pub fn predefined_constant<'a>() -> impl Parser<'a, &'a str, PredefinedConstant> + Clone {
    choice((
        just("false").map(|_| PredefinedConstant::False),
        just("true").map(|_| PredefinedConstant::True),
        just("nullptr").map(|_| PredefinedConstant::Nullptr),
    ))
}

/// Parse a constant with full C23 support
pub fn constant<'a>() -> impl Parser<'a, &'a str, Constant> + Clone {
    choice((
        predefined_constant().map(Constant::Predefined),
        floating_constant().map(Constant::Floating),
        character_constant().map(Constant::Character),
        integer_constant().map(Constant::Integer),
        identifier().map(Constant::Enumeration),
    ))
}

/// Parse simple punctuators for balanced tokens (excluding brackets)
pub fn simple_punctuator<'a>() -> impl Parser<'a, &'a str, Punctuator> + Clone {
    // Use separate parsers and chain them with or()
    // Put longer operators first to avoid partial matches
    let assignment_ops = choice((
        just("<<=").map(|_| Punctuator::LeftShiftAssign),
        just(">>=").map(|_| Punctuator::RightShiftAssign),
        just("*=").map(|_| Punctuator::MulAssign),
        just("/=").map(|_| Punctuator::DivAssign),
        just("%=").map(|_| Punctuator::ModAssign),
        just("+=").map(|_| Punctuator::AddAssign),
        just("-=").map(|_| Punctuator::SubAssign),
        just("&=").map(|_| Punctuator::AndAssign),
        just("^=").map(|_| Punctuator::XorAssign),
        just("|=").map(|_| Punctuator::OrAssign),
        just("##").map(|_| Punctuator::HashHash),
    ));

    let compound_ops = choice((
        just("++").map(|_| Punctuator::Increment),
        just("--").map(|_| Punctuator::Decrement),
        just("<<").map(|_| Punctuator::LeftShift),
        just(">>").map(|_| Punctuator::RightShift),
        just("<=").map(|_| Punctuator::LessEqual),
        just(">=").map(|_| Punctuator::GreaterEqual),
        just("==").map(|_| Punctuator::Equal),
        just("!=").map(|_| Punctuator::NotEqual),
        just("&&").map(|_| Punctuator::LogicalAnd),
        just("||").map(|_| Punctuator::LogicalOr),
        just("->").map(|_| Punctuator::Arrow),
        just("::").map(|_| Punctuator::Scope),
        just("...").map(|_| Punctuator::Ellipsis),
    ));

    let simple_ops = choice((
        just(".").map(|_| Punctuator::Dot),
        just("&").map(|_| Punctuator::Ampersand),
        just("*").map(|_| Punctuator::Star),
        just("+").map(|_| Punctuator::Plus),
        just("-").map(|_| Punctuator::Minus),
        just("~").map(|_| Punctuator::Tilde),
        just("!").map(|_| Punctuator::Bang),
        just("/").map(|_| Punctuator::Slash),
        just("%").map(|_| Punctuator::Percent),
        just("<").map(|_| Punctuator::Less),
        just(">").map(|_| Punctuator::Greater),
        just("^").map(|_| Punctuator::Caret),
        just("|").map(|_| Punctuator::Pipe),
        just("?").map(|_| Punctuator::Question),
        just(":").map(|_| Punctuator::Colon),
        just(";").map(|_| Punctuator::Semicolon),
        just("=").map(|_| Punctuator::Assign),
        just(",").map(|_| Punctuator::Comma),
        just("#").map(|_| Punctuator::Hash),
    ));

    assignment_ops.or(compound_ops).or(simple_ops)
}

/// Parse any other token as unknown
pub fn unknown_token<'a>() -> impl Parser<'a, &'a str, String> + Clone {
    none_of(" \t\n\r()[]{}")
        .repeated()
        .at_least(1)
        .collect::<String>()
}

/// Parse a balanced token
pub fn balanced_token<'a>(
    balanced_token_sequence: impl Parser<'a, &'a str, BalancedTokenSequence> + Clone,
) -> impl Parser<'a, &'a str, BalancedToken> + Clone {
    // Parenthesized: ( balanced-token-sequence? )
    let parenthesized = balanced_token_sequence
        .clone()
        .or_not()
        .padded()
        .delimited_by(just('('), just(')'))
        .map(BalancedToken::Parenthesized);

    // Bracketed: [ balanced-token-sequence? ]
    let bracketed = balanced_token_sequence
        .clone()
        .or_not()
        .padded()
        .delimited_by(just('['), just(']'))
        .map(BalancedToken::Bracketed);

    // Braced: { balanced-token-sequence? }
    let braced = balanced_token_sequence
        .clone()
        .or_not()
        .padded()
        .delimited_by(just('{'), just('}'))
        .map(BalancedToken::Braced);

    // Other tokens (non-brackets)
    let identifier_token = identifier().map(BalancedToken::Identifier);
    let string_token = string_literal().map(BalancedToken::StringLiteral);
    let constant_token = constant().map(BalancedToken::Constant);
    let punctuator_token = simple_punctuator().map(BalancedToken::Punctuator);
    let unknown_token_parser = unknown_token().map(BalancedToken::Unknown);

    choice((
        parenthesized,
        bracketed,
        braced,
        identifier_token,
        string_token,
        constant_token,
        punctuator_token,
        unknown_token_parser,
    ))
}

/// Parse a balanced token sequence
pub fn balanced_token_sequence<'a>() -> impl Parser<'a, &'a str, BalancedTokenSequence> {
    recursive(|balanced_token_sequence| {
        balanced_token(balanced_token_sequence)
            .padded()
            .repeated()
            .collect::<Vec<_>>()
            .map(|tokens| BalancedTokenSequence { tokens })
    })
}
