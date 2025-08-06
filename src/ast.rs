use std::fmt;

/// Source code position information
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

impl Span {
    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }
}

/// AST node with position information
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Spanned<T> {
    pub node: T,
    pub span: Span,
}

impl<T> Spanned<T> {
    pub fn new(node: T, span: Span) -> Self {
        Self { node, span }
    }
}

// =============================================================================
// Basic Types
// =============================================================================

/// Identifier
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct Identifier(pub String);

impl fmt::Display for Identifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

// =============================================================================
// Lexical Elements (6.4)
// =============================================================================

/// Keywords (6.4.1)
#[derive(Debug, Clone, PartialEq)]
pub enum Keyword {
    // Storage class
    Auto,
    Constexpr,
    Extern,
    Register,
    Static,
    ThreadLocal,
    Typedef,

    // Type specifiers
    Void,
    Char,
    Short,
    Int,
    Long,
    Float,
    Double,
    Signed,
    Unsigned,
    Bool,
    Complex,
    BitInt,
    Decimal32,
    Decimal64,
    Decimal128,

    // Type qualifiers
    Const,
    Restrict,
    Volatile,
    Atomic,

    // Function specifiers
    Inline,
    Noreturn,

    // Alignment
    Alignas,
    Alignof,

    // Control flow
    If,
    Else,
    Switch,
    Case,
    Default,
    While,
    Do,
    For,
    Goto,
    Continue,
    Break,
    Return,

    // Other
    Sizeof,
    Typeof,
    TypeofUnqual,
    Struct,
    Union,
    Enum,
    StaticAssert,
    Generic,

    // Literals
    True,
    False,
    Nullptr,
}

/// Constants (6.4.4)
#[derive(Debug, Clone, PartialEq)]
pub enum Constant {
    Integer(IntegerConstant),
    Floating(FloatingConstant),
    Enumeration(Identifier),
    Character(CharacterConstant),
    Predefined(PredefinedConstant),
}

/// Integer constants (6.4.4.1)
#[derive(Debug, Default, Clone, PartialEq)]
pub struct IntegerConstant {
    pub value: i128,
    pub suffix: Option<IntegerSuffix>,
}

#[derive(Debug, Clone, PartialEq)]
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
#[derive(Debug, Default, Clone, PartialEq)]
pub struct FloatingConstant {
    pub value: f64,
    pub suffix: Option<FloatingSuffix>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum FloatingSuffix {
    F,
    L,
    DF,
    DD,
    DL,
}

/// Character constants (6.4.4.4)
#[derive(Debug, Default, Clone, PartialEq)]
pub struct CharacterConstant {
    pub encoding_prefix: Option<EncodingPrefix>,
    pub value: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum EncodingPrefix {
    U8,
    U,
    CapitalU,
    L,
}

/// Predefined constants (6.4.4.5)
#[derive(Debug, Clone, PartialEq)]
pub enum PredefinedConstant {
    False,
    True,
    Nullptr,
}

/// String literals (6.4.5)
#[derive(Debug, Default, Clone, PartialEq)]
pub struct StringLiteral {
    pub encoding_prefix: Option<EncodingPrefix>,
    pub value: String,
}

/// Punctuators (6.4.6)
#[derive(Debug, Clone, PartialEq)]
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
#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Primary(PrimaryExpression),
    Postfix(PostfixExpression),
    Unary(UnaryExpression),
    Cast(CastExpression),
    Binary(BinaryExpression),
    Conditional(ConditionalExpression),
    Assignment(AssignmentExpression),
    Comma(CommaExpression),
}

/// Primary expressions (6.5.1)
#[derive(Debug, Clone, PartialEq)]
pub enum PrimaryExpression {
    Identifier(Identifier),
    Constant(Constant),
    StringLiteral(StringLiteral),
    Parenthesized(Box<Expression>),
    Generic(GenericSelection),
}

/// Generic selection (6.5.1.1)
#[derive(Debug, Clone, PartialEq)]
pub struct GenericSelection {
    pub controlling_expression: Box<Expression>,
    pub associations: Vec<GenericAssociation>,
}

#[derive(Debug, Clone, PartialEq)]
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
#[derive(Debug, Clone, PartialEq)]
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
#[derive(Debug, Default, Clone, PartialEq)]
pub struct CompoundLiteral {
    pub storage_class_specifiers: Vec<StorageClassSpecifier>,
    pub type_name: TypeName,
    pub initializer: BracedInitializer,
}

/// Unary expressions (6.5.3)
#[derive(Debug, Clone, PartialEq)]
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

#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOperator {
    Address,
    Dereference,
    Plus,
    Minus,
    BitwiseNot,
    LogicalNot,
}

