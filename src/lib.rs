use chumsky::prelude::*;

pub mod ast;

pub use ast::*;

/// Parse an identifier
pub fn identifier<'a>() -> impl Parser<'a, &'a str, Identifier> + Clone {
    text::ident().map(|value: &str| Identifier(value.into()))
}

/// Parse a string literal
pub fn string_literal<'a>() -> impl Parser<'a, &'a str, StringLiteral> + Clone {
    just('"')
        .ignore_then(none_of('"').repeated().collect::<String>())
        .then_ignore(just('"'))
        .map(|value| StringLiteral {
            encoding_prefix: None,
            value,
        })
}

/// Parse an integer constant
pub fn integer_constant<'a>() -> impl Parser<'a, &'a str, IntegerConstant> + Clone {
    one_of('0'..='9')
        .repeated()
        .at_least(1)
        .collect::<String>()
        .map(|value| IntegerConstant {
            value,
            suffix: None,
        })
}

/// Parse a constant
pub fn constant<'a>() -> impl Parser<'a, &'a str, Constant> + Clone {
    integer_constant()
        .map(Constant::Integer)
        .or(identifier().map(Constant::Enumeration))
}

/// Parse simple punctuators for balanced tokens (excluding brackets)
pub fn simple_punctuator<'a>() -> impl Parser<'a, &'a str, Punctuator> + Clone {
    // Use separate parsers and chain them with or()
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

    let assignment_ops = choice((
        just("*=").map(|_| Punctuator::MulAssign),
        just("/=").map(|_| Punctuator::DivAssign),
        just("%=").map(|_| Punctuator::ModAssign),
        just("+=").map(|_| Punctuator::AddAssign),
        just("-=").map(|_| Punctuator::SubAssign),
        just("<<=").map(|_| Punctuator::LeftShiftAssign),
        just(">>=").map(|_| Punctuator::RightShiftAssign),
        just("&=").map(|_| Punctuator::AndAssign),
        just("^=").map(|_| Punctuator::XorAssign),
        just("|=").map(|_| Punctuator::OrAssign),
        just("##").map(|_| Punctuator::HashHash),
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

    compound_ops.or(assignment_ops).or(simple_ops)
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
