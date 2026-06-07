//! Lexical token types — the input vocabulary for the parser.
//!
//! These are NOT AST nodes. They are the token-level types produced by the lexer
//! and consumed by the parser.

#![allow(missing_docs)]

use std::{fmt, sync::Arc};

#[cfg(feature = "dbg-pls")]
use dbg_pls::DebugPls;
use ordered_float::NotNan;

use crate::span::{Span, Spanned};

// =============================================================================
// Identifier
// =============================================================================

/// Identifier (6.4.2.1)
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "dbg-pls", derive(DebugPls))]
pub struct Identifier(pub Arc<str>);

impl fmt::Display for Identifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<&str> for Identifier {
    fn from(s: &str) -> Self {
        Identifier(s.into())
    }
}

impl AsRef<str> for Identifier {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

// =============================================================================
// Constants (6.4.4)
// =============================================================================

/// Constants (6.4.4)
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "dbg-pls", derive(DebugPls))]
pub enum Constant {
    Integer(IntegerConstant),
    Floating(FloatingConstant),
    Character(CharacterConstant),
    Predefined(PredefinedConstant),
}

/// Integer constants (6.4.4.1)
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "dbg-pls", derive(DebugPls))]
pub struct IntegerConstant {
    pub value: i128,
    pub suffix: Option<IntegerSuffix>,
}

impl From<i128> for IntegerConstant {
    fn from(value: i128) -> Self {
        Self { value, suffix: None }
    }
}

/// Integer suffixes (6.4.4.1)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "dbg-pls", derive(DebugPls))]
pub enum IntegerSuffix {
    Unsigned,
    Long,
    LongLong,
    UnsignedLong,
    UnsignedLongLong,
    BitPrecise,
    UnsignedBitPrecise,
}

/// Floating-point constants (6.4.4.2)
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct FloatingConstant {
    pub value: NotNan<f64>,
    pub suffix: Option<FloatingSuffix>,
}

impl From<f64> for FloatingConstant {
    fn from(value: f64) -> Self {
        Self {
            value: value.try_into().unwrap(),
            suffix: None,
        }
    }
}

#[cfg(feature = "dbg-pls")]
impl DebugPls for FloatingConstant {
    fn fmt(&self, f: dbg_pls::Formatter<'_>) {
        f.debug_struct("FloatingConstant")
            .field("value", &self.value.into_inner())
            .field("suffix", &self.suffix)
            .finish()
    }
}

/// Floating-point suffixes (6.4.4.2)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "dbg-pls", derive(DebugPls))]
pub enum FloatingSuffix {
    F,
    L,
    DF,
    DD,
    DL,
}

/// Character constants (6.4.4.4)
#[derive(Debug, Default, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "dbg-pls", derive(DebugPls))]
pub struct CharacterConstant {
    pub encoding_prefix: Option<EncodingPrefix>,
    pub value: String,
}

impl From<char> for CharacterConstant {
    fn from(value: char) -> Self {
        Self {
            encoding_prefix: None,
            value: value.to_string(),
        }
    }
}

/// Encoding prefixes (6.4.4.4)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "dbg-pls", derive(DebugPls))]
pub enum EncodingPrefix {
    U8,
    U,
    CapitalU,
    L,
}

/// Predefined constants (6.4.4.5)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "dbg-pls", derive(DebugPls))]
pub enum PredefinedConstant {
    False,
    True,
    Nullptr,
}

impl From<bool> for PredefinedConstant {
    fn from(value: bool) -> Self {
        if value { Self::True } else { Self::False }
    }
}

// =============================================================================
// String Literals (6.4.5)
// =============================================================================

/// Concatenation of string literals (6.4.5)
#[derive(Debug, Default, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "dbg-pls", derive(DebugPls))]
pub struct StringLiterals(pub Vec<StringLiteral>);

impl StringLiterals {
    pub fn to_joined(&self) -> String {
        self.0.iter().map(|s| s.value.as_str()).collect::<Vec<_>>().join("")
    }
}

impl From<String> for StringLiterals {
    fn from(value: String) -> Self {
        Self(vec![StringLiteral { encoding_prefix: None, value }])
    }
}

/// String literal (6.4.5)
#[derive(Debug, Default, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "dbg-pls", derive(DebugPls))]
pub struct StringLiteral {
    pub encoding_prefix: Option<EncodingPrefix>,
    pub value: String,
}

// =============================================================================
// Punctuators (6.4.6)
// =============================================================================

/// Punctuators (6.4.6)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "dbg-pls", derive(DebugPls))]
pub enum Punctuator {
    // Brackets
    LeftBracket,
    RightBracket,
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,

    // Operators
    Dot,
    Arrow,
    Increment,
    Decrement,
    Ampersand,
    Star,
    Plus,
    Minus,
    Tilde,
    Bang,
    Slash,
    Percent,
    LeftShift,
    RightShift,
    Less,
    Greater,
    LessEqual,
    GreaterEqual,
    Equal,
    NotEqual,
    Caret,
    Pipe,
    LogicalAnd,
    LogicalOr,
    Question,
    Colon,
    Scope,
    Semicolon,
    Ellipsis,

    // Assignment
    Assign,
    MulAssign,
    DivAssign,
    ModAssign,
    AddAssign,
    SubAssign,
    LeftShiftAssign,
    RightShiftAssign,
    AndAssign,
    XorAssign,
    OrAssign,

    // Other
    Comma,
    Hash,
    HashHash,
}

// =============================================================================
// Balanced Token Sequence (input to the parser)
// =============================================================================

/// A flat sequence of tokens — the parser's input.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BalancedTokenSequence {
    pub tokens: Vec<Spanned<BalancedToken>>,
    pub eoi: Span,
}

