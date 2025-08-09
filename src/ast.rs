//! https://www.open-std.org/jtc1/sc22/wg14/www/docs/n3096.pdf

use dbg_pls::DebugPls;
use std::fmt;

// =============================================================================
// Basic Types
// =============================================================================

/// Identifier
#[derive(Debug, DebugPls, Default, Clone, PartialEq, Eq, Hash)]
pub struct Identifier(pub String);

impl fmt::Display for Identifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<&str> for Identifier {
    fn from(s: &str) -> Self {
        Identifier(s.to_string())
    }
}

// =============================================================================
// Lexical Elements (6.4)
// =============================================================================

/// Constants (6.4.4)
#[derive(Debug, DebugPls, Clone, PartialEq)]
pub enum Constant {
    Integer(IntegerConstant),
    Floating(FloatingConstant),
    Character(CharacterConstant),
    Predefined(PredefinedConstant),
}

/// Integer constants (6.4.4.1)
#[derive(Debug, DebugPls, Default, Clone, PartialEq)]
pub struct IntegerConstant {
    pub value: i128,
    pub suffix: Option<IntegerSuffix>,
}

#[derive(Debug, DebugPls, Clone, PartialEq)]
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
#[derive(Debug, DebugPls, Default, Clone, PartialEq)]
pub struct FloatingConstant {
    pub value: f64,
    pub suffix: Option<FloatingSuffix>,
}

#[derive(Debug, DebugPls, Clone, PartialEq)]
pub enum FloatingSuffix {
    F,
    L,
    DF,
    DD,
    DL,
}

/// Character constants (6.4.4.4)
#[derive(Debug, DebugPls, Default, Clone, PartialEq)]
pub struct CharacterConstant {
    pub encoding_prefix: Option<EncodingPrefix>,
    pub value: String,
}

#[derive(Debug, DebugPls, Clone, PartialEq)]
pub enum EncodingPrefix {
    U8,
    U,
    CapitalU,
    L,
}

/// Predefined constants (6.4.4.5)
#[derive(Debug, DebugPls, Clone, PartialEq)]
pub enum PredefinedConstant {
    False,
    True,
    Nullptr,
}

/// String literals (6.4.5)
#[derive(Debug, DebugPls, Default, Clone, PartialEq)]
pub struct StringLiterals(pub Vec<StringLiteral>);

#[derive(Debug, DebugPls, Default, Clone, PartialEq)]
pub struct StringLiteral {
    pub encoding_prefix: Option<EncodingPrefix>,
    pub value: String,
}

/// Punctuators (6.4.6)
#[derive(Debug, DebugPls, Clone, PartialEq)]
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

    // Digraphs
    LeftBracketAlt,
    RightBracketAlt,
    LeftBraceAlt,
    RightBraceAlt,
    HashAlt,
    HashHashAlt,
}

// =============================================================================
// Expressions (6.5)
// =============================================================================

