//! Syntax kinds for the red-green tree.
//!
//! Every node and token in the green tree is tagged with a [`SyntaxKind`].
//! This is the only type-level distinction between different grammatical
//! categories in the green tree.

#![allow(missing_docs)]

/// The kind of a syntax node or token.
///
/// Uses a `u16` representation for compact storage in the green tree.
/// Variants are grouped by category: tokens, expressions, declarations,
/// statements, and top-level constructs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SyntaxKind {
    // =========================================================================
    // Trivia
    // =========================================================================
    Whitespace,
    Comment,

    // =========================================================================
    // Tokens: identifiers, literals, etc.
    // =========================================================================
    Ident,
    IntegerConst,
    FloatConst,
    CharConst,
    PredefinedConst,
    StringLiteral,
    QuotedString,
    Unknown,

    // =========================================================================
    // Punctuators (6.4.6) — one per variant for lossless round-trip
    // =========================================================================
    // Brackets
    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,
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

    // =========================================================================
    // Expressions (6.5)
    // =========================================================================
    // Primary expressions (6.5.1)
    PrimaryExpr,
    GenericSelection,
    GenericAssociationType,
    GenericAssociationDefault,

    // Postfix expressions (6.5.2)
    PostfixExpr,
    ArrayAccess,
    FunctionCall,
    MemberAccess,
    MemberAccessPtr,
    PostIncrement,
    PostDecrement,
    CompoundLiteral,

    // Unary expressions (6.5.3)
    UnaryExpr,
    PreIncrement,
    PreDecrement,
    UnaryOp,
    SizeofExpr,
    SizeofType,
    Alignof,

    // Cast expressions (6.5.4)
    CastExpr,
    Cast,

    // Binary expressions (6.5.14)
    BinaryExpr,

    // Conditional expressions (6.5.15)
    ConditionalExpr,

    // Assignment expressions (6.5.16)
    AssignmentExpr,

    // Comma expressions (6.5.17)
    CommaExpr,

    /// Generic expression (6.5)
    Expr,

    /// Error expression (recovery)
    ExprError,

    /// Constant expression (6.6)
    ConstantExpr,

    // =========================================================================
    // Declarations (6.7)
    // =========================================================================
    Declaration,
    NormalDecl,
    TypedefDecl,
    StaticAssertDecl,
    AttributeDecl,
    DeclError,

    DeclarationSpecifiers,
    DeclarationSpecifier,
    InitDeclarator,

    // Type specifiers (6.7.2)
    TypeSpecifier,
    StructSpecifier,
    EnumSpecifier,
    AtomicTypeSpecifier,
    TypeofSpecifier,
    TypeofSpecifierArg,

    // Type qualifiers, storage classes, function specifiers, alignment
    TypeQualifier,
    StorageClassSpecifier,
    FunctionSpecifier,
    AlignmentSpecifier,
    TypeSpecifierQualifier,

    // Struct/union members (6.7.2.1)
    MemberDeclaration,
    SpecifierQualifierList,
    MemberDeclarator,
    MemberBitField,
    StructOrUnion,
    Enumerator,

    // Declarators (6.7.6)
    Declarator,
    DirectDeclarator,
    DirectDeclaratorIdent,
    DirectDeclaratorParen,
    DirectDeclaratorArray,
    DirectDeclaratorFunc,
    Pointer,
    PointerOrBlock,

    ArrayDeclarator,
    ArrayDeclaratorNormal,
    ArrayDeclaratorStatic,
    ArrayDeclaratorVLA,

    ParameterTypeList,
    ParameterDeclaration,
    ParameterDeclarationKind,

    // Type names (6.7.7)
    TypeName,
    AbstractDeclarator,
    DirectAbstractDeclarator,
    DirectAbstractDeclaratorParen,
    DirectAbstractDeclaratorArray,
    DirectAbstractDeclaratorFunc,

    // Initializers (6.7.10)
    Initializer,
    BracedInitializer,
    DesignatedInitializer,
    Designation,
    Designator,

    // Attributes (6.7.12.1)
    AttributeSpecifier,
    Attribute,
    AttributeToken,
    AsmAttribute,

    // =========================================================================
    // Statements (6.8)
    // =========================================================================
    Statement,
    LabeledStatement,
    UnlabeledStatement,
    Label,

    ExpressionStatement,
    PrimaryBlock,
    CompoundStatement,
    BlockItem,
    SelectionStatement,
    IfStatement,
    SwitchStatement,
    IterationStatement,
    WhileStatement,
    DoWhileStatement,
    ForStatement,
    ForInit,
    JumpStatement,
    GotoStatement,
    ContinueStatement,
    BreakStatement,
    ReturnStatement,

    /// Error iteration statement
    IterError,
    StmtError,

    // =========================================================================
    // Top-level (6.9)
    // =========================================================================
    TranslationUnit,
    ExternalDeclaration,
    FunctionDefinition,

    // =========================================================================
    // Error sentinel
    // =========================================================================
    /// Marker for error nodes inserted during recovery.
    Error,
}

