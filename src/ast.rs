//! <https://www.open-std.org/jtc1/sc22/wg14/www/docs/n3096.pdf>
#![allow(missing_docs)]

use std::{fmt, sync::Arc};

#[cfg(feature = "dbg-pls")]
use dbg_pls::DebugPls;
use ordered_float::NotNan;

use crate::span::{Span, Spanned};

// =============================================================================
// Spanned Type Aliases
// =============================================================================

/// An expression with source span information.
pub type Expr = Spanned<Expression>;

/// A statement with source span information.
pub type Stmt = Spanned<Statement>;

/// A declaration with source span information.
pub type Decl = Spanned<Declaration>;

/// A type name with source span information.
pub type TypeNm = Spanned<TypeName>;

/// A declarator with source span information.
pub type Declr = Spanned<Declarator>;

/// An identifier with source span information.
pub type Ident = Spanned<Identifier>;

// =============================================================================
// Lexical Elements (6.4)
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

/// Balanced token sequence (6.4.4.3)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BalancedTokenSequence {
    pub tokens: Vec<Spanned<BalancedToken>>,
    pub closed: bool,
    pub eoi: Span,
}

#[cfg(feature = "dbg-pls")]
impl DebugPls for BalancedTokenSequence {
    fn fmt(&self, f: dbg_pls::Formatter<'_>) {
        f.debug_list().entries(&self.tokens).finish()
    }
}

/// Balanced tokens (6.4.4.3)
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "dbg-pls", derive(DebugPls))]
pub enum BalancedToken {
    Parenthesized(BalancedTokenSequence),
    Bracketed(BalancedTokenSequence),
    Braced(BalancedTokenSequence),
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
    Unknown, // For any other tokens not explicitly defined
}

// =============================================================================
// Expressions (6.5)
// =============================================================================

/// Expression
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "dbg-pls", derive(DebugPls))]
pub struct Expression {
    /// The kind of expression.
    pub kind: ExpressionKind,
    /// The source span of the expression.
    pub span: Span,
}

impl Expression {
    /// Create a new expression with the given kind and span.
    pub fn new(kind: ExpressionKind, span: Span) -> Self {
        Self { kind, span }
    }

    /// Create a new expression with a default span.
    pub fn dummy(kind: ExpressionKind) -> Self {
        Self { kind, span: Span::default() }
    }
}

/// Expression kinds
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "dbg-pls", derive(DebugPls))]
pub enum ExpressionKind {
    Postfix(PostfixExpression),
    Unary(UnaryExpression),
    Cast(CastExpression),
    Binary(BinaryExpression),
    Conditional(ConditionalExpression),
    Assignment(AssignmentExpression),
    Comma(CommaExpression),
    Error,
}

/// Primary expressions (6.5.1)
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "dbg-pls", derive(DebugPls))]
pub enum PrimaryExpression {
    Identifier(Identifier),
    Constant(Constant),
    EnumerationConstant(Identifier),
    StringLiteral(StringLiterals),
    QuotedString(String),
    Parenthesized(Box<Expression>),
    Generic(GenericSelection),
    Error,
}

/// Generic selection (6.5.1.1)
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "dbg-pls", derive(DebugPls))]
pub struct GenericSelection {
    pub controlling_expression: Box<Expression>,
    pub associations: Vec<GenericAssociation>,
}

/// Generic association (6.5.1.1)
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "dbg-pls", derive(DebugPls))]
pub enum GenericAssociation {
    Type {
        type_name: TypeName,
        expression: Box<Expression>,
    },
    Default {
        expression: Box<Expression>,
    },
}