/// Expression
#[derive(Debug, DebugPls, Clone, PartialEq)]
pub enum Expression {
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
#[derive(Debug, DebugPls, Clone, PartialEq)]
pub enum PrimaryExpression {
    Identifier(Identifier),
    Constant(Constant),
    EnumerationConstant(Identifier),
    StringLiteral(StringLiterals),
    Parenthesized(Box<Expression>),
    Generic(GenericSelection),
    Error,
}

/// Generic selection (6.5.1.1)
#[derive(Debug, DebugPls, Clone, PartialEq)]
pub struct GenericSelection {
    pub controlling_expression: Box<Expression>,
    pub associations: Vec<GenericAssociation>,
}

#[derive(Debug, DebugPls, Clone, PartialEq)]
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
#[derive(Debug, DebugPls, Clone, PartialEq)]
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
#[derive(Debug, DebugPls, Default, Clone, PartialEq)]
pub struct CompoundLiteral {
    pub storage_class_specifiers: Vec<StorageClassSpecifier>,
    pub type_name: TypeName,
    pub initializer: BracedInitializer,
}

/// Unary expressions (6.5.3)
#[derive(Debug, DebugPls, Clone, PartialEq)]
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

#[derive(Debug, DebugPls, Clone, PartialEq)]
pub enum UnaryOperator {
    Address,
    Dereference,
    Plus,
    Minus,
    BitwiseNot,
    LogicalNot,
}

/// Cast expressions (6.5.4)
#[derive(Debug, DebugPls, Clone, PartialEq)]
pub enum CastExpression {
    Unary(UnaryExpression),
    Cast {
        type_name: TypeName,
        expression: Box<CastExpression>,
    },
}

/// Binary expressions
#[derive(Debug, DebugPls, Clone, PartialEq)]
pub struct BinaryExpression {
    pub left: Box<Expression>,
    pub operator: BinaryOperator,
    pub right: Box<Expression>,
}

#[derive(Debug, DebugPls, Clone, Copy, PartialEq)]
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
#[derive(Debug, DebugPls, Clone, PartialEq)]
pub struct ConditionalExpression {
    pub condition: Box<Expression>,
    pub then_expr: Box<Expression>,
    pub else_expr: Box<Expression>,
}

/// Assignment expressions (6.5.16)
#[derive(Debug, DebugPls, Clone, PartialEq)]
pub struct AssignmentExpression {
    pub left: Box<Expression>,
    pub operator: AssignmentOperator,
    pub right: Box<Expression>,
}

#[derive(Debug, DebugPls, Clone, Copy, PartialEq)]
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
#[derive(Debug, DebugPls, Default, Clone, PartialEq)]
pub struct CommaExpression {
    pub expressions: Vec<Expression>,
}

/// Constant expressions (6.6)
#[derive(Debug, DebugPls, Clone, PartialEq)]
pub struct ConstantExpression(pub Box<Expression>);

// =============================================================================
// Declarations (6.7)
// =============================================================================

/// Declarations (6.7)
#[derive(Debug, DebugPls, Clone, PartialEq)]
pub enum Declaration {
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
}

/// Declaration specifiers (6.7)
#[derive(Debug, DebugPls, Default, Clone, PartialEq)]
pub struct DeclarationSpecifiers {
    pub specifiers: Vec<DeclarationSpecifier>,
}

#[derive(Debug, DebugPls, Clone, PartialEq)]
pub enum DeclarationSpecifier {
    StorageClass(StorageClassSpecifier),
    TypeSpecifierQualifier(TypeSpecifierQualifier),
    Function {
        specifier: FunctionSpecifier,
        attributes: Vec<AttributeSpecifier>,
    },
}

/// Init declarators (6.7)
#[derive(Debug, DebugPls, Clone, PartialEq)]
pub struct InitDeclarator {
    pub declarator: Declarator,
    pub initializer: Option<Initializer>,
}

/// Storage class specifiers (6.7.1)
#[derive(Debug, DebugPls, Clone, PartialEq)]
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
#[derive(Debug, DebugPls, Clone, PartialEq)]
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
#[derive(Debug, DebugPls, Clone, PartialEq)]
pub struct StructOrUnionSpecifier {
    pub kind: StructOrUnion,
    pub attributes: Vec<AttributeSpecifier>,
    pub identifier: Option<Identifier>,
    pub members: Option<Vec<MemberDeclaration>>,
}

#[derive(Debug, DebugPls, Clone, PartialEq)]
pub enum StructOrUnion {
    Struct,
    Union,
}

/// Member declarations (6.7.2.1)
#[derive(Debug, DebugPls, Clone, PartialEq)]
pub enum MemberDeclaration {
    Normal {
        attributes: Vec<AttributeSpecifier>,
        specifiers: SpecifierQualifierList,
        declarators: Vec<MemberDeclarator>,
    },
    StaticAssert(StaticAssertDeclaration),
}

/// Specifier qualifier lists (6.7.2.1)
#[derive(Debug, DebugPls, Default, Clone, PartialEq)]
pub struct SpecifierQualifierList {
    pub items: Vec<TypeSpecifierQualifier>,
    pub attributes: Vec<AttributeSpecifier>,
}

#[derive(Debug, DebugPls, Clone, PartialEq)]
pub enum TypeSpecifierQualifier {
    TypeSpecifier(TypeSpecifier),
    TypeQualifier(TypeQualifier),
    AlignmentSpecifier(AlignmentSpecifier),
}

/// Member declarators (6.7.2.1)
#[derive(Debug, DebugPls, Clone, PartialEq)]
pub enum MemberDeclarator {
    Declarator(Declarator),
    BitField {
        declarator: Option<Declarator>,
        width: ConstantExpression,
    },
}

/// Enum specifiers (6.7.2.2)
#[derive(Debug, DebugPls, Default, Clone, PartialEq)]
pub struct EnumSpecifier {
    pub attributes: Vec<AttributeSpecifier>,
    pub identifier: Option<Identifier>,
    pub type_specifier: Option<SpecifierQualifierList>,
    pub enumerators: Option<Vec<Enumerator>>,
}

#[derive(Debug, DebugPls, Default, Clone, PartialEq)]
pub struct Enumerator {
    pub name: Identifier,
    pub attributes: Vec<AttributeSpecifier>,
    pub value: Option<ConstantExpression>,
}

/// Atomic type specifiers (6.7.2.4)
#[derive(Debug, DebugPls, Default, Clone, PartialEq)]
pub struct AtomicTypeSpecifier {
    pub type_name: TypeName,
}

/// typeof specifiers (6.7.2.5)
#[derive(Debug, DebugPls, Clone, PartialEq)]
pub enum TypeofSpecifier {
    Typeof(TypeofSpecifierArgument),
    TypeofUnqual(TypeofSpecifierArgument),
}

#[derive(Debug, DebugPls, Clone, PartialEq)]
pub enum TypeofSpecifierArgument {
    Expression(Box<Expression>),
    TypeName(TypeName),
}

/// Type qualifiers (6.7.3)
#[derive(Debug, DebugPls, Clone, PartialEq)]
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
#[derive(Debug, DebugPls, Clone, PartialEq)]
pub enum FunctionSpecifier {
    Inline,
    Noreturn,
}

/// Alignment specifiers (6.7.5)
#[derive(Debug, DebugPls, Clone, PartialEq)]
pub enum AlignmentSpecifier {
    Type(TypeName),
    Expression(ConstantExpression),
}

/// Declarators (6.7.6)
#[derive(Debug, DebugPls, Clone, PartialEq)]
pub enum Declarator {
    Direct(DirectDeclarator),
    Pointer {
        pointer: Pointer,
        declarator: Box<Declarator>,
    },
}

impl Declarator {
    pub fn identifier(&self) -> &Identifier {
        match self {
            Declarator::Direct(direct) => direct.identifier(),
            Declarator::Pointer { declarator, .. } => declarator.identifier(),
        }
    }
}

#[derive(Debug, DebugPls, Clone, PartialEq)]
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
    pub fn identifier(&self) -> &Identifier {
        match self {
            DirectDeclarator::Identifier { identifier, .. } => identifier,
            DirectDeclarator::Parenthesized(declarator) => declarator.identifier(),
            DirectDeclarator::Array { declarator, .. } => declarator.identifier(),
            DirectDeclarator::Function { declarator, .. } => declarator.identifier(),
        }
    }
}