/// Cast expressions (6.5.4)
#[derive(Debug, Clone, PartialEq)]
pub enum CastExpression {
    Unary(UnaryExpression),
    Cast {
        type_name: TypeName,
        expression: Box<CastExpression>,
    },
}

/// Binary expressions
#[derive(Debug, Clone, PartialEq)]
pub struct BinaryExpression {
    pub left: Box<Expression>,
    pub operator: BinaryOperator,
    pub right: Box<Expression>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
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
#[derive(Debug, Clone, PartialEq)]
pub struct ConditionalExpression {
    pub condition: Box<Expression>,
    pub then_expr: Box<Expression>,
    pub else_expr: Box<Expression>,
}

/// Assignment expressions (6.5.16)
#[derive(Debug, Clone, PartialEq)]
pub struct AssignmentExpression {
    pub left: Box<Expression>,
    pub operator: AssignmentOperator,
    pub right: Box<Expression>,
}

#[derive(Debug, Clone, PartialEq)]
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
#[derive(Debug, Default, Clone, PartialEq)]
pub struct CommaExpression {
    pub expressions: Vec<Expression>,
}

// =============================================================================
// Declarations (6.7)
// =============================================================================

/// Declarations (6.7)
#[derive(Debug, Clone, PartialEq)]
pub enum Declaration {
    Normal {
        attributes: Vec<AttributeSpecifier>,
        specifiers: DeclarationSpecifiers,
        declarators: Vec<InitDeclarator>,
    },
    StaticAssert(StaticAssertDeclaration),
    Attribute(Vec<AttributeSpecifier>),
}

/// Declaration specifiers (6.7)
#[derive(Debug, Default, Clone, PartialEq)]
pub struct DeclarationSpecifiers {
    pub specifiers: Vec<DeclarationSpecifier>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum DeclarationSpecifier {
    StorageClass(StorageClassSpecifier),
    TypeSpecifierQualifier(TypeSpecifierQualifier),
    Function(FunctionSpecifier),
}

/// Init declarators (6.7)
#[derive(Debug, Clone, PartialEq)]
pub struct InitDeclarator {
    pub declarator: Declarator,
    pub initializer: Option<Initializer>,
}

/// Storage class specifiers (6.7.1)
#[derive(Debug, Clone, PartialEq)]
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
#[derive(Debug, Clone, PartialEq)]
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
    BitInt(Box<Expression>),
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
#[derive(Debug, Clone, PartialEq)]
pub struct StructOrUnionSpecifier {
    pub kind: StructOrUnion,
    pub attributes: Vec<AttributeSpecifier>,
    pub identifier: Option<Identifier>,
    pub members: Option<Vec<MemberDeclaration>>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum StructOrUnion {
    Struct,
    Union,
}

/// Member declarations (6.7.2.1)
#[derive(Debug, Clone, PartialEq)]
pub enum MemberDeclaration {
    Normal {
        attributes: Vec<AttributeSpecifier>,
        specifiers: SpecifierQualifierList,
        declarators: Vec<MemberDeclarator>,
    },
    StaticAssert(StaticAssertDeclaration),
}

/// Specifier qualifier lists (6.7.2.1)
#[derive(Debug, Default, Clone, PartialEq)]
pub struct SpecifierQualifierList {
    pub items: Vec<TypeSpecifierQualifier>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TypeSpecifierQualifier {
    TypeSpecifier(TypeSpecifier),
    TypeQualifier(TypeQualifier),
    AlignmentSpecifier(AlignmentSpecifier),
}

/// Member declarators (6.7.2.1)
#[derive(Debug, Clone, PartialEq)]
pub enum MemberDeclarator {
    Declarator(Declarator),
    BitField {
        declarator: Option<Declarator>,
        width: Box<Expression>,
    },
}

/// Enum specifiers (6.7.2.2)
#[derive(Debug, Default, Clone, PartialEq)]
pub struct EnumSpecifier {
    pub attributes: Vec<AttributeSpecifier>,
    pub identifier: Option<Identifier>,
    pub type_specifier: Option<SpecifierQualifierList>,
    pub enumerators: Option<Vec<Enumerator>>,
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Enumerator {
    pub name: Identifier,
    pub attributes: Vec<AttributeSpecifier>,
    pub value: Option<Box<Expression>>,
}

/// Atomic type specifiers (6.7.2.4)
#[derive(Debug, Default, Clone, PartialEq)]
pub struct AtomicTypeSpecifier {
    pub type_name: TypeName,
}

/// typeof specifiers (6.7.2.5)
#[derive(Debug, Clone, PartialEq)]
pub enum TypeofSpecifier {
    Typeof(TypeofSpecifierArgument),
    TypeofUnqual(TypeofSpecifierArgument),
}

#[derive(Debug, Clone, PartialEq)]
pub enum TypeofSpecifierArgument {
    Expression(Box<Expression>),
    TypeName(TypeName),
}

/// Type qualifiers (6.7.3)
#[derive(Debug, Clone, PartialEq)]
pub enum TypeQualifier {
    Const,
    Restrict,
    Volatile,
    Atomic,
}

/// Function specifiers (6.7.4)
#[derive(Debug, Clone, PartialEq)]
pub enum FunctionSpecifier {
    Inline,
    Noreturn,
}

/// Alignment specifiers (6.7.5)
#[derive(Debug, Clone, PartialEq)]
pub enum AlignmentSpecifier {
    Type(TypeName),
    Expression(Box<Expression>),
}

/// Declarators (6.7.6)
#[derive(Debug, Clone, PartialEq)]
pub struct Declarator {
    pub pointer: Option<Pointer>,
    pub direct_declarator: DirectDeclarator,
}

#[derive(Debug, Clone, PartialEq)]
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
        parameters: Option<ParameterTypeList>,
    },
}

/// Array declarators (6.7.6)
#[derive(Debug, Clone, PartialEq)]
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
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Pointer {
    pub attributes: Vec<AttributeSpecifier>,
    pub type_qualifiers: Vec<TypeQualifier>,
    pub next: Option<Box<Pointer>>,
}

/// Parameter type lists (6.7.6)
#[derive(Debug, Clone, PartialEq)]
pub enum ParameterTypeList {
    Parameters(Vec<ParameterDeclaration>),
    Variadic(Vec<ParameterDeclaration>),
    OnlyVariadic,
}

/// Parameter declarations (6.7.6)
#[derive(Debug, Default, Clone, PartialEq)]
pub struct ParameterDeclaration {
    pub attributes: Vec<AttributeSpecifier>,
    pub specifiers: DeclarationSpecifiers,
    pub declarator: Option<Declarator>,
    pub abstract_declarator: Option<AbstractDeclarator>,
}

/// Type names (6.7.7)
#[derive(Debug, Default, Clone, PartialEq)]
pub struct TypeName {
    pub specifiers: SpecifierQualifierList,
    pub abstract_declarator: Option<AbstractDeclarator>,
}

/// Abstract declarators (6.7.7)
#[derive(Debug, Default, Clone, PartialEq)]
pub struct AbstractDeclarator {
    pub pointer: Option<Pointer>,
    pub direct_abstract_declarator: Option<DirectAbstractDeclarator>,
}

#[derive(Debug, Clone, PartialEq)]
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
        parameters: Option<ParameterTypeList>,
    },
}

/// Initializers (6.7.10)
#[derive(Debug, Clone, PartialEq)]
pub enum Initializer {
    Expression(Box<Expression>),
    Braced(BracedInitializer),
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct BracedInitializer {
    pub initializers: Vec<DesignatedInitializer>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DesignatedInitializer {
    pub designation: Option<Designation>,
    pub initializer: Initializer,
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Designation {
    pub designators: Vec<Designator>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Designator {
    Array(Box<Expression>),
    Member(Identifier),
}

/// Static assert declarations (6.7.11)
#[derive(Debug, Clone, PartialEq)]
pub struct StaticAssertDeclaration {
    pub condition: Box<Expression>,
    pub message: Option<StringLiteral>,
}

/// Attribute specifiers (6.7.12.1)
#[derive(Debug, Default, Clone, PartialEq)]
pub struct AttributeSpecifier {
    pub attributes: Vec<Option<Attribute>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Attribute {
    pub token: AttributeToken,
    pub arguments: Option<BalancedTokenSequence>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum AttributeToken {
    Standard(Identifier),
    Prefixed {
        prefix: Identifier,
        identifier: Identifier,
    },
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct BalancedTokenSequence {
    pub tokens: Vec<BalancedToken>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum BalancedToken {
    Parenthesized(BalancedTokenSequence),
    Bracketed(BalancedTokenSequence),
    Braced(BalancedTokenSequence),
    Identifier(Identifier),
    StringLiteral(StringLiteral),
    Constant(Constant),
    Punctuator(Punctuator),
    Unknown(String), // For any other tokens not explicitly defined
}

// =============================================================================
// Statements (6.8)
// =============================================================================

/// Statements (6.8)
#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    Labeled(LabeledStatement),
    Unlabeled(UnlabeledStatement),
}

#[derive(Debug, Clone, PartialEq)]
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

#[derive(Debug, Clone, PartialEq)]
pub enum PrimaryBlock {
    Compound(CompoundStatement),
    Selection(SelectionStatement),
    Iteration(IterationStatement),
}

/// Labels (6.8.1)
#[derive(Debug, Clone, PartialEq)]
pub enum Label {
    Identifier {
        attributes: Vec<AttributeSpecifier>,
        identifier: Identifier,
    },
    Case {
        attributes: Vec<AttributeSpecifier>,
        expression: Box<Expression>,
    },
    Default {
        attributes: Vec<AttributeSpecifier>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub struct LabeledStatement {
    pub label: Label,
    pub statement: Box<Statement>,
}

/// Compound statements (6.8.2)
#[derive(Debug, Default, Clone, PartialEq)]
pub struct CompoundStatement {
    pub items: Vec<BlockItem>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum BlockItem {
    Declaration(Declaration),
    Statement(UnlabeledStatement),
    Label(Label),
}

/// Expression statements (6.8.3)
#[derive(Debug, Clone, PartialEq)]
pub enum ExpressionStatement {
    Expression(Box<Expression>),
    AttributedExpression {
        attributes: Vec<AttributeSpecifier>,
        expression: Box<Expression>,
    },
    Empty,
}

/// Selection statements (6.8.4)
#[derive(Debug, Clone, PartialEq)]
pub enum SelectionStatement {
    If {
        condition: Box<Expression>,
        then_stmt: Box<Statement>,
    },
    IfElse {
        condition: Box<Expression>,
        then_stmt: Box<Statement>,
        else_stmt: Box<Statement>,
    },
    Switch {
        expression: Box<Expression>,
        statement: Box<Statement>,
    },
}

/// Iteration statements (6.8.5)
#[derive(Debug, Clone, PartialEq)]
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
        init: Option<Box<Expression>>,
        condition: Option<Box<Expression>>,
        update: Option<Box<Expression>>,
        body: Box<Statement>,
    },
    ForDecl {
        init: Declaration,
        condition: Option<Box<Expression>>,
        update: Option<Box<Expression>>,
        body: Box<Statement>,
    },
}

/// Jump statements (6.8.6)
#[derive(Debug, Clone, PartialEq)]
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
#[derive(Debug, Default, Clone, PartialEq)]
pub struct TranslationUnit {
    pub external_declarations: Vec<ExternalDeclaration>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ExternalDeclaration {
    Function(FunctionDefinition),
    Declaration(Declaration),
}

/// Function definitions (6.9.1)
#[derive(Debug, Clone, PartialEq)]
pub struct FunctionDefinition {
    pub attributes: Vec<AttributeSpecifier>,
    pub specifiers: DeclarationSpecifiers,
    pub declarator: Declarator,
    pub body: CompoundStatement,
}

// =============================================================================
// Preprocessing (6.10)
// =============================================================================

/// Preprocessing files (6.10)
#[derive(Debug, Default, Clone, PartialEq)]
pub struct PreprocessingFile {
    pub group: Option<Group>,
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Group {
    pub parts: Vec<GroupPart>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum GroupPart {
    IfSection(IfSection),
    ControlLine(ControlLine),
    TextLine(TextLine),
    NonDirective(NonDirective),
}

#[derive(Debug, Clone, PartialEq)]
pub struct IfSection {
    pub if_group: IfGroup,
    pub elif_groups: Vec<ElifGroup>,
    pub else_group: Option<ElseGroup>,
    pub endif_line: EndifLine,
}

#[derive(Debug, Clone, PartialEq)]
pub enum IfGroup {
    If {
        condition: Box<Expression>,
        group: Option<Group>,
    },
    Ifdef {
        identifier: Identifier,
        group: Option<Group>,
    },
    Ifndef {
        identifier: Identifier,
        group: Option<Group>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum ElifGroup {
    Elif {
        condition: Box<Expression>,
        group: Option<Group>,
    },
    Elifdef {
        identifier: Identifier,
        group: Option<Group>,
    },
    Elifndef {
        identifier: Identifier,
        group: Option<Group>,
    },
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct ElseGroup {
    pub group: Option<Group>,
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct EndifLine;

#[derive(Debug, Clone, PartialEq)]
pub enum ControlLine {
    Include(Vec<String>), // pp-tokens
    Embed(Vec<String>),   // pp-tokens
    Define {
        identifier: Identifier,
        parameters: Option<Vec<Identifier>>,
        variadic: bool,
        replacement: Vec<String>, // pp-tokens
    },
    Undef(Identifier),
    Line(Vec<String>),    // pp-tokens
    Error(Vec<String>),   // pp-tokens
    Warning(Vec<String>), // pp-tokens
    Pragma(Vec<String>),  // pp-tokens
    Empty,
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct TextLine {
    pub tokens: Vec<String>, // pp-tokens
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct NonDirective {
    pub tokens: Vec<String>, // pp-tokens
}

// =============================================================================
// Helper Types
// =============================================================================

/// Preprocessor expressions
#[derive(Debug, Clone, PartialEq)]
pub enum PreprocessorExpression {
    Defined(Identifier),
    DefinedParen(Identifier),
    HasInclude(HeaderName),
    HasIncludeTokens(HeaderNameTokens),
    HasEmbed {
        header: HeaderName,
        parameters: Vec<String>, // embed-parameter-sequence
    },
    HasEmbedTokens {
        header: HeaderNameTokens,
        parameters: Vec<String>, // pp-balanced-token-sequence
    },
    HasCAttribute(Vec<String>), // pp-tokens
    VaOpt(Vec<String>),         // pp-tokens
}

#[derive(Debug, Clone, PartialEq)]
pub enum HeaderName {
    SystemHeader(String), // < h-char-sequence >
    LocalHeader(String),  // " q-char-sequence "
}

#[derive(Debug, Clone, PartialEq)]
pub enum HeaderNameTokens {
    StringLiteral(StringLiteral),
    Tokens(Vec<String>), // < h-pp-tokens >
}