impl SyntaxKind {
    /// Returns `true` if this kind represents a token (leaf).
    pub fn is_token(self) -> bool {
        matches!(
            self,
            SyntaxKind::Whitespace
                | SyntaxKind::Comment
                | SyntaxKind::Ident
                | SyntaxKind::IntegerConst
                | SyntaxKind::FloatConst
                | SyntaxKind::CharConst
                | SyntaxKind::PredefinedConst
                | SyntaxKind::StringLiteral
                | SyntaxKind::QuotedString
                | SyntaxKind::Unknown
                | SyntaxKind::LeftParen
                | SyntaxKind::RightParen
                | SyntaxKind::LeftBracket
                | SyntaxKind::RightBracket
                | SyntaxKind::LeftBrace
                | SyntaxKind::RightBrace
                | SyntaxKind::Dot
                | SyntaxKind::Arrow
                | SyntaxKind::Increment
                | SyntaxKind::Decrement
                | SyntaxKind::Ampersand
                | SyntaxKind::Star
                | SyntaxKind::Plus
                | SyntaxKind::Minus
                | SyntaxKind::Tilde
                | SyntaxKind::Bang
                | SyntaxKind::Slash
                | SyntaxKind::Percent
                | SyntaxKind::LeftShift
                | SyntaxKind::RightShift
                | SyntaxKind::Less
                | SyntaxKind::Greater
                | SyntaxKind::LessEqual
                | SyntaxKind::GreaterEqual
                | SyntaxKind::Equal
                | SyntaxKind::NotEqual
                | SyntaxKind::Caret
                | SyntaxKind::Pipe
                | SyntaxKind::LogicalAnd
                | SyntaxKind::LogicalOr
                | SyntaxKind::Question
                | SyntaxKind::Colon
                | SyntaxKind::Scope
                | SyntaxKind::Semicolon
                | SyntaxKind::Ellipsis
                | SyntaxKind::Assign
                | SyntaxKind::MulAssign
                | SyntaxKind::DivAssign
                | SyntaxKind::ModAssign
                | SyntaxKind::AddAssign
                | SyntaxKind::SubAssign
                | SyntaxKind::LeftShiftAssign
                | SyntaxKind::RightShiftAssign
                | SyntaxKind::AndAssign
                | SyntaxKind::XorAssign
                | SyntaxKind::OrAssign
                | SyntaxKind::Comma
                | SyntaxKind::Hash
                | SyntaxKind::HashHash
        )
    }

    /// Returns `true` if this kind represents an error.
    pub fn is_error(self) -> bool {
        matches!(
            self,
            SyntaxKind::Error
                | SyntaxKind::ExprError
                | SyntaxKind::DeclError
                | SyntaxKind::StmtError
                | SyntaxKind::IterError
        )
    }