/// Postfix expressions (6.5.2)
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "dbg-pls", derive(DebugPls))]
pub enum PostfixExpression {
    Primary(PrimaryExpression),
    ArrayAccess {
        array: Box<PostfixExpression>,
        index: Box<Expression>,
    },
    FunctionCall {
        function: Box<PostfixExpression>,
        arguments: Vec<Expression>,
    },
    MemberAccess {
        object: Box<PostfixExpression>,
        member: Identifier,
    },
    MemberAccessPtr {
        object: Box<PostfixExpression>,
        member: Identifier,
    },
    PostIncrement(Box<PostfixExpression>),
    PostDecrement(Box<PostfixExpression>),
    CompoundLiteral(CompoundLiteral),
}

/// Compound literals (6.5.2.5)
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "dbg-pls", derive(DebugPls))]
pub struct CompoundLiteral {
    pub storage_class_specifiers: Vec<StorageClassSpecifier>,
    pub type_name: TypeName,
    pub initializer: BracedInitializer,
}

/// Unary expressions (6.5.3)
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "dbg-pls", derive(DebugPls))]
pub enum UnaryExpression {
    Postfix(PostfixExpression),
    PreIncrement(Box<UnaryExpression>),
    PreDecrement(Box<UnaryExpression>),
    Unary {
        operator: UnaryOperator,
        operand: Box<CastExpression>,
    },
    Sizeof(Box<UnaryExpression>),
    SizeofType(TypeName),
    Alignof(TypeName),
}

/// Unary operators (6.5.3)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "dbg-pls", derive(DebugPls))]
pub enum UnaryOperator {
    Address,
    Dereference,
    Plus,
    Minus,
    BitwiseNot,
    LogicalNot,
}

/// Cast expressions (6.5.4)
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "dbg-pls", derive(DebugPls))]
pub enum CastExpression {
    Unary(UnaryExpression),
    Cast {
        type_name: TypeName,
        expression: Box<CastExpression>,
    },
}

/// Binary expressions (6.5.14)
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "dbg-pls", derive(DebugPls))]
pub struct BinaryExpression {
    pub left: Box<Expression>,
    pub operator: BinaryOperator,
    pub right: Box<Expression>,
}

/// Binary operators (6.5.14)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "dbg-pls", derive(DebugPls))]
pub enum BinaryOperator {
    // Arithmetic
    Multiply,
    Divide,
    Modulo,
    Add,
    Subtract,

    // Bitwise
    LeftShift,
    RightShift,
    BitwiseAnd,
    BitwiseXor,
    BitwiseOr,

    // Relational
    Less,
    Greater,
    LessEqual,
    GreaterEqual,
    Equal,
    NotEqual,

    // Logical
    LogicalAnd,
    LogicalOr,
}

/// Conditional expressions (6.5.15)
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "dbg-pls", derive(DebugPls))]
pub struct ConditionalExpression {
    pub condition: Box<Expression>,
    pub then_expr: Box<Expression>,
    pub else_expr: Box<Expression>,
}

/// Assignment expressions (6.5.16)
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "dbg-pls", derive(DebugPls))]
pub struct AssignmentExpression {
    pub left: Box<Expression>,
    pub operator: AssignmentOperator,
    pub right: Box<Expression>,
}

/// Assignment operators (6.5.16)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "dbg-pls", derive(DebugPls))]
pub enum AssignmentOperator {
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
}

/// Comma expressions (6.5.17)
#[derive(Debug, Default, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "dbg-pls", derive(DebugPls))]
pub struct CommaExpression {
    pub expressions: Vec<Expression>,
}

/// Constant expressions (6.6)
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "dbg-pls", derive(DebugPls))]
pub enum ConstantExpression {
    Expression(Box<Expression>),
    Error,
}

// =============================================================================
// Declarations (6.7)
// =============================================================================

/// Declarations (6.7)
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "dbg-pls", derive(DebugPls))]
pub struct Declaration {
    /// The kind of declaration.
    pub kind: DeclarationKind,
    /// The source span of the declaration.
    pub span: Span,
}

impl Declaration {
    /// Create a new declaration with the given kind and span.
    pub fn new(kind: DeclarationKind, span: Span) -> Self {
        Self { kind, span }
    }