/// Array declarators (6.7.6)
#[derive(Debug, DebugPls, Clone, PartialEq)]
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
}

/// Pointers (6.7.6)
#[derive(Debug, DebugPls, Clone, PartialEq)]
pub struct Pointer {
    pub pointer_or_block: PointerOrBlock,
    pub attributes: Vec<AttributeSpecifier>,
    pub type_qualifiers: Vec<TypeQualifier>,
}

#[derive(Debug, DebugPls, Clone, Copy, PartialEq)]
pub enum PointerOrBlock {
    Pointer,
    Block,
}

/// Parameter type lists (6.7.6)
#[derive(Debug, DebugPls, Clone, PartialEq)]
pub enum ParameterTypeList {
    Parameters(Vec<ParameterDeclaration>),
    Variadic(Vec<ParameterDeclaration>),
    OnlyVariadic,
}

/// Parameter declarations (6.7.6)
#[derive(Debug, DebugPls, Default, Clone, PartialEq)]
pub struct ParameterDeclaration {
    pub attributes: Vec<AttributeSpecifier>,
    pub specifiers: DeclarationSpecifiers,
    pub declarator: Option<ParameterDeclarationKind>,
}

#[derive(Debug, DebugPls, Clone, PartialEq)]
pub enum ParameterDeclarationKind {
    Declarator(Declarator),
    Abstract(AbstractDeclarator),
}

/// Type names (6.7.7)
#[derive(Debug, DebugPls, Default, Clone, PartialEq)]
pub struct TypeName {
    pub specifiers: SpecifierQualifierList,
    pub abstract_declarator: Option<AbstractDeclarator>,
}

/// Abstract declarators (6.7.7)
#[derive(Debug, DebugPls, Clone, PartialEq)]
pub enum AbstractDeclarator {
    Direct(DirectAbstractDeclarator),
    Pointer {
        pointer: Pointer,
        abstract_declarator: Option<Box<AbstractDeclarator>>,
    },
}

#[derive(Debug, DebugPls, Clone, PartialEq)]
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
#[derive(Debug, DebugPls, Clone, PartialEq)]
pub enum Initializer {
    Expression(Box<Expression>),
    Braced(BracedInitializer),
}

#[derive(Debug, DebugPls, Default, Clone, PartialEq)]
pub struct BracedInitializer {
    pub initializers: Vec<DesignatedInitializer>,
}

#[derive(Debug, DebugPls, Clone, PartialEq)]
pub struct DesignatedInitializer {
    pub designation: Option<Designation>,
    pub initializer: Initializer,
}

