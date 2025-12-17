use std::cell::RefCell;

use chumsky::{
    input::{Checkpoint, Cursor, MapExtra},
    inspector::Inspector,
    prelude::*,
    text::Char,
};
use slab::Slab;

use crate::{ast::*, span::*};

/// Utilities for the lexer.
pub mod lexer_utils {
    use super::*;

    pub type Extra<'a> = chumsky::extra::Full<Simple<'a, char>, State, ()>;

    #[derive(Clone)]
    pub struct State {
        pub line_begin: bool,
        pub cursor: usize,
        pub context: SourceContext,
        checkpoints: RefCell<Slab<SourceContext>>,
    }

    impl Default for State {
        fn default() -> Self {
            Self {
                line_begin: true,
                cursor: 0,
                context: SourceContext::default(),
                checkpoints: RefCell::new(Slab::new()),
            }
        }
    }

    impl State {
        pub fn new(file: Option<&str>) -> Self {
            Self {
                line_begin: true,
                cursor: 0,
                context: SourceContext {
                    file: file.map(|s| s.into()),
                    line: 1,
                    bol: 0,
                },
                checkpoints: RefCell::new(Slab::new()),
            }
        }
    }

    #[derive(Clone, Copy)]
    pub struct StateCheckpoint {
        line_begin: bool,
        cursor: usize,
        context: usize,
    }

    impl<'src> Inspector<'src, &'src str> for State {
        type Checkpoint = StateCheckpoint;

        fn on_token(&mut self, token: &char) {
            self.cursor += token.len_utf8();
            if token.is_newline() {
                self.line_begin = true;
                self.context.line += 1;
                self.context.bol = self.cursor;
            } else if self.line_begin && !token.is_whitespace() {
                self.line_begin = false;
            }
        }

        fn on_save<'parse>(&self, _cursor: &Cursor<'src, 'parse, &'src str>) -> Self::Checkpoint {
            let mut checkpoints = self.checkpoints.borrow_mut();
            let context = checkpoints.insert(self.context.clone());
            StateCheckpoint {
                line_begin: self.line_begin,
                cursor: self.cursor,
                context,
            }
        }

        fn on_rewind<'parse>(&mut self, marker: &Checkpoint<'src, 'parse, &'src str, Self::Checkpoint>) {
            let checkpoint = marker.inspector();
            self.line_begin = checkpoint.line_begin;
            self.cursor = checkpoint.cursor;
            self.context = self
                .checkpoints
                .borrow()
                .get(checkpoint.context)
                .expect("Invalid checkpoint")
                .clone();
        }
    }
}

use lexer_utils::*;

/// (6.4.2.1) identifier
pub fn identifier<'a>() -> impl Parser<'a, &'a str, Identifier, Extra<'a>> + Clone {
    text::ident().map(|value: &str| Identifier(value.into()))
}

/// (6.4.4) constant
pub fn constant<'a>() -> impl Parser<'a, &'a str, Constant, Extra<'a>> + Clone {
    choice((
        predefined_constant().map(Constant::Predefined),
        floating_constant().map(Constant::Floating),
        character_constant().map(Constant::Character),
        integer_constant().map(Constant::Integer),
    ))
}

/// (6.4.4.1) integer constant
pub fn integer_constant<'a>() -> impl Parser<'a, &'a str, IntegerConstant, Extra<'a>> + Clone {
    choice((
        hexadecimal_constant(),
        binary_constant(),
        octal_constant(),
        decimal_constant(),
    ))
    .then(integer_suffix().or_not())
    .map(|(value, suffix)| IntegerConstant { value, suffix })
}

/// (6.4.4.1) decimal constant
pub fn decimal_constant<'a>() -> impl Parser<'a, &'a str, i128, Extra<'a>> + Clone {
    regex(r"[1-9](?:'?[0-9])*")
        .map(|s: &str| s.replace("'", ""))
        .from_str::<i128>()
        .unwrapped()
}