    /// Create a new declaration with a default span.
    pub fn dummy(kind: DeclarationKind) -> Self {
        Self { kind, span: Span::default() }
    }
}

/// Declaration kinds
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "dbg-pls", derive(DebugPls))]
pub enum DeclarationKind {
    Normal {
        attributes: Vec<AttributeSpecifier>,
        specifiers: DeclarationSpecifiers,
        declarators: Vec<InitDeclarator>,
    },
    Typedef {
        attributes: Vec<AttributeSpecifier>,
        specifiers: DeclarationSpecifiers,
        declarators: Vec<Declarator>,
    },
    StaticAssert(StaticAssertDeclaration),
    Attribute(Vec<AttributeSpecifier>),
    Error,
}

/// Declaration specifiers (6.7)
#[derive(Debug, Default, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "dbg-pls", derive(DebugPls))]
pub struct DeclarationSpecifiers {
    pub specifiers: Vec<DeclarationSpecifier>,
    pub attributes: Vec<AttributeSpecifier>,
}

/// Declaration specifiers (6.7)
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "dbg-pls", derive(DebugPls))]
pub enum DeclarationSpecifier {
    StorageClass(StorageClassSpecifier),
    TypeSpecifierQualifier(TypeSpecifierQualifier),
    Function(FunctionSpecifier),
}

/// Init declarators (6.7)
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "dbg-pls", derive(DebugPls))]
pub struct InitDeclarator {
    pub declarator: Declarator,
    pub initializer: Option<Initializer>,
}

/// Storage class specifiers (6.7.1)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "dbg-pls", derive(DebugPls))]
pub enum StorageClassSpecifier {
    Auto,
    Constexpr,
    Extern,
    Register,
    Static,
    ThreadLocal,
    Typedef,
}

/// Type specifiers (6.7.2)
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "dbg-pls", derive(DebugPls))]
pub enum TypeSpecifier {
    Void,
    Char,
    Short,
    Int,
    Long,
    Float,
    Double,
    Signed,
    Unsigned,
    BitInt(ConstantExpression),
    Bool,
    Complex,
    Decimal32,
    Decimal64,
    Decimal128,
    Atomic(AtomicTypeSpecifier),
    Struct(StructOrUnionSpecifier),
    Enum(EnumSpecifier),
    TypedefName(Identifier),
    Typeof(TypeofSpecifier),
}

/// Struct or union specifiers (6.7.2.1)
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "dbg-pls", derive(DebugPls))]
pub struct StructOrUnionSpecifier {
    pub kind: StructOrUnion,
    pub attributes: Vec<AttributeSpecifier>,
    pub identifier: Option<Identifier>,
    pub members: Option<Vec<MemberDeclaration>>,
}

/// Struct or union (6.7.2.1)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "dbg-pls", derive(DebugPls))]
pub enum StructOrUnion {
    Struct,
    Union,
}

/// Member declarations (6.7.2.1)
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "dbg-pls", derive(DebugPls))]
pub enum MemberDeclaration {
    Normal {
        attributes: Vec<AttributeSpecifier>,
        specifiers: SpecifierQualifierList,
        declarators: Vec<MemberDeclarator>,
    },
    StaticAssert(StaticAssertDeclaration),
    Error,
}

/// Specifier qualifier lists (6.7.2.1)
#[derive(Debug, Default, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "dbg-pls", derive(DebugPls))]
pub struct SpecifierQualifierList {
    pub items: Vec<TypeSpecifierQualifier>,
    pub attributes: Vec<AttributeSpecifier>,
}

/// Type specifier qualifiers (6.7.2.1)
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "dbg-pls", derive(DebugPls))]
pub enum TypeSpecifierQualifier {
    TypeSpecifier(TypeSpecifier),
    TypeQualifier(TypeQualifier),
    AlignmentSpecifier(AlignmentSpecifier),
}