#[cfg(feature = "dbg-pls")]
impl DebugPls for BalancedTokenSequence {
    fn fmt(&self, f: dbg_pls::Formatter<'_>) {
        f.debug_list().entries(&self.tokens).finish()
    }
}

/// A single token in the lexer output.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "dbg-pls", derive(DebugPls))]
pub enum BalancedToken {
    Identifier(Identifier),
    StringLiteral(StringLiterals),
    /// extension syntax: `xxx` for quoted strings
    QuotedString(String),
    Constant(Constant),
    Punctuator(Punctuator),
    #[cfg(feature = "quasi-quote")]
    Template(quasi_quote::Template),
    #[cfg(feature = "quasi-quote")]
    Interpolation(Box<dyn quasi_quote::Interpolate + 'static>),
    Unknown,
}

// =============================================================================
// Attribute Specifiers (6.7.12.1)
// =============================================================================

/// Attribute specifiers (6.7.12.1)
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "dbg-pls", derive(DebugPls))]
pub enum AttributeSpecifier {
    Attributes(Vec<Attribute>),
    Asm(StringLiterals),
    Error,
}

impl AttributeSpecifier {
    pub fn try_into_attributes(self) -> Option<Vec<Attribute>> {
        match self {
            AttributeSpecifier::Attributes(attrs) => Some(attrs),
            _ => None,
        }
    }

    pub fn try_into_asm(self) -> Option<StringLiterals> {
        match self {
            AttributeSpecifier::Asm(asm) => Some(asm),
            _ => None,
        }
    }
}

/// Attribute (6.7.12.1)
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "dbg-pls", derive(DebugPls))]
pub struct Attribute {
    pub token: AttributeToken,
    pub arguments: Option<BalancedTokenSequence>,
}

/// Attribute tokens (6.7.12.1)
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "dbg-pls", derive(DebugPls))]
pub enum AttributeToken {
    Standard(Identifier),
    Prefixed { prefix: Identifier, identifier: Identifier },
}

impl AttributeToken {
    pub fn is_prefixed(&self, prefix: &str) -> bool {
        matches!(self, AttributeToken::Prefixed { prefix: p, .. } if p.as_ref() == prefix)
    }

    pub fn is_standard(&self, name: &str) -> bool {
        matches!(self, AttributeToken::Standard(id) if id.as_ref() == name)
    }

    pub fn as_prefixed(&self) -> Option<(&Identifier, &Identifier)> {
        match self {
            AttributeToken::Prefixed { prefix, identifier } => Some((prefix, identifier)),
            _ => None,
        }
    }

    pub fn as_standard(&self) -> Option<&Identifier> {
        match self {
            AttributeToken::Standard(id) => Some(id),
            _ => None,
        }
    }

    pub fn get_identifier(&self, prefix: &str) -> Option<&Identifier> {
        match self {
            AttributeToken::Prefixed { prefix: p, identifier } if p.as_ref() == prefix => Some(identifier),
            _ => None,
        }
    }
}

// =============================================================================
// Quasi-quote (feature-gated)
// =============================================================================

#[cfg(feature = "quasi-quote")]
pub mod quasi_quote {
    use std::{any::Any, collections::HashMap};

    use dyn_clone::DynClone;
    use dyn_eq::DynEq;

    use super::*;

    pub trait NamedAny: Any {
        fn type_name(&self) -> &'static str;
    }

    impl<T: Any + Sized> NamedAny for T {
        fn type_name(&self) -> &'static str {
            std::any::type_name::<T>()
        }
    }

    pub trait Interpolate: NamedAny + DynClone + DynEq + Send + Sync {}
    impl<T: Any + DynClone + DynEq + Send + Sync> Interpolate for T {}

    dyn_clone::clone_trait_object!(Interpolate);
    dyn_eq::eq_trait_object!(Interpolate);

    impl std::fmt::Debug for Box<dyn Interpolate> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            if let Some(template) = (self.as_ref() as &dyn Any).downcast_ref::<Template>() {
                write!(f, "{template:#?}")
            } else {
                f.debug_struct("Interpolate")
                    .field("type_name", &self.as_ref().type_name())
                    .finish()
            }
        }
    }

    #[cfg(feature = "dbg-pls")]
    impl dbg_pls::DebugPls for Box<dyn Interpolate> {
        fn fmt(&self, f: dbg_pls::Formatter<'_>) {
            if let Some(template) = (self.as_ref() as &dyn Any).downcast_ref::<Template>() {
                dbg_pls::DebugPls::fmt(template, f);
            } else {
                f.debug_struct("Interpolate")
                    .field("type_name", &self.as_ref().type_name())
                    .finish();
            }
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    #[cfg_attr(feature = "dbg-pls", derive(DebugPls))]
    pub struct Template {
        pub name: Arc<str>,
    }

    impl BalancedTokenSequence {
        pub fn interpolate(&mut self, mapping: &HashMap<&'static str, Box<dyn Interpolate>>) -> Result<(), String> {
            for token in &mut self.tokens {
                if let BalancedToken::Template(template) = &mut token.value {
                    let name = template.name.as_ref();
                    let value = mapping.get(&name).ok_or(format!("template slot `{name}` not given"))?;
                    token.value = BalancedToken::Interpolation(value.clone());
                }
            }
            Ok(())
        }
    }

    #[macro_export]
    macro_rules! interpolate {
        ($($name:ident => $value:expr),* $(,)?) => {
            [
                $((
                    stringify!($name),
                    ::std::boxed::Box::new($value) as ::std::boxed::Box<dyn $crate::quasi_quote::Interpolate>,
                ),)*
            ].into_iter().collect::<::std::collections::HashMap<_, _>>()
        }
    }
}