    /// Returns a human-readable name for this syntax kind.
    pub fn name(self) -> &'static str {
        match self {
            SyntaxKind::Whitespace => "whitespace",
            SyntaxKind::Comment => "comment",
            SyntaxKind::Ident => "identifier",
            SyntaxKind::IntegerConst => "integer constant",
            SyntaxKind::FloatConst => "floating constant",
            SyntaxKind::CharConst => "character constant",
            SyntaxKind::PredefinedConst => "predefined constant",
            SyntaxKind::StringLiteral => "string literal",
            SyntaxKind::QuotedString => "quoted string",
            SyntaxKind::Unknown => "unknown token",
            SyntaxKind::LeftParen => "(",
            SyntaxKind::RightParen => ")",
            SyntaxKind::LeftBracket => "[",
            SyntaxKind::RightBracket => "]",
            SyntaxKind::LeftBrace => "{",
            SyntaxKind::RightBrace => "}",
            SyntaxKind::Dot => ".",
            SyntaxKind::Arrow => "->",
            SyntaxKind::Increment => "++",
            SyntaxKind::Decrement => "--",
            SyntaxKind::Ampersand => "&",
            SyntaxKind::Star => "*",
            SyntaxKind::Plus => "+",
            SyntaxKind::Minus => "-",
            SyntaxKind::Tilde => "~",
            SyntaxKind::Bang => "!",
            SyntaxKind::Slash => "/",
            SyntaxKind::Percent => "%",
            SyntaxKind::LeftShift => "<<",
            SyntaxKind::RightShift => ">>",
            SyntaxKind::Less => "<",
            SyntaxKind::Greater => ">",
            SyntaxKind::LessEqual => "<=",
            SyntaxKind::GreaterEqual => ">=",
            SyntaxKind::Equal => "==",
            SyntaxKind::NotEqual => "!=",
            SyntaxKind::Caret => "^",
            SyntaxKind::Pipe => "|",
            SyntaxKind::LogicalAnd => "&&",
            SyntaxKind::LogicalOr => "||",
            SyntaxKind::Question => "?",
            SyntaxKind::Colon => ":",
            SyntaxKind::Scope => "::",
            SyntaxKind::Semicolon => ";",
            SyntaxKind::Ellipsis => "...",
            SyntaxKind::Assign => "=",
            SyntaxKind::MulAssign => "*=",
            SyntaxKind::DivAssign => "/=",
            SyntaxKind::ModAssign => "%=",
            SyntaxKind::AddAssign => "+=",
            SyntaxKind::SubAssign => "-=",
            SyntaxKind::LeftShiftAssign => "<<=",
            SyntaxKind::RightShiftAssign => ">>=",
            SyntaxKind::AndAssign => "&=",
            SyntaxKind::XorAssign => "^=",
            SyntaxKind::OrAssign => "|=",
            SyntaxKind::Comma => ",",
            SyntaxKind::Hash => "#",
            SyntaxKind::HashHash => "##",
            SyntaxKind::PrimaryExpr => "primary expression",
            SyntaxKind::GenericSelection => "generic selection",
            SyntaxKind::GenericAssociationType => "generic association (type)",
            SyntaxKind::GenericAssociationDefault => "generic association (default)",
            SyntaxKind::PostfixExpr => "postfix expression",
            SyntaxKind::ArrayAccess => "array access",
            SyntaxKind::FunctionCall => "function call",
            SyntaxKind::MemberAccess => "member access",
            SyntaxKind::MemberAccessPtr => "member access (ptr)",
            SyntaxKind::PostIncrement => "post-increment",
            SyntaxKind::PostDecrement => "post-decrement",
            SyntaxKind::CompoundLiteral => "compound literal",
            SyntaxKind::UnaryExpr => "unary expression",
            SyntaxKind::PreIncrement => "pre-increment",
            SyntaxKind::PreDecrement => "pre-decrement",
            SyntaxKind::UnaryOp => "unary operation",
            SyntaxKind::SizeofExpr => "sizeof expression",
            SyntaxKind::SizeofType => "sizeof type",
            SyntaxKind::Alignof => "alignof",
            SyntaxKind::CastExpr => "cast expression",
            SyntaxKind::Cast => "cast",
            SyntaxKind::BinaryExpr => "binary expression",
            SyntaxKind::ConditionalExpr => "conditional expression",
            SyntaxKind::AssignmentExpr => "assignment expression",
            SyntaxKind::CommaExpr => "comma expression",
            SyntaxKind::Expr => "expression",
            SyntaxKind::ExprError => "error expression",
            SyntaxKind::ConstantExpr => "constant expression",
            SyntaxKind::Declaration => "declaration",
            SyntaxKind::NormalDecl => "declaration",
            SyntaxKind::TypedefDecl => "typedef",
            SyntaxKind::StaticAssertDecl => "static assert",
            SyntaxKind::AttributeDecl => "attribute declaration",
            SyntaxKind::DeclError => "error declaration",
            SyntaxKind::DeclarationSpecifiers => "declaration specifiers",
            SyntaxKind::DeclarationSpecifier => "declaration specifier",
            SyntaxKind::InitDeclarator => "init declarator",
            SyntaxKind::TypeSpecifier => "type specifier",
            SyntaxKind::StructSpecifier => "struct/union specifier",
            SyntaxKind::EnumSpecifier => "enum specifier",
            SyntaxKind::AtomicTypeSpecifier => "atomic type specifier",
            SyntaxKind::TypeofSpecifier => "typeof specifier",
            SyntaxKind::TypeofSpecifierArg => "typeof argument",
            SyntaxKind::TypeQualifier => "type qualifier",
            SyntaxKind::StorageClassSpecifier => "storage class",
            SyntaxKind::FunctionSpecifier => "function specifier",
            SyntaxKind::AlignmentSpecifier => "alignment specifier",
            SyntaxKind::TypeSpecifierQualifier => "type specifier qualifier",
            SyntaxKind::MemberDeclaration => "member declaration",
            SyntaxKind::SpecifierQualifierList => "specifier qualifier list",
            SyntaxKind::MemberDeclarator => "member declarator",
            SyntaxKind::MemberBitField => "bit field",
            SyntaxKind::StructOrUnion => "struct or union",
            SyntaxKind::Enumerator => "enumerator",
            SyntaxKind::Declarator => "declarator",
            SyntaxKind::DirectDeclarator => "direct declarator",
            SyntaxKind::DirectDeclaratorIdent => "declarator identifier",
            SyntaxKind::DirectDeclaratorParen => "parenthesized declarator",
            SyntaxKind::DirectDeclaratorArray => "array declarator",
            SyntaxKind::DirectDeclaratorFunc => "function declarator",
            SyntaxKind::Pointer => "pointer",
            SyntaxKind::PointerOrBlock => "pointer or block",
            SyntaxKind::ArrayDeclarator => "array declarator",
            SyntaxKind::ArrayDeclaratorNormal => "array declarator",
            SyntaxKind::ArrayDeclaratorStatic => "static array declarator",
            SyntaxKind::ArrayDeclaratorVLA => "VLA declarator",
            SyntaxKind::ParameterTypeList => "parameter type list",
            SyntaxKind::ParameterDeclaration => "parameter declaration",
            SyntaxKind::ParameterDeclarationKind => "parameter kind",
            SyntaxKind::TypeName => "type name",
            SyntaxKind::AbstractDeclarator => "abstract declarator",
            SyntaxKind::DirectAbstractDeclarator => "direct abstract declarator",
            SyntaxKind::DirectAbstractDeclaratorParen => "parenthesized abstract declarator",
            SyntaxKind::DirectAbstractDeclaratorArray => "array abstract declarator",
            SyntaxKind::DirectAbstractDeclaratorFunc => "function abstract declarator",
            SyntaxKind::Initializer => "initializer",
            SyntaxKind::BracedInitializer => "braced initializer",
            SyntaxKind::DesignatedInitializer => "designated initializer",
            SyntaxKind::Designation => "designation",
            SyntaxKind::Designator => "designator",
            SyntaxKind::AttributeSpecifier => "attribute specifier",
            SyntaxKind::Attribute => "attribute",
            SyntaxKind::AttributeToken => "attribute token",
            SyntaxKind::AsmAttribute => "asm attribute",
            SyntaxKind::Statement => "statement",
            SyntaxKind::LabeledStatement => "labeled statement",
            SyntaxKind::UnlabeledStatement => "unlabeled statement",
            SyntaxKind::Label => "label",
            SyntaxKind::ExpressionStatement => "expression statement",
            SyntaxKind::PrimaryBlock => "primary block",
            SyntaxKind::CompoundStatement => "compound statement",
            SyntaxKind::BlockItem => "block item",
            SyntaxKind::SelectionStatement => "selection statement",
            SyntaxKind::IfStatement => "if statement",
            SyntaxKind::SwitchStatement => "switch statement",
            SyntaxKind::IterationStatement => "iteration statement",
            SyntaxKind::WhileStatement => "while statement",
            SyntaxKind::DoWhileStatement => "do-while statement",
            SyntaxKind::ForStatement => "for statement",
            SyntaxKind::ForInit => "for initializer",
            SyntaxKind::JumpStatement => "jump statement",
            SyntaxKind::GotoStatement => "goto",
            SyntaxKind::ContinueStatement => "continue",
            SyntaxKind::BreakStatement => "break",
            SyntaxKind::ReturnStatement => "return",
            SyntaxKind::IterError => "error iteration",
            SyntaxKind::StmtError => "error statement",
            SyntaxKind::TranslationUnit => "translation unit",
            SyntaxKind::ExternalDeclaration => "external declaration",
            SyntaxKind::FunctionDefinition => "function definition",
            SyntaxKind::Error => "error",
        }
    }
}