/// Member declarators (6.7.2.1)
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "dbg-pls", derive(DebugPls))]
pub enum MemberDeclarator {
    Declarator(Declarator),
    BitField {
        declarator: Option<Declarator>,
        width: ConstantExpression,
    },
}

/// Enum specifiers (6.7.2.2)
#[derive(Debug, Default, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "dbg-pls", derive(DebugPls))]
pub struct EnumSpecifier {
    pub attributes: Vec<AttributeSpecifier>,
    pub identifier: Option<Identifier>,
    pub type_specifier: Option<SpecifierQualifierList>,
    pub enumerators: Option<Vec<Enumerator>>,
}

/// Enumerator (6.7.2.2)
#[derive(Debug, Default, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "dbg-pls", derive(DebugPls))]
pub struct Enumerator {
    pub name: Identifier,
    pub attributes: Vec<AttributeSpecifier>,
    pub value: Option<ConstantExpression>,
}

/// Atomic type specifiers (6.7.2.4)
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "dbg-pls", derive(DebugPls))]
pub struct AtomicTypeSpecifier {
    pub type_name: TypeName,
}

/// typeof specifiers (6.7.2.5)
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "dbg-pls", derive(DebugPls))]
pub enum TypeofSpecifier {
    Typeof(TypeofSpecifierArgument),
    TypeofUnqual(TypeofSpecifierArgument),
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "dbg-pls", derive(DebugPls))]
pub enum TypeofSpecifierArgument {
    Expression(Box<Expression>),
    TypeName(TypeName),
    Error,
}

/// Type qualifiers (6.7.3)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "dbg-pls", derive(DebugPls))]
pub enum TypeQualifier {
    Const,
    Restrict,
    Volatile,
    Atomic,
    Nonnull,
    Nullable,
    ThreadLocal,
}

/// Function specifiers (6.7.4)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "dbg-pls", derive(DebugPls))]
pub enum FunctionSpecifier {
    Inline,
    Noreturn,
}

/// Alignment specifiers (6.7.5)
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "dbg-pls", derive(DebugPls))]
pub enum AlignmentSpecifier {
    Type(TypeName),
    Expression(ConstantExpression),
}

/// Declarators (6.7.6)
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "dbg-pls", derive(DebugPls))]
pub enum Declarator {
    Direct(DirectDeclarator),
    Pointer {
        pointer: Pointer,
        declarator: Box<Declarator>,
    },
    Error,
}

impl Declarator {
    /// Get the identifier from the declarator. None if error inside.
    pub fn identifier(&self) -> Option<&Identifier> {
        match self {
            Declarator::Direct(direct) => direct.identifier(),
            Declarator::Pointer { declarator, .. } => declarator.identifier(),
            Declarator::Error => None,
        }
    }

    /// Get the parameter type list from the declarator. None if not a function
    /// declarator.
    pub fn parameters(&self) -> Option<&ParameterTypeList> {
        match self {
            Declarator::Direct(direct) => direct.parameters(),
            Declarator::Pointer { declarator, .. } => declarator.parameters(),
            Declarator::Error => None,
        }
    }
}

/// Direct declarators (6.7.6)
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "dbg-pls", derive(DebugPls))]
pub enum DirectDeclarator {
    Identifier {
        identifier: Identifier,
        attributes: Vec<AttributeSpecifier>,
    },
    Parenthesized(Box<Declarator>),
    Array {
        declarator: Box<DirectDeclarator>,
        attributes: Vec<AttributeSpecifier>,
        array_declarator: ArrayDeclarator,
    },
    Function {
        declarator: Box<DirectDeclarator>,
        attributes: Vec<AttributeSpecifier>,
        parameters: ParameterTypeList,
    },
}