/// (6.4.4.1) octal constant
pub fn octal_constant<'a>() -> impl Parser<'a, &'a str, i128, Extra<'a>> + Clone {
    choice((
        choice((just("0o"), just("0O"))).ignore_then(regex(r"[0-7](?:'?[0-7])*")),
        regex(r"0(?:'?[0-7])*"),
    ))
    .map(|s: &str| s.replace("'", ""))
    .map(|s| i128::from_str_radix(&s, 8))
    .unwrapped()
}

/// (6.4.4.1) hexadecimal constant
pub fn hexadecimal_constant<'a>() -> impl Parser<'a, &'a str, i128, Extra<'a>> + Clone {
    choice((just("0x"), just("0X")))
        .ignore_then(regex(r"[0-9a-fA-F](?:'?[0-9a-fA-F])*"))
        .map(|s: &str| s.replace("'", ""))
        .map(|s| i128::from_str_radix(&s, 16))
        .unwrapped()
}

/// (6.4.4.1) binary constant
pub fn binary_constant<'a>() -> impl Parser<'a, &'a str, i128, Extra<'a>> + Clone {
    choice((just("0b"), just("0B")))
        .ignore_then(regex(r"[01](?:'?[01])*"))
        .map(|s: &str| s.replace("'", ""))
        .map(|digits| i128::from_str_radix(&digits, 2))
        .unwrapped()
}

/// (6.4.4.1) integer suffix
pub fn integer_suffix<'a>() -> impl Parser<'a, &'a str, IntegerSuffix, Extra<'a>> + Clone {
    choice((
        // Unsigned + Long variations
        just("ull").or(just("ULL")).map(|_| IntegerSuffix::UnsignedLongLong),
        just("ul").or(just("UL")).map(|_| IntegerSuffix::UnsignedLong),
        just("llu").or(just("LLU")).map(|_| IntegerSuffix::UnsignedLongLong),
        just("lu").or(just("LU")).map(|_| IntegerSuffix::UnsignedLong),
        // Long variations
        just("ll").or(just("LL")).map(|_| IntegerSuffix::LongLong),
        just("l").or(just("L")).map(|_| IntegerSuffix::Long),
        // Unsigned
        just("u").or(just("U")).map(|_| IntegerSuffix::Unsigned),
        // Bit-precise suffixes
        just("uwb").or(just("UWB")).map(|_| IntegerSuffix::UnsignedBitPrecise),
        just("wb").or(just("WB")).map(|_| IntegerSuffix::BitPrecise),
    ))
}

/// (6.4.4.2) floating constant
pub fn floating_constant<'a>() -> impl Parser<'a, &'a str, FloatingConstant, Extra<'a>> + Clone {
    choice((decimal_floating_constant(), hexadecimal_floating_constant()))
        .then(floating_suffix().or_not())
        .map(|(value, suffix)| FloatingConstant { value, suffix })
}

/// (6.4.4.2) decimal floating constant
pub fn decimal_floating_constant<'a>() -> impl Parser<'a, &'a str, f64, Extra<'a>> + Clone {
    regex(r"(?:(?:\d+(?:'?\d+)*)?\.(?:\d+(?:'?\d+)*)|(?:\d+(?:'?\d+)*)\.)(?:[eE][+-]?(?:\d+(?:'?\d+)*))?|(?:\d+(?:'?\d+)*)(?:[eE][+-]?(?:\d+(?:'?\d+)*))")
        .map(|s: &str| s.replace("'", ""))
        .from_str::<f64>()
        .unwrapped()
}

/// (6.4.4.2) hexadecimal floating constant
pub fn hexadecimal_floating_constant<'a>() -> impl Parser<'a, &'a str, f64, Extra<'a>> + Clone {
    regex(r"(?:0[xX])(?:(?:[0-9a-fA-F]+(?:'?[0-9a-fA-F]+)*)?\.(?:[0-9a-fA-F]+(?:'?[0-9a-fA-F]+)*)|(?:[0-9a-fA-F]+(?:'?[0-9a-fA-F]+)*)\.?)(?:[pP][+-]?(?:\d+(?:'?\d+)*))")
        .map(|s: &str| s.replace("'", ""))
        .map(|s| hexf_parse::parse_hexf64(&s, false))
        .unwrapped()
}