#[derive(Debug, DebugPls, Clone, PartialEq)]
pub struct Designation {
    pub designator: Designator,
    pub designation: Option<Box<Designation>>,
}

#[derive(Debug, DebugPls, Clone, PartialEq)]
pub enum Designator {
    Array(Box<Expression>),
    Member(Identifier),
}

/// Static assert declarations (6.7.11)
#[derive(Debug, DebugPls, Clone, PartialEq)]
pub struct StaticAssertDeclaration {
    pub condition: ConstantExpression,
    pub message: Option<StringLiterals>,
}

/// Attribute specifiers (6.7.12.1)
#[derive(Debug, DebugPls, Clone, PartialEq)]
pub enum AttributeSpecifier {
    Attributes(Vec<Attribute>),
    Asm(StringLiterals),
}

#[derive(Debug, DebugPls, Clone, PartialEq)]
pub struct Attribute {
    pub token: AttributeToken,
    pub arguments: BalancedTokenSequence,
}

#[derive(Debug, DebugPls, Clone, PartialEq)]
pub enum AttributeToken {
    Standard(Identifier),
    Prefixed { prefix: Identifier, identifier: Identifier },
}

#[derive(Debug, DebugPls, Default, Clone, PartialEq)]
pub struct BalancedTokenSequence(pub Vec<BalancedToken>);

impl AsRef<[BalancedToken]> for BalancedTokenSequence {
    fn as_ref(&self) -> &[BalancedToken] {
        &self.0
    }
}

#[derive(Debug, DebugPls, Clone, PartialEq)]
pub enum BalancedToken {
    Parenthesized(BalancedTokenSequence),
    Bracketed(BalancedTokenSequence),
    Braced(BalancedTokenSequence),
    Identifier(Identifier),
    StringLiteral(StringLiterals),
    Constant(Constant),
    Punctuator(Punctuator),
    Unknown, // For any other tokens not explicitly defined
}

// =============================================================================
// Statements (6.8)
// =============================================================================

/// Statements (6.8)
#[derive(Debug, DebugPls, Clone, PartialEq)]
pub enum Statement {
    Labeled(LabeledStatement),
    Unlabeled(UnlabeledStatement),
}

#[derive(Debug, DebugPls, Clone, PartialEq)]
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

#[derive(Debug, DebugPls, Clone, PartialEq)]
pub enum PrimaryBlock {
    Compound(CompoundStatement),
    Selection(SelectionStatement),
    Iteration(IterationStatement),
}

/// Labels (6.8.1)
#[derive(Debug, DebugPls, Clone, PartialEq)]
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

#[derive(Debug, DebugPls, Clone, PartialEq)]
pub struct LabeledStatement {
    pub label: Label,
    pub statement: Box<Statement>,
}

/// Compound statements (6.8.2)
#[derive(Debug, DebugPls, Default, Clone, PartialEq)]
pub struct CompoundStatement {
    pub items: Vec<BlockItem>,
}

#[derive(Debug, DebugPls, Clone, PartialEq)]
pub enum BlockItem {
    Declaration(Declaration),
    Statement(UnlabeledStatement),
    Label(Label),
}

/// Expression statements (6.8.3)
#[derive(Debug, DebugPls, Clone, PartialEq)]
pub struct ExpressionStatement {
    pub attributes: Vec<AttributeSpecifier>,
    pub expression: Option<Box<Expression>>,
}

/// Selection statements (6.8.4)
#[derive(Debug, DebugPls, Clone, PartialEq)]
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
#[derive(Debug, DebugPls, Clone, PartialEq)]
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
}

#[derive(Debug, DebugPls, Clone, PartialEq)]
pub enum ForInit {
    Expression(Box<Expression>),
    Declaration(Declaration),
}

/// Jump statements (6.8.6)
#[derive(Debug, DebugPls, Clone, PartialEq)]
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
#[derive(Debug, DebugPls, Default, Clone, PartialEq)]
pub struct TranslationUnit {
    pub external_declarations: Vec<ExternalDeclaration>,
}

#[derive(Debug, DebugPls, Clone, PartialEq)]
pub enum ExternalDeclaration {
    Function(FunctionDefinition),
    Declaration(Declaration),
}

/// Function definitions (6.9.1)
#[derive(Debug, DebugPls, Clone, PartialEq)]
pub struct FunctionDefinition {
    pub attributes: Vec<AttributeSpecifier>,
    pub specifiers: DeclarationSpecifiers,
    pub declarator: Declarator,
    pub body: CompoundStatement,
}