impl DirectDeclarator {
    /// Get the identifier from the direct declarator. None if error inside.
    pub fn identifier(&self) -> Option<&Identifier> {
        match self {
            DirectDeclarator::Identifier { identifier, .. } => Some(identifier),
            DirectDeclarator::Parenthesized(declarator) => declarator.identifier(),
            DirectDeclarator::Array { declarator, .. } => declarator.identifier(),
            DirectDeclarator::Function { declarator, .. } => declarator.identifier(),
        }
    }

    /// Get the parameter type list from the direct declarator. None if not a
    /// function declarator.
    pub fn parameters(&self) -> Option<&ParameterTypeList> {
        match self {
            DirectDeclarator::Function { parameters, .. } => Some(parameters),
            DirectDeclarator::Parenthesized(declarator) => declarator.parameters(),
            DirectDeclarator::Array { declarator, .. } => declarator.parameters(),
            DirectDeclarator::Identifier { .. } => None,
        }
    }
}

/// Array declarators (6.7.6)
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "dbg-pls", derive(DebugPls))]
pub enum ArrayDeclarator {
    Normal {
        type_qualifiers: Vec<TypeQualifier>,
        size: Option<Box<Expression>>,
    },
    Static {
        type_qualifiers: Vec<TypeQualifier>,
        size: Box<Expression>,
    },
    VLA {
        type_qualifiers: Vec<TypeQualifier>,
    },
    Error,
}

/// Pointers (6.7.6)
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "dbg-pls", derive(DebugPls))]
pub struct Pointer {
    pub pointer_or_block: PointerOrBlock,
    pub attributes: Vec<AttributeSpecifier>,
    pub type_qualifiers: Vec<TypeQualifier>,
}

/// Pointer or block (clang extension)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "dbg-pls", derive(DebugPls))]
pub enum PointerOrBlock {
    Pointer,
    Block,
}

/// Parameter type lists (6.7.6)
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "dbg-pls", derive(DebugPls))]
pub enum ParameterTypeList {
    Parameters(Vec<ParameterDeclaration>),
    Variadic(Vec<ParameterDeclaration>),
    OnlyVariadic,
}

/// Parameter declarations (6.7.6)
#[derive(Debug, Default, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "dbg-pls", derive(DebugPls))]
pub struct ParameterDeclaration {
    pub attributes: Vec<AttributeSpecifier>,
    pub specifiers: DeclarationSpecifiers,
    pub declarator: Option<ParameterDeclarationKind>,
}

/// Parameter declaration kinds (6.7.6)
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "dbg-pls", derive(DebugPls))]
pub enum ParameterDeclarationKind {
    Declarator(Declarator),
    Abstract(AbstractDeclarator),
}

/// Type names (6.7.7)
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "dbg-pls", derive(DebugPls))]
pub enum TypeName {
    TypeName {
        specifiers: SpecifierQualifierList,
        abstract_declarator: Option<AbstractDeclarator>,
    },
    Error,
}

/// Abstract declarators (6.7.7)
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "dbg-pls", derive(DebugPls))]
pub enum AbstractDeclarator {
    Direct(DirectAbstractDeclarator),
    Pointer {
        pointer: Pointer,
        abstract_declarator: Option<Box<AbstractDeclarator>>,
    },
    Error,
}

/// Direct abstract declarators (6.7.7)
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "dbg-pls", derive(DebugPls))]
pub enum DirectAbstractDeclarator {
    Parenthesized(Box<AbstractDeclarator>),
    Array {
        declarator: Option<Box<DirectAbstractDeclarator>>,
        attributes: Vec<AttributeSpecifier>,
        array_declarator: ArrayDeclarator,
    },
    Function {
        declarator: Option<Box<DirectAbstractDeclarator>>,
        attributes: Vec<AttributeSpecifier>,
        parameters: ParameterTypeList,
    },
}

/// Initializers (6.7.10)
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "dbg-pls", derive(DebugPls))]
pub enum Initializer {
    Expression(Box<Expression>),
    Braced(BracedInitializer),
}