/// (6.4.4.2) floating suffix
pub fn floating_suffix<'a>() -> impl Parser<'a, &'a str, FloatingSuffix, Extra<'a>> + Clone {
    choice((
        just("df").or(just("DF")).map(|_| FloatingSuffix::DF),
        just("dd").or(just("DD")).map(|_| FloatingSuffix::DD),
        just("dl").or(just("DL")).map(|_| FloatingSuffix::DL),
        just("f").or(just("F")).map(|_| FloatingSuffix::F),
        just("l").or(just("L")).map(|_| FloatingSuffix::L),
    ))
}

/// (6.4.4.4) encoding prefix
pub fn encoding_prefix<'a>() -> impl Parser<'a, &'a str, EncodingPrefix, Extra<'a>> + Clone {
    choice((
        just("u8").map(|_| EncodingPrefix::U8),
        just("u").map(|_| EncodingPrefix::U),
        just("U").map(|_| EncodingPrefix::CapitalU),
        just("L").map(|_| EncodingPrefix::L),
    ))
}

/// (6.4.4.4) escape sequence
pub fn escape_sequence<'a>() -> impl Parser<'a, &'a str, char, Extra<'a>> + Clone {
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

/// (6.4.4.4) character constant
pub fn character_constant<'a>() -> impl Parser<'a, &'a str, CharacterConstant, Extra<'a>> + Clone {
    let prefix = encoding_prefix().or_not();

    let c_char = choice((escape_sequence(), none_of("'\\")));

    let content = c_char.repeated().at_least(1).collect::<String>();

    prefix
        .then(content.delimited_by(just('\''), just('\'')))
        .map(|(encoding_prefix, value)| CharacterConstant { encoding_prefix, value })
}

/// (6.4.4.5) predefined constant
pub fn predefined_constant<'a>() -> impl Parser<'a, &'a str, PredefinedConstant, Extra<'a>> + Clone {
    text::ident().try_map(|name, span| match name {
        "false" => Ok(PredefinedConstant::False),
        "true" => Ok(PredefinedConstant::True),
        "nullptr" => Ok(PredefinedConstant::Nullptr),
        _ => Err(Simple::new(None, span)),
    })
}

/// (6.4.5) string-literal
pub fn string_literal<'a>() -> impl Parser<'a, &'a str, StringLiterals, Extra<'a>> + Clone {
    let prefix = encoding_prefix().or_not();

    let content = escape_sequence().or(none_of("\"\\")).repeated().collect::<String>();

    prefix
        .then(content.delimited_by(just('"'), just('"')))
        .map(|(encoding_prefix, value)| StringLiteral { encoding_prefix, value })
        .separated_by(whitespace())
        .at_least(1)
        .collect::<Vec<StringLiteral>>()
        .map(StringLiterals)
}

/// extension syntax: `xxx` for quoted strings
pub fn quoted_string<'a>() -> impl Parser<'a, &'a str, String, Extra<'a>> + Clone {
    let content = none_of("`").repeated().collect::<String>();

    content.delimited_by(just('`'), just('`'))
}

/// (6.4.6) punctuator (excluding parentheses and brackets)
pub fn punctuator<'a>() -> impl Parser<'a, &'a str, Punctuator, Extra<'a>> + Clone {
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

/// (6.7.12.1) balanced token
pub fn balanced_token<'a>(
    balanced_token_sequence: impl Parser<'a, &'a str, BalancedTokenSequence, Extra<'a>> + Clone,
) -> impl Parser<'a, &'a str, BalancedToken, Extra<'a>> + Clone {
    // Parenthesized: ( balanced-token-sequence? )
    let parenthesized = balanced_token_sequence
        .clone()
        .delimited_by(just('('), just(')'))
        .recover_with(via_parser(just('(').ignore_then(balanced_token_sequence.clone())));

    // Bracketed: [ balanced-token-sequence? ]
    let bracketed = balanced_token_sequence
        .clone()
        .delimited_by(just('['), just(']'))
        .recover_with(via_parser(just('[').ignore_then(balanced_token_sequence.clone())));

    // Braced: { balanced-token-sequence? }
    let braced = balanced_token_sequence
        .clone()
        .delimited_by(just('{'), just('}'))
        .recover_with(via_parser(just('{').ignore_then(balanced_token_sequence.clone())));

    // Other tokens (non-brackets)
    let identifier = identifier();
    let string_literal = string_literal();
    let quoted_string = quoted_string();
    let constant = constant();
    let punctuator = punctuator();
    let unknown_token = unknown();

    choice((
        parenthesized.map(BalancedToken::Parenthesized),
        bracketed.map(BalancedToken::Bracketed),
        braced.map(BalancedToken::Braced),
        string_literal.map(BalancedToken::StringLiteral),
        quoted_string.map(BalancedToken::QuotedString),
        constant.map(BalancedToken::Constant),
        identifier.map(BalancedToken::Identifier),
        punctuator.map(BalancedToken::Punctuator),
        unknown_token.to(BalancedToken::Unknown),
    ))
}

/// (6.7.12.1) balanced token sequence
pub fn balanced_token_sequence<'a>() -> impl Parser<'a, &'a str, BalancedTokenSequence, Extra<'a>> + Clone {
    recursive(|balanced_token_sequence| {
        balanced_token(balanced_token_sequence)
            .map_with(|token: BalancedToken, extra| {
                let range = SourceRange {
                    start: extra.span().start,
                    end: extra.span().end,
                    context: extra.state().context.clone(),
                };
                Spanned::new(token, range)
            })
            .separated_by(whitespace())
            .allow_leading()
            .allow_trailing()
            .collect::<Vec<_>>()
            .map_with(|tokens, extra| {
                let eoi = SourceRange {
                    start: extra.span().end,
                    end: extra.span().end,
                    context: extra.state().context.clone(),
                };
                BalancedTokenSequence { tokens, eoi }
            })
    })
}

/// any other token as unknown
pub fn unknown<'a>() -> impl Parser<'a, &'a str, (), Extra<'a>> + Clone {
    none_of(" \t\n\r()[]{}").repeated().at_least(1).collect()
}

fn whitespace<'a>() -> impl Parser<'a, &'a str, (), Extra<'a>> + Clone {
    line_directive()
        .or(comment())
        .or(text::whitespace().at_least(1))
        .repeated()
}

fn line_directive<'a>() -> impl Parser<'a, &'a str, (), Extra<'a>> + Clone {
    let pragma = just("#pragma").ignore_then(none_of("\n").repeated()).ignored();

    let line = just('#').ignore_then(custom(|inp| {
        let mut directive = String::new();
        while let Some(token) = inp.next() {
            directive.push(token);
            if token.is_newline() {
                break;
            }
        }
        let directive = directive.split_whitespace().collect::<Vec<_>>();
        let state: &mut State = inp.state();
        if let [line, file, ..] = &directive[..] {
            state.context.line = line.parse().unwrap();
            state.context.file = Some(file.trim_matches('"').to_string().into());
        }
        Ok(())
    }));

    empty()
        .try_map_with(|_, extra: &mut MapExtra<'_, '_, &'a str, Extra<'a>>| {
            if extra.state().line_begin {
                Ok(())
            } else {
                Err(Simple::new(None, extra.span()))
            }
        })
        .ignore_then(choice((pragma, line)))
}

fn comment<'a>() -> impl Parser<'a, &'a str, (), Extra<'a>> + Clone {
    choice((
        // Single-line comment
        just("//").ignore_then(none_of("\n").ignored().repeated()),
        // Multi-line comment
        just("/*")
            .ignore_then(any().and_is(just("*/").not()).ignored().repeated())
            .then_ignore(just("*/")),
    ))
}