/// Braced initializers (6.7.10)
#[derive(Debug, Default, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "dbg-pls", derive(DebugPls))]
pub struct BracedInitializer {
    pub initializers: Vec<DesignatedInitializer>,
}

/// Designated initializers (6.7.10)
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "dbg-pls", derive(DebugPls))]
pub struct DesignatedInitializer {
    pub designation: Option<Designation>,
    pub initializer: Initializer,
}

/// Designation (6.7.10)
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "dbg-pls", derive(DebugPls))]
pub struct Designation {
    pub designator: Designator,
    pub designation: Option<Box<Designation>>,
}

/// Designators (6.7.10)
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "dbg-pls", derive(DebugPls))]
pub enum Designator {
    Array(ConstantExpression),
    Member(Identifier),
}

/// Static assert declarations (6.7.11)
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "dbg-pls", derive(DebugPls))]
pub struct StaticAssertDeclaration {
    pub condition: ConstantExpression,
    pub message: Option<StringLiterals>,
}

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
    /// Check if the attribute token is a prefixed attribute with the given
    /// prefix.
    pub fn is_prefixed(&self, prefix: &str) -> bool {
        matches!(self, AttributeToken::Prefixed { prefix: p, .. } if p.as_ref() == prefix)
    }

    /// Check if the attribute token is a standard attribute with the given
    /// name.
    pub fn is_standard(&self, name: &str) -> bool {
        matches!(self, AttributeToken::Standard(id) if id.as_ref() == name)
    }

    /// Get the prefix and identifier of a prefixed attribute token, or None if
    /// not prefixed.
    pub fn as_prefixed(&self) -> Option<(&Identifier, &Identifier)> {
        match self {
            AttributeToken::Prefixed { prefix, identifier } => Some((prefix, identifier)),
            _ => None,
        }
    }

    /// Get the identifier of a standard attribute token, or None if not
    /// standard.
    pub fn as_standard(&self) -> Option<&Identifier> {
        match self {
            AttributeToken::Standard(id) => Some(id),
            _ => None,
        }
    }

    /// If the attribute token is a prefixed attribute with the given prefix,
    /// return its identifier.
    pub fn get_identifier(&self, prefix: &str) -> Option<&Identifier> {
        match self {
            AttributeToken::Prefixed { prefix: p, identifier } if p.as_ref() == prefix => Some(identifier),
            _ => None,
        }
    }
}

// =============================================================================
// Statements (6.8)
// =============================================================================

/// Statements (6.8)
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "dbg-pls", derive(DebugPls))]
pub struct Statement {
    /// The kind of statement.
    pub kind: StatementKind,
    /// The source span of the statement.
    pub span: Span,
}

impl Statement {
    /// Create a new statement with the given kind and span.
    pub fn new(kind: StatementKind, span: Span) -> Self {
        Self { kind, span }
    }

    /// Create a new statement with a default span.
    pub fn dummy(kind: StatementKind) -> Self {
        Self { kind, span: Span::default() }
    }
}

/// Statement kinds
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "dbg-pls", derive(DebugPls))]
pub enum StatementKind {
    Labeled(LabeledStatement),
    Unlabeled(UnlabeledStatement),
}

/// Unlabeled statements (6.8)
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "dbg-pls", derive(DebugPls))]
pub enum UnlabeledStatement {
    Expression(ExpressionStatement),
    Primary {
        attributes: Vec<AttributeSpecifier>,
        block: PrimaryBlock,
    },
    Jump {
        attributes: Vec<AttributeSpecifier>,
        statement: JumpStatement,
    },
}

/// Primary blocks (6.8.4)
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "dbg-pls", derive(DebugPls))]
pub enum PrimaryBlock {
    Compound(CompoundStatement),
    Selection(SelectionStatement),
    Iteration(IterationStatement),
}

/// Labels (6.8.1)
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "dbg-pls", derive(DebugPls))]
pub enum Label {
    Identifier {
        attributes: Vec<AttributeSpecifier>,
        identifier: Identifier,
    },
    Case {
        attributes: Vec<AttributeSpecifier>,
        expression: ConstantExpression,
    },
    Default {
        attributes: Vec<AttributeSpecifier>,
    },
}

/// Labeled statements (6.8.1)
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "dbg-pls", derive(DebugPls))]
pub struct LabeledStatement {
    pub label: Label,
    pub statement: Box<Statement>,
}

/// Compound statements (6.8.2)
#[derive(Debug, Default, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "dbg-pls", derive(DebugPls))]
pub struct CompoundStatement {
    pub items: Vec<BlockItem>,
}

/// Block items (6.8.2)
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "dbg-pls", derive(DebugPls))]
pub enum BlockItem {
    Declaration(Declaration),
    Statement(UnlabeledStatement),
    Label(Label),
}

/// Expression statements (6.8.3)
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "dbg-pls", derive(DebugPls))]
pub struct ExpressionStatement {
    pub attributes: Vec<AttributeSpecifier>,
    pub expression: Option<Box<Expression>>,
}

/// Selection statements (6.8.4)
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "dbg-pls", derive(DebugPls))]
pub enum SelectionStatement {
    If {
        condition: Box<Expression>,
        then_stmt: Box<Statement>,
        else_stmt: Option<Box<Statement>>,
    },
    Switch {
        expression: Box<Expression>,
        statement: Box<Statement>,
    },
}

/// Iteration statements (6.8.5)
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "dbg-pls", derive(DebugPls))]
pub enum IterationStatement {
    While {
        condition: Box<Expression>,
        body: Box<Statement>,
    },
    DoWhile {
        body: Box<Statement>,
        condition: Box<Expression>,
    },
    For {
        init: Option<ForInit>,
        condition: Option<Box<Expression>>,
        update: Option<Box<Expression>>,
        body: Box<Statement>,
    },
    Error,
}

/// For initialization subclause (6.8.5)
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "dbg-pls", derive(DebugPls))]
pub enum ForInit {
    Expression(Box<Expression>),
    Declaration(Declaration),
}

/// Jump statements (6.8.6)
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "dbg-pls", derive(DebugPls))]
pub enum JumpStatement {
    Goto(Identifier),
    Continue,
    Break,
    Return(Option<Box<Expression>>),
}

// =============================================================================
// Translation Units (6.9)
// =============================================================================

/// Translation units (6.9)
#[derive(Debug, Default, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "dbg-pls", derive(DebugPls))]
pub struct TranslationUnit {
    pub external_declarations: Vec<ExternalDeclaration>,
}

/// External declarations (6.9)
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "dbg-pls", derive(DebugPls))]
pub enum ExternalDeclaration {
    Function(FunctionDefinition),
    Declaration(Declaration),
}

/// Function definitions (6.9.1)
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "dbg-pls", derive(DebugPls))]
pub struct FunctionDefinition {
    pub attributes: Vec<AttributeSpecifier>,
    pub specifiers: DeclarationSpecifiers,
    pub declarator: Declarator,
    pub body: CompoundStatement,
}

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
                match &mut token.value {
                    BalancedToken::Template(template) => {
                        let name = template.name.as_ref();
                        let value = mapping.get(&name).ok_or(format!("template slot `{name}` not given"))?;
                        token.value = BalancedToken::Interpolation(value.clone());
                    }
                    BalancedToken::Parenthesized(tokens) => tokens.interpolate(mapping)?,
                    BalancedToken::Bracketed(tokens) => tokens.interpolate(mapping)?,
                    BalancedToken::Braced(tokens) => tokens.interpolate(mapping)?,
                    _ => {}
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

#[cfg(test)]
mod test {
    fn assert_send<T: Send>() {}
    fn assert_sync<T: Sync>() {}

    #[test]
    fn test_send_sync() {
        assert_send::<super::TranslationUnit>();
        assert_sync::<super::TranslationUnit>();
    }
}
