//! Visitor pattern implementation for traversing the C AST.
//!
//! This module provides a flexible visitor pattern for traversing and analyzing
//! C abstract syntax trees. It distinguishes between different semantic types
//! of identifiers (variable names, type names, labels, etc.) to enable more
//! precise analysis and transformation of C code.
//!
//! # Example
//!
//! ```ignore
//! struct MyVisitor {
//!     variable_count: usize,
//! }
//!
//! impl<'a> Visitor<'a> for MyVisitor {
//!     type Result = ();
//!
//!     fn visit_variable_name(&mut self, id: &'a Identifier) {
//!         self.variable_count += 1;
//!         Self::Result::output()
//!     }
//! }
//! ```

use std::ops::ControlFlow;

use crate::{Identifier, ast::*};

/// A trait that represents the result type of visitor operations.
///
/// This trait allows visitor methods to return different result types:
/// - `()` for simple traversal without returning a value
/// - `ControlFlow<T>` for early termination with a value
/// - Custom types implementing `VisitorResult` for complex scenarios
///
/// The trait provides mechanisms to work with Rust's `?` operator equivalent
/// through the `branch()` and `from_branch()` methods.
pub trait VisitorResult {
    /// The type of value returned when breaking early from traversal.
    type Residual;

    /// Creates a successful output value.
    fn output() -> Self;

    /// Creates a result from a residual value (used when breaking early).
    fn from_residual(residual: Self::Residual) -> Self;

    /// Creates a result from a control flow, handling both continue and break
    /// cases.
    fn from_branch(b: ControlFlow<Self::Residual>) -> Self;

    /// Converts this result into a control flow for early termination handling.
    fn branch(self) -> ControlFlow<Self::Residual>;
}

/// Implementation of `VisitorResult` for unit type `()`.
///
/// This implementation is used when the visitor doesn't need to return values
/// or break early from traversal. It simply processes each node and continues.
impl VisitorResult for () {
    type Residual = core::convert::Infallible;

    fn output() -> Self {}

    fn from_residual(_: Self::Residual) -> Self {}

    fn from_branch(_: ControlFlow<Self::Residual>) -> Self {}

    fn branch(self) -> ControlFlow<Self::Residual> {
        ControlFlow::Continue(())
    }
}

impl<T> VisitorResult for Result<(), T> {
    type Residual = T;

    fn output() -> Self {
        Ok(())
    }

    fn from_residual(residual: Self::Residual) -> Self {
        Err(residual)
    }

    fn from_branch(b: ControlFlow<Self::Residual>) -> Self {
        match b {
            ControlFlow::Break(r) => Err(r),
            ControlFlow::Continue(()) => Ok(()),
        }
    }

    fn branch(self) -> ControlFlow<Self::Residual> {
        match self {
            Ok(()) => ControlFlow::Continue(()),
            Err(r) => ControlFlow::Break(r),
        }
    }
}

/// Implementation of `VisitorResult` for `ControlFlow<T>`.
///
/// This implementation allows visitors to return `ControlFlow::Break(value)`
/// to terminate traversal early and `ControlFlow::Continue(())` to continue.
impl<T> VisitorResult for ControlFlow<T> {
    type Residual = T;

    fn output() -> Self {
        ControlFlow::Continue(())
    }

    fn from_residual(residual: Self::Residual) -> Self {
        ControlFlow::Break(residual)
    }

    fn from_branch(b: Self) -> Self {
        b
    }

    fn branch(self) -> Self {
        self
    }
}

/// The main visitor trait for traversing C AST nodes.
///
/// Implementers of this trait can customize behavior for different AST node
/// types. The trait provides default implementations that perform recursive
/// traversal via corresponding `walk_*` functions.
///
/// # Result Type
///
/// The associated `Result` type determines how visitor methods report their
/// outcome. Common choices are:
/// - `()` for simple analysis without early termination
/// - `ControlFlow<T>` for analysis that may terminate early with a result
///
/// # Default Behavior
///
/// Each visit method has a default implementation that calls the corresponding
/// `walk_*` function, which performs recursive traversal of child nodes.
/// Override a visit method to insert custom logic before, after, or instead of
/// the default traversal.
pub trait Visitor<'a> {
    /// The result type produced by visitor operations.
    type Result: VisitorResult;

    /// Visits a variable name identifier.
    ///
    /// This is called when encountering an identifier in an expression context
    /// (e.g., variable references) or in a declarator (e.g., variable
    /// declarations).
    ///
    /// # Examples
    /// - `x` in `x = 5`
    /// - `printf` in `printf("hello")`
    /// - Variable declarations: `int x;`
    fn visit_variable_name(&mut self, _: &'a Identifier) -> Self::Result {
        Self::Result::output()
    }

    /// Visits a typedef name identifier.
    ///
    /// This is called when encountering a typedef name used as a type
    /// specifier.
    ///
    /// # Examples
    /// - `size_t` in `size_t len;`
    /// - `FILE` in `FILE* fp;`
    fn visit_type_name_identifier(&mut self, _: &'a Identifier) -> Self::Result {
        Self::Result::output()
    }

    /// Visits an enumeration constant identifier.
    ///
    /// This is called when encountering an enumeration constant in an
    /// expression.
    ///
    /// # Examples
    /// - `RED` in `color = RED;` where RED is an enum constant
    fn visit_enum_constant(&mut self, _: &'a Identifier) -> Self::Result {
        Self::Result::output()
    }

    /// Visits a label name identifier.
    ///
    /// This is called when encountering a goto label or a label in switch
    /// statements.
    ///
    /// # Examples
    /// - `error_handler:` in label declarations
    /// - `error_handler` in `goto error_handler;`
    fn visit_label_name(&mut self, _: &'a Identifier) -> Self::Result {
        Self::Result::output()
    }

    /// Visits a struct or union member name identifier.
    ///
    /// This is called when accessing a struct/union member via `.` or `->`
    /// operators.
    ///
    /// # Examples
    /// - `x` in `point.x`
    /// - `next` in `node->next`
    fn visit_member_name(&mut self, _: &'a Identifier) -> Self::Result {
        Self::Result::output()
    }

    /// Visits a struct type identifier.
    ///
    /// This is called when encountering a struct name in a struct specifier.
    ///
    /// # Examples
    /// - `Point` in `struct Point { int x; int y; }`
    /// - `Node` in `struct Node* next;`
    fn visit_struct_name(&mut self, _: &'a Identifier) -> Self::Result {
        Self::Result::output()
    }

    /// Visits an enum type identifier.
    ///
    /// This is called when encountering an enum name in an enum specifier.
    ///
    /// # Examples
    /// - `Color` in `enum Color { RED, GREEN, BLUE }`
    /// - `Status` in `enum Status code;`
    fn visit_enum_name(&mut self, _: &'a Identifier) -> Self::Result {
        Self::Result::output()
    }

    /// Visits an enumerator name identifier.
    ///
    /// This is called when encountering an enumerator in an enum definition.
    ///
    /// # Examples
    /// - `RED` in `enum Color { RED, GREEN, BLUE }`
    /// - Each name in enumeration values
    fn visit_enumerator_name(&mut self, _: &'a Identifier) -> Self::Result {
        Self::Result::output()
    }

    /// Visits a translation unit (the root of a C program).
    ///
    /// This is the top-level method that traverses the entire AST.
    /// Override this to perform initialization or finalization of analysis.
    fn visit_translation_unit(&mut self, tu: &'a TranslationUnit) -> Self::Result {
        walk_translation_unit(self, tu)
    }

    /// Visits an external declaration (function definition or global
    /// declaration).
    fn visit_external_declaration(&mut self, d: &'a ExternalDeclaration) -> Self::Result {
        walk_external_declaration(self, d)
    }

    /// Visits a function definition.
    fn visit_function_definition(&mut self, f: &'a FunctionDefinition) -> Self::Result {
        walk_function_definition(self, f)
    }

    /// Visits an attribute specifier.
    fn visit_attribute_specifier(&mut self, a: &'a AttributeSpecifier) -> Self::Result {
        walk_attribute_specifier(self, a)
    }

    /// Visits an attribute.
    fn visit_attribute(&mut self, _: &'a Attribute) -> Self::Result {
        Self::Result::output()
    }

    /// Visits an asm attribute specifier.
    fn visit_asm_attribute_specifier(&mut self, _: &'a StringLiterals) -> Self::Result {
        Self::Result::output()
    }

    /// Visits a statement.
    fn visit_statement(&mut self, s: &'a Statement) -> Self::Result {
        walk_statement(self, s)
    }

    /// Visits an unlabeled statement.
    fn visit_unlabeled_statement(&mut self, s: &'a UnlabeledStatement) -> Self::Result {
        walk_unlabeled_statement(self, s)
    }

    /// Visits an expression.
    fn visit_expression(&mut self, e: &'a Expression) -> Self::Result {
        walk_expression(self, e)
    }

    /// Visits a declaration.
    fn visit_declaration(&mut self, d: &'a Declaration) -> Self::Result {
        walk_declaration(self, d)
    }

    /// Visits a declarator.
    fn visit_declarator(&mut self, d: &'a Declarator) -> Self::Result {
        walk_declarator(self, d)
    }

    /// Visits a direct declarator.
    fn visit_direct_declarator(&mut self, d: &'a DirectDeclarator) -> Self::Result {
        walk_direct_declarator(self, d)
    }

    /// Visits a postfix expression.
    fn visit_postfix_expression(&mut self, p: &'a PostfixExpression) -> Self::Result {
        walk_postfix_expression(self, p)
    }

    /// Visits a unary expression.
    fn visit_unary_expression(&mut self, u: &'a UnaryExpression) -> Self::Result {
        walk_unary_expression(self, u)
    }

    /// Visits a cast expression.
    fn visit_cast_expression(&mut self, c: &'a CastExpression) -> Self::Result {
        walk_cast_expression(self, c)
    }

    /// Visits a compound statement (block of statements).
    fn visit_compound_statement(&mut self, c: &'a CompoundStatement) -> Self::Result {
        walk_compound_statement(self, c)
    }

    /// Visits declaration specifiers.
    fn visit_declaration_specifiers(&mut self, s: &'a DeclarationSpecifiers) -> Self::Result {
        walk_declaration_specifiers(self, s)
    }

    /// Visits a type specifier or qualifier.
    fn visit_type_specifier_qualifier(&mut self, x: &'a TypeSpecifierQualifier) -> Self::Result {
        walk_type_specifier_qualifier(self, x)
    }

    /// Visits a type specifier.
    fn visit_type_specifier(&mut self, ts: &'a TypeSpecifier) -> Self::Result {
        walk_type_specifier(self, ts)
    }

    /// Visits an atomic type specifier.
    fn visit_atomic_type_specifier(&mut self, a: &'a AtomicTypeSpecifier) -> Self::Result {
        walk_atomic_type_specifier(self, a)
    }

    /// Visits a typeof specifier.
    fn visit_typeof(&mut self, t: &'a TypeofSpecifier) -> Self::Result {
        walk_typeof(self, t)
    }

    /// Visits a specifier qualifier list.
    fn visit_specifier_qualifier_list(&mut self, s: &'a SpecifierQualifierList) -> Self::Result {
        walk_specifier_qualifier_list(self, s)
    }

    /// Visits a type name.
    fn visit_type_name(&mut self, tn: &'a TypeName) -> Self::Result {
        walk_type_name(self, tn)
    }

    /// Visits an abstract declarator.
    fn visit_abstract_declarator(&mut self, a: &'a AbstractDeclarator) -> Self::Result {
        walk_abstract_declarator(self, a)
    }

    /// Visits a direct abstract declarator.
    fn visit_direct_abstract_declarator(&mut self, d: &'a DirectAbstractDeclarator) -> Self::Result {
        walk_direct_abstract_declarator(self, d)
    }
}

// ============================================================================
// Default Walker Functions
// ============================================================================
//
// The functions in this section implement the default recursive traversal
// of the AST. They dispatch to the corresponding visitor methods for each
// node type. Most visitor implementations can use these default walkers
// by simply calling them from their visit methods.
//
// These functions perform depth-first traversal and support early termination
// via the ControlFlow type. When a visitor method returns ControlFlow::Break,
// the traversal stops and the value is propagated back to the caller.

/// Walks a translation unit, visiting each external declaration.
pub fn walk_translation_unit<'a, V: Visitor<'a> + ?Sized>(v: &mut V, tu: &'a TranslationUnit) -> V::Result {
    for ed in &tu.external_declarations {
        let br = v.visit_external_declaration(ed).branch();
        if let ControlFlow::Break(res) = br {
            return V::Result::from_residual(res);
        }
    }
    V::Result::output()
}

/// Walks an external declaration (function or declaration).
pub fn walk_external_declaration<'a, V: Visitor<'a> + ?Sized>(v: &mut V, d: &'a ExternalDeclaration) -> V::Result {
    match d {
        ExternalDeclaration::Function(f) => v.visit_function_definition(f),
        ExternalDeclaration::Declaration(d) => v.visit_declaration(d),
    }
}

/// Walks a function definition, visiting declaration specifiers, declarator,
/// and body.
pub fn walk_function_definition<'a, V: Visitor<'a> + ?Sized>(v: &mut V, f: &'a FunctionDefinition) -> V::Result {
    // walk attributes
    for attr in &f.attributes {
        let br = v.visit_attribute_specifier(attr).branch();
        if let ControlFlow::Break(res) = br {
            return V::Result::from_residual(res);
        }
    }
    // walk specifiers
    let br = v.visit_declaration_specifiers(&f.specifiers).branch();
    if let ControlFlow::Break(res) = br {
        return V::Result::from_residual(res);
    }
    // walk declarator
    let br = v.visit_declarator(&f.declarator).branch();
    if let ControlFlow::Break(res) = br {
        return V::Result::from_residual(res);
    }
    // walk body
    v.visit_compound_statement(&f.body)
}

/// Walks a statement, handling both labeled and unlabeled statements.
///
/// For labeled statements, calls `visit_label_name` for the label identifier
/// before walking the associated statement.
pub fn walk_statement<'a, V: Visitor<'a> + ?Sized>(v: &mut V, s: &'a Statement) -> V::Result {
    match s {
        Statement::Labeled(ls) => {
            // visit label identifiers if any
            match &ls.label {
                Label::Identifier { identifier, .. } => {
                    let br = v.visit_label_name(identifier).branch();
                    if let ControlFlow::Break(res) = br {
                        return V::Result::from_residual(res);
                    }
                }
                Label::Case { .. } | Label::Default { .. } => {}
            }
            v.visit_statement(&ls.statement)
        }
        Statement::Unlabeled(u) => v.visit_unlabeled_statement(u),
    }
}

/// Walks an unlabeled statement.
///
/// This handles expression statements, compound statements, selection
/// statements, iteration statements, and jump statements. For each type, visits
/// relevant sub-expressions, declarations, and statements.
pub fn walk_unlabeled_statement<'a, V: Visitor<'a> + ?Sized>(v: &mut V, s: &'a UnlabeledStatement) -> V::Result {
    match s {
        UnlabeledStatement::Expression(es) => {
            if let Some(expr) = &es.expression {
                v.visit_expression(expr)
            } else {
                V::Result::output()
            }
        }
        UnlabeledStatement::Primary { block, .. } => match block {
            PrimaryBlock::Compound(c) => {
                for item in &c.items {
                    let br = match item {
                        BlockItem::Declaration(d) => v.visit_declaration(d).branch(),
                        BlockItem::Statement(u) => v.visit_unlabeled_statement(u).branch(),
                        BlockItem::Label(l) => match l {
                            Label::Identifier { identifier, .. } => v.visit_label_name(identifier).branch(),
                            Label::Case { .. } | Label::Default { .. } => ControlFlow::Continue(()),
                        },
                    };
                    if let ControlFlow::Break(res) = br {
                        return V::Result::from_residual(res);
                    }
                }
                V::Result::output()
            }
            PrimaryBlock::Selection(sel) => match sel {
                SelectionStatement::If { condition, then_stmt, else_stmt } => {
                    let br = v.visit_expression(condition).branch();
                    if let ControlFlow::Break(res) = br {
                        return V::Result::from_residual(res);
                    }
                    let br = v.visit_statement(then_stmt).branch();
                    if let ControlFlow::Break(res) = br {
                        return V::Result::from_residual(res);
                    }
                    if let Some(e) = else_stmt {
                        v.visit_statement(e)
                    } else {
                        V::Result::output()
                    }
                }
                SelectionStatement::Switch { expression, statement } => {
                    let br = v.visit_expression(expression).branch();
                    if let ControlFlow::Break(res) = br {
                        return V::Result::from_residual(res);
                    }
                    v.visit_statement(statement)
                }
            },
            PrimaryBlock::Iteration(iter) => match iter {
                IterationStatement::While { condition, body } => {
                    let br = v.visit_expression(condition).branch();
                    if let ControlFlow::Break(res) = br {
                        return V::Result::from_residual(res);
                    }
                    v.visit_statement(body)
                }
                IterationStatement::DoWhile { body, condition } => {
                    let br = v.visit_statement(body).branch();
                    if let ControlFlow::Break(res) = br {
                        return V::Result::from_residual(res);
                    }
                    v.visit_expression(condition)
                }
                IterationStatement::For { init, condition, update, body } => {
                    if let Some(i) = init {
                        let br = match i {
                            ForInit::Expression(e) => v.visit_expression(e).branch(),
                            ForInit::Declaration(d) => v.visit_declaration(d).branch(),
                        };
                        if let ControlFlow::Break(res) = br {
                            return V::Result::from_residual(res);
                        }
                    }
                    if let Some(c) = condition {
                        let br = v.visit_expression(c).branch();
                        if let ControlFlow::Break(res) = br {
                            return V::Result::from_residual(res);
                        }
                    }
                    if let Some(u) = update {
                        let br = v.visit_expression(u).branch();
                        if let ControlFlow::Break(res) = br {
                            return V::Result::from_residual(res);
                        }
                    }
                    v.visit_statement(body)
                }
                IterationStatement::Error => V::Result::output(),
            },
        },
        UnlabeledStatement::Jump { statement, .. } => match statement {
            JumpStatement::Goto(id) => v.visit_label_name(id),
            JumpStatement::Continue | JumpStatement::Break => V::Result::output(),
            JumpStatement::Return(expr) => {
                if let Some(e) = expr {
                    v.visit_expression(e)
                } else {
                    V::Result::output()
                }
            }
        },
    }
}

/// Walks a compound statement (block), visiting declarations, statements, and
/// labels.
///
/// Iterates through all block items, dispatching to appropriate visitor
/// methods. For label identifiers within the block, calls `visit_label_name`.
pub fn walk_compound_statement<'a, V: Visitor<'a> + ?Sized>(v: &mut V, c: &'a CompoundStatement) -> V::Result {
    for item in &c.items {
        let br = match item {
            BlockItem::Declaration(d) => v.visit_declaration(d).branch(),
            BlockItem::Statement(u) => v.visit_unlabeled_statement(u).branch(),
            BlockItem::Label(l) => match l {
                Label::Identifier { identifier, .. } => v.visit_label_name(identifier).branch(),
                Label::Case { .. } | Label::Default { .. } => ControlFlow::Continue(()),
            },
        };
        if let ControlFlow::Break(res) = br {
            return V::Result::from_residual(res);
        }
    }
    V::Result::output()
}

/// Walks an expression, visiting all sub-expressions in depth-first order.
///
/// This includes handling of binary operators, conditional expressions,
/// assignments, comma expressions, and more.
pub fn walk_expression<'a, V: Visitor<'a> + ?Sized>(v: &mut V, e: &'a Expression) -> V::Result {
    match e {
        Expression::Postfix(p) => v.visit_postfix_expression(p),
        Expression::Unary(u) => v.visit_unary_expression(u),
        Expression::Cast(c) => v.visit_cast_expression(c),
        Expression::Binary(b) => {
            let br = v.visit_expression(&b.left).branch();
            if let ControlFlow::Break(res) = br {
                return V::Result::from_residual(res);
            }
            v.visit_expression(&b.right)
        }
        Expression::Conditional(c) => {
            let br = v.visit_expression(&c.condition).branch();
            if let ControlFlow::Break(res) = br {
                return V::Result::from_residual(res);
            }
            let br = v.visit_expression(&c.then_expr).branch();
            if let ControlFlow::Break(res) = br {
                return V::Result::from_residual(res);
            }
            v.visit_expression(&c.else_expr)
        }
        Expression::Assignment(a) => {
            let br = v.visit_expression(&a.left).branch();
            if let ControlFlow::Break(res) = br {
                return V::Result::from_residual(res);
            }
            v.visit_expression(&a.right)
        }
        Expression::Comma(c) => {
            for e in &c.expressions {
                let br = v.visit_expression(e).branch();
                if let ControlFlow::Break(res) = br {
                    return V::Result::from_residual(res);
                }
            }
            V::Result::output()
        }
        Expression::Error => V::Result::output(),
    }
}

/// Walks a postfix expression.
///
/// Handles primary expressions (identifiers, enum constants, literals, etc.),
/// array access, function calls, member access, and increment/decrement
/// operators. Calls `visit_variable_name` for identifiers and
/// `visit_enum_constant` for enumeration constants. Calls `visit_member_name`
/// for member access operations.
pub fn walk_postfix_expression<'a, V: Visitor<'a> + ?Sized>(v: &mut V, p: &'a PostfixExpression) -> V::Result {
    match p {
        PostfixExpression::Primary(pr) => match pr {
            PrimaryExpression::Identifier(id) => v.visit_variable_name(id),
            PrimaryExpression::EnumerationConstant(id) => v.visit_enum_constant(id),
            PrimaryExpression::Parenthesized(e) => v.visit_expression(e),
            PrimaryExpression::Generic(g) => {
                let br = v.visit_expression(&g.controlling_expression).branch();
                if let ControlFlow::Break(res) = br {
                    return V::Result::from_residual(res);
                }
                for assoc in &g.associations {
                    let br = match assoc {
                        GenericAssociation::Type { expression, .. } => v.visit_expression(expression).branch(),
                        GenericAssociation::Default { expression } => v.visit_expression(expression).branch(),
                    };
                    if let ControlFlow::Break(res) = br {
                        return V::Result::from_residual(res);
                    }
                }
                V::Result::output()
            }
            _ => V::Result::output(),
        },
        PostfixExpression::ArrayAccess { array, index } => {
            let br = v.visit_postfix_expression(array).branch();
            if let ControlFlow::Break(res) = br {
                return V::Result::from_residual(res);
            }
            v.visit_expression(index)
        }
        PostfixExpression::FunctionCall { function, arguments } => {
            let br = v.visit_postfix_expression(function).branch();
            if let ControlFlow::Break(res) = br {
                return V::Result::from_residual(res);
            }
            for a in arguments {
                let br = v.visit_expression(a).branch();
                if let ControlFlow::Break(res) = br {
                    return V::Result::from_residual(res);
                }
            }
            V::Result::output()
        }
        PostfixExpression::MemberAccess { object, member } => {
            let br = v.visit_postfix_expression(object).branch();
            if let ControlFlow::Break(res) = br {
                return V::Result::from_residual(res);
            }
            v.visit_member_name(member)
        }
        PostfixExpression::MemberAccessPtr { object, member } => {
            let br = v.visit_postfix_expression(object).branch();
            if let ControlFlow::Break(res) = br {
                return V::Result::from_residual(res);
            }
            v.visit_member_name(member)
        }
        PostfixExpression::PostIncrement(inner) | PostfixExpression::PostDecrement(inner) => {
            v.visit_postfix_expression(inner)
        }
        PostfixExpression::CompoundLiteral(cl) => {
            // walk type name and initializer
            v.visit_type_name(&cl.type_name)
        }
    }
}

/// Walks a unary expression.
///
/// Handles pre/post increment/decrement, unary operators (address-of,
/// dereference, etc.), sizeof, and alignof operations.
pub fn walk_unary_expression<'a, V: Visitor<'a> + ?Sized>(v: &mut V, u: &'a UnaryExpression) -> V::Result {
    match u {
        UnaryExpression::Postfix(p) => v.visit_postfix_expression(p),
        UnaryExpression::PreIncrement(inner) | UnaryExpression::PreDecrement(inner) => v.visit_unary_expression(inner),
        UnaryExpression::Unary { operand, .. } => v.visit_cast_expression(operand),
        UnaryExpression::Sizeof(inner) => v.visit_unary_expression(inner),
        UnaryExpression::SizeofType(tn) | UnaryExpression::Alignof(tn) => v.visit_type_name(tn),
    }
}

/// Walks a cast expression.
///
/// Handles unary expressions and explicit type casts.
pub fn walk_cast_expression<'a, V: Visitor<'a> + ?Sized>(v: &mut V, c: &'a CastExpression) -> V::Result {
    match c {
        CastExpression::Unary(u) => v.visit_unary_expression(u),
        CastExpression::Cast { type_name, expression } => {
            let br = v.visit_type_name(type_name).branch();
            if let ControlFlow::Break(res) = br {
                return V::Result::from_residual(res);
            }
            v.visit_cast_expression(expression)
        }
    }
}

/// Walks a declaration.
///
/// Handles normal declarations, typedef declarations, static assertions, and
/// attributes. Visits declaration specifiers and declarators for each
/// declaration.
pub fn walk_declaration<'a, V: Visitor<'a> + ?Sized>(v: &mut V, d: &'a Declaration) -> V::Result {
    match d {
        Declaration::Normal { attributes, specifiers, declarators } => {
            for attr in attributes {
                let br = v.visit_attribute_specifier(attr).branch();
                if let ControlFlow::Break(res) = br {
                    return V::Result::from_residual(res);
                }
            }
            let br = v.visit_declaration_specifiers(specifiers).branch();
            if let ControlFlow::Break(res) = br {
                return V::Result::from_residual(res);
            }
            for init in declarators {
                let br = v.visit_declarator(&init.declarator).branch();
                if let ControlFlow::Break(res) = br {
                    return V::Result::from_residual(res);
                }
            }
            V::Result::output()
        }
        Declaration::Typedef { attributes, specifiers, declarators } => {
            for attr in attributes {
                let br = v.visit_attribute_specifier(attr).branch();
                if let ControlFlow::Break(res) = br {
                    return V::Result::from_residual(res);
                }
            }
            let br = v.visit_declaration_specifiers(specifiers).branch();
            if let ControlFlow::Break(res) = br {
                return V::Result::from_residual(res);
            }
            for d in declarators {
                let br = v.visit_declarator(d).branch();
                if let ControlFlow::Break(res) = br {
                    return V::Result::from_residual(res);
                }
            }
            V::Result::output()
        }
        Declaration::StaticAssert(sa) => {
            if let Some(_m) = &sa.message { /* no-op walk for message */ }
            V::Result::output()
        }
        Declaration::Attribute(attrs) => {
            for attr in attrs {
                let br = v.visit_attribute_specifier(attr).branch();
                if let ControlFlow::Break(res) = br {
                    return V::Result::from_residual(res);
                }
            }
            V::Result::output()
        }
        Declaration::Error => V::Result::output(),
    }
}

/// Walks declaration specifiers (storage class, type specifiers, qualifiers,
/// etc.).
pub fn walk_declaration_specifiers<'a, V: Visitor<'a> + ?Sized>(v: &mut V, s: &'a DeclarationSpecifiers) -> V::Result {
    for it in &s.specifiers {
        let br = match it {
            DeclarationSpecifier::StorageClass(_) => ControlFlow::Continue(()),
            DeclarationSpecifier::TypeSpecifierQualifier(tsq) => v.visit_type_specifier_qualifier(tsq).branch(),
            DeclarationSpecifier::Function { .. } => ControlFlow::Continue(()),
        };
        if let ControlFlow::Break(res) = br {
            return V::Result::from_residual(res);
        }
    }
    V::Result::output()
}

/// Walks a type specifier or qualifier.
///
/// Dispatches to type specifiers or ignores qualifiers.
pub fn walk_type_specifier_qualifier<'a, V: Visitor<'a> + ?Sized>(
    v: &mut V,
    x: &'a TypeSpecifierQualifier,
) -> V::Result {
    match x {
        TypeSpecifierQualifier::TypeSpecifier(ts) => v.visit_type_specifier(ts),
        TypeSpecifierQualifier::TypeQualifier(_) => V::Result::output(),
        TypeSpecifierQualifier::AlignmentSpecifier(a) => match a {
            AlignmentSpecifier::Type(tn) => v.visit_type_name(tn),
            AlignmentSpecifier::Expression(_) => V::Result::output(),
        },
    }
}

/// Walks a type specifier.
///
/// Handles struct, union, enum, typedef names, atomic types, and typeof
/// specifiers. Calls `visit_struct_name` for struct identifiers,
/// `visit_enum_name` for enum identifiers, `visit_enumerator_name` for
/// enumerators, and `visit_type_name_identifier` for typedef names.
pub fn walk_type_specifier<'a, V: Visitor<'a> + ?Sized>(v: &mut V, ts: &'a TypeSpecifier) -> V::Result {
    match ts {
        TypeSpecifier::Struct(s) => {
            if let Some(id) = &s.identifier {
                let br = v.visit_struct_name(id).branch();
                if let ControlFlow::Break(res) = br {
                    return V::Result::from_residual(res);
                }
            }
            if let Some(members) = &s.members {
                for m in members {
                    let br = match m {
                        MemberDeclaration::Normal { specifiers, declarators, .. } => {
                            let br2 = v.visit_specifier_qualifier_list(specifiers).branch();
                            if let ControlFlow::Break(res) = br2 {
                                return V::Result::from_residual(res);
                            }
                            for d in declarators {
                                let br3 = match d {
                                    MemberDeclarator::Declarator(dd) => v.visit_declarator(dd).branch(),
                                    MemberDeclarator::BitField { declarator, .. } => {
                                        if let Some(dd) = declarator {
                                            v.visit_declarator(dd).branch()
                                        } else {
                                            ControlFlow::Continue(())
                                        }
                                    }
                                };
                                if let ControlFlow::Break(res) = br3 {
                                    return V::Result::from_residual(res);
                                }
                            }
                            ControlFlow::Continue(())
                        }
                        MemberDeclaration::StaticAssert(_) => ControlFlow::Continue(()),
                        MemberDeclaration::Error => ControlFlow::Continue(()),
                    };
                    if let ControlFlow::Break(res) = br {
                        return V::Result::from_residual(res);
                    }
                }
            }
            V::Result::output()
        }
        TypeSpecifier::Enum(e) => {
            if let Some(id) = &e.identifier {
                let br = v.visit_enum_name(id).branch();
                if let ControlFlow::Break(res) = br {
                    return V::Result::from_residual(res);
                }
            }
            if let Some(enumerators) = &e.enumerators {
                for en in enumerators {
                    let br = v.visit_enumerator_name(&en.name).branch();
                    if let ControlFlow::Break(res) = br {
                        return V::Result::from_residual(res);
                    }
                }
            }
            V::Result::output()
        }
        TypeSpecifier::TypedefName(id) => v.visit_type_name_identifier(id),
        TypeSpecifier::Atomic(a) => v.visit_atomic_type_specifier(a),
        TypeSpecifier::Typeof(t) => v.visit_typeof(t),
        TypeSpecifier::BitInt(_)
        | TypeSpecifier::Void
        | TypeSpecifier::Char
        | TypeSpecifier::Short
        | TypeSpecifier::Int
        | TypeSpecifier::Long
        | TypeSpecifier::Float
        | TypeSpecifier::Double
        | TypeSpecifier::Signed
        | TypeSpecifier::Unsigned
        | TypeSpecifier::Bool
        | TypeSpecifier::Complex
        | TypeSpecifier::Decimal32
        | TypeSpecifier::Decimal64
        | TypeSpecifier::Decimal128 => V::Result::output(),
    }
}

/// Walks an atomic type specifier.
pub fn walk_atomic_type_specifier<'a, V: Visitor<'a> + ?Sized>(v: &mut V, a: &'a AtomicTypeSpecifier) -> V::Result {
    v.visit_type_name(&a.type_name)
}

/// Walks a typeof specifier.
///
/// The argument can be an expression or a type name.
pub fn walk_typeof<'a, V: Visitor<'a> + ?Sized>(v: &mut V, t: &'a TypeofSpecifier) -> V::Result {
    match t {
        TypeofSpecifier::Typeof(arg) | TypeofSpecifier::TypeofUnqual(arg) => match arg {
            TypeofSpecifierArgument::Expression(e) => v.visit_expression(e),
            TypeofSpecifierArgument::TypeName(tn) => v.visit_type_name(tn),
            TypeofSpecifierArgument::Error => V::Result::output(),
        },
    }
}

/// Walks a specifier qualifier list.
///
/// This is used for type names in casts, sizeof/alignof expressions, etc.
pub fn walk_specifier_qualifier_list<'a, V: Visitor<'a> + ?Sized>(
    v: &mut V,
    s: &'a SpecifierQualifierList,
) -> V::Result {
    for item in &s.items {
        let br = v.visit_type_specifier_qualifier(item).branch();
        if let ControlFlow::Break(res) = br {
            return V::Result::from_residual(res);
        }
    }
    V::Result::output()
}

/// Walks a declarator.
///
/// Recursively handles pointer, direct, and error declarators.
pub fn walk_declarator<'a, V: Visitor<'a> + ?Sized>(v: &mut V, d: &'a Declarator) -> V::Result {
    match d {
        Declarator::Direct(dd) => v.visit_direct_declarator(dd),
        Declarator::Pointer { declarator, .. } => v.visit_declarator(declarator),
        Declarator::Error => V::Result::output(),
    }
}

/// Walks a direct declarator.
///
/// Handles identifier declarators (where `visit_variable_name` is called),
/// parenthesized declarators, array declarators, and function declarators.
/// For function declarators, visits parameter declarations.
pub fn walk_direct_declarator<'a, V: Visitor<'a> + ?Sized>(v: &mut V, d: &'a DirectDeclarator) -> V::Result {
    match d {
        DirectDeclarator::Identifier { identifier, attributes } => {
            let br = v.visit_variable_name(identifier).branch();
            if let ControlFlow::Break(res) = br {
                return V::Result::from_residual(res);
            }
            for attr in attributes {
                let br = v.visit_attribute_specifier(attr).branch();
                if let ControlFlow::Break(res) = br {
                    return V::Result::from_residual(res);
                }
            }
            V::Result::output()
        }
        DirectDeclarator::Parenthesized(inner) => v.visit_declarator(inner),
        DirectDeclarator::Array { declarator, attributes, .. } => {
            let br = v.visit_direct_declarator(declarator).branch();
            if let ControlFlow::Break(res) = br {
                return V::Result::from_residual(res);
            }
            for attr in attributes {
                let br = v.visit_attribute_specifier(attr).branch();
                if let ControlFlow::Break(res) = br {
                    return V::Result::from_residual(res);
                }
            }
            V::Result::output()
        }
        DirectDeclarator::Function { declarator, attributes, parameters } => {
            let br = v.visit_direct_declarator(declarator).branch();
            if let ControlFlow::Break(res) = br {
                return V::Result::from_residual(res);
            }
            for attr in attributes {
                let br = v.visit_attribute_specifier(attr).branch();
                if let ControlFlow::Break(res) = br {
                    return V::Result::from_residual(res);
                }
            }
            match parameters {
                ParameterTypeList::Parameters(params) | ParameterTypeList::Variadic(params) => {
                    for p in params {
                        let br = v.visit_declaration_specifiers(&p.specifiers).branch();
                        if let ControlFlow::Break(res) = br {
                            return V::Result::from_residual(res);
                        }
                        if let Some(kind) = &p.declarator {
                            let br = match kind {
                                ParameterDeclarationKind::Declarator(d) => v.visit_declarator(d).branch(),
                                ParameterDeclarationKind::Abstract(a) => v.visit_abstract_declarator(a).branch(),
                            };
                            if let ControlFlow::Break(res) = br {
                                return V::Result::from_residual(res);
                            }
                        }
                    }
                }
                ParameterTypeList::OnlyVariadic => {}
            }
            V::Result::output()
        }
    }
}

/// Walks a type name (used in casts, sizeof, alignof, etc.).
pub fn walk_type_name<'a, V: Visitor<'a> + ?Sized>(v: &mut V, tn: &'a TypeName) -> V::Result {
    match tn {
        TypeName::TypeName { specifiers, abstract_declarator } => {
            let br = v.visit_specifier_qualifier_list(specifiers).branch();
            if let ControlFlow::Break(res) = br {
                return V::Result::from_residual(res);
            }
            if let Some(ad) = abstract_declarator {
                v.visit_abstract_declarator(ad)
            } else {
                V::Result::output()
            }
        }
        TypeName::Error => V::Result::output(),
    }
}

/// Walks an abstract declarator.
///
/// Used in type names where declarators can omit names.
/// Recursively handles pointers and direct abstract declarators.
pub fn walk_abstract_declarator<'a, V: Visitor<'a> + ?Sized>(v: &mut V, a: &'a AbstractDeclarator) -> V::Result {
    match a {
        AbstractDeclarator::Direct(d) => v.visit_direct_abstract_declarator(d),
        AbstractDeclarator::Pointer { abstract_declarator, .. } => {
            if let Some(ad) = abstract_declarator {
                v.visit_abstract_declarator(ad)
            } else {
                V::Result::output()
            }
        }
        AbstractDeclarator::Error => V::Result::output(),
    }
}

/// Walks a direct abstract declarator.
///
/// Handles array and function declarators without identifiers,
/// used in type names and abstract declarators.
pub fn walk_direct_abstract_declarator<'a, V: Visitor<'a> + ?Sized>(
    v: &mut V,
    d: &'a DirectAbstractDeclarator,
) -> V::Result {
    match d {
        DirectAbstractDeclarator::Parenthesized(ad) => v.visit_abstract_declarator(ad),
        DirectAbstractDeclarator::Array { declarator, .. } => {
            if let Some(dd) = declarator {
                v.visit_direct_abstract_declarator(dd)
            } else {
                V::Result::output()
            }
        }
        DirectAbstractDeclarator::Function { declarator, .. } => {
            if let Some(dd) = declarator {
                v.visit_direct_abstract_declarator(dd)
            } else {
                V::Result::output()
            }
        }
    }
}

/// Walks an attribute specifier.
///
/// This is called when encountering an attribute specifier in a function
/// definition.
pub fn walk_attribute_specifier<'a, V: Visitor<'a> + ?Sized>(v: &mut V, a: &'a AttributeSpecifier) -> V::Result {
    match a {
        AttributeSpecifier::Attributes(attributes) => {
            for attr in attributes {
                let br = v.visit_attribute(attr).branch();
                if let ControlFlow::Break(res) = br {
                    return V::Result::from_residual(res);
                }
            }
        }
        AttributeSpecifier::Asm(string_literals) => {
            let br = v.visit_asm_attribute_specifier(string_literals).branch();
            if let ControlFlow::Break(res) = br {
                return V::Result::from_residual(res);
            }
        }
        AttributeSpecifier::Error => {}
    }
    V::Result::output()
}

// ============================================================================
// Mutable Visitor Trait
// ============================================================================

/// The mutable visitor trait for traversing and modifying C AST nodes.
///
/// This trait is similar to [`Visitor`] but provides mutable access to AST
/// nodes, allowing modifications during traversal.
///
/// # Example
///
/// ```ignore
/// struct RenameVisitor {
///     old_name: String,
///     new_name: String,
/// }
///
/// impl<'a> VisitorMut<'a> for RenameVisitor {
///     type Result = ();
///
///     fn visit_variable_name_mut(&mut self, id: &'a mut Identifier) {
///         if id.name == self.old_name {
///             id.name = self.new_name.clone();
///         }
///         Self::Result::output()
///     }
/// }
/// ```
pub trait VisitorMut<'a> {
    /// The result type produced by visitor operations.
    type Result: VisitorResult;

    /// Visits a variable name identifier with mutable access.
    fn visit_variable_name_mut(&mut self, _: &'a mut Identifier) -> Self::Result {
        Self::Result::output()
    }

    /// Visits a typedef name identifier with mutable access.
    fn visit_type_name_identifier_mut(&mut self, _: &'a mut Identifier) -> Self::Result {
        Self::Result::output()
    }

    /// Visits an enumeration constant identifier with mutable access.
    fn visit_enum_constant_mut(&mut self, _: &'a mut Identifier) -> Self::Result {
        Self::Result::output()
    }

    /// Visits a label name identifier with mutable access.
    fn visit_label_name_mut(&mut self, _: &'a mut Identifier) -> Self::Result {
        Self::Result::output()
    }

    /// Visits a struct or union member name identifier with mutable access.
    fn visit_member_name_mut(&mut self, _: &'a mut Identifier) -> Self::Result {
        Self::Result::output()
    }

    /// Visits a struct type identifier with mutable access.
    fn visit_struct_name_mut(&mut self, _: &'a mut Identifier) -> Self::Result {
        Self::Result::output()
    }

    /// Visits an enum type identifier with mutable access.
    fn visit_enum_name_mut(&mut self, _: &'a mut Identifier) -> Self::Result {
        Self::Result::output()
    }

    /// Visits an enumerator name identifier with mutable access.
    fn visit_enumerator_name_mut(&mut self, _: &'a mut Identifier) -> Self::Result {
        Self::Result::output()
    }

    /// Visits a translation unit with mutable access.
    fn visit_translation_unit_mut(&mut self, tu: &'a mut TranslationUnit) -> Self::Result {
        walk_translation_unit_mut(self, tu)
    }

    /// Visits an external declaration with mutable access.
    fn visit_external_declaration_mut(&mut self, d: &'a mut ExternalDeclaration) -> Self::Result {
        walk_external_declaration_mut(self, d)
    }

    /// Visits a function definition with mutable access.
    fn visit_function_definition_mut(&mut self, f: &'a mut FunctionDefinition) -> Self::Result {
        walk_function_definition_mut(self, f)
    }

    /// Visits an attribute specifier with mutable access.
    fn visit_attribute_specifier_mut(&mut self, a: &'a mut AttributeSpecifier) -> Self::Result {
        walk_attribute_specifier_mut(self, a)
    }

    /// Visits an attribute with mutable access.
    fn visit_attribute_mut(&mut self, _: &'a mut Attribute) -> Self::Result {
        Self::Result::output()
    }

    /// Visits an asm attribute specifier with mutable access.
    fn visit_asm_attribute_specifier_mut(&mut self, _: &'a mut StringLiterals) -> Self::Result {
        Self::Result::output()
    }

    /// Visits a statement with mutable access.
    fn visit_statement_mut(&mut self, s: &'a mut Statement) -> Self::Result {
        walk_statement_mut(self, s)
    }

    /// Visits an unlabeled statement with mutable access.
    fn visit_unlabeled_statement_mut(&mut self, s: &'a mut UnlabeledStatement) -> Self::Result {
        walk_unlabeled_statement_mut(self, s)
    }

    /// Visits an expression with mutable access.
    fn visit_expression_mut(&mut self, e: &'a mut Expression) -> Self::Result {
        walk_expression_mut(self, e)
    }

    /// Visits a declaration with mutable access.
    fn visit_declaration_mut(&mut self, d: &'a mut Declaration) -> Self::Result {
        walk_declaration_mut(self, d)
    }

    /// Visits a declarator with mutable access.
    fn visit_declarator_mut(&mut self, d: &'a mut Declarator) -> Self::Result {
        walk_declarator_mut(self, d)
    }

    /// Visits a direct declarator with mutable access.
    fn visit_direct_declarator_mut(&mut self, d: &'a mut DirectDeclarator) -> Self::Result {
        walk_direct_declarator_mut(self, d)
    }

    /// Visits a postfix expression with mutable access.
    fn visit_postfix_expression_mut(&mut self, p: &'a mut PostfixExpression) -> Self::Result {
        walk_postfix_expression_mut(self, p)
    }

    /// Visits a unary expression with mutable access.
    fn visit_unary_expression_mut(&mut self, u: &'a mut UnaryExpression) -> Self::Result {
        walk_unary_expression_mut(self, u)
    }

    /// Visits a cast expression with mutable access.
    fn visit_cast_expression_mut(&mut self, c: &'a mut CastExpression) -> Self::Result {
        walk_cast_expression_mut(self, c)
    }

    /// Visits a compound statement with mutable access.
    fn visit_compound_statement_mut(&mut self, c: &'a mut CompoundStatement) -> Self::Result {
        walk_compound_statement_mut(self, c)
    }

    /// Visits declaration specifiers with mutable access.
    fn visit_declaration_specifiers_mut(&mut self, s: &'a mut DeclarationSpecifiers) -> Self::Result {
        walk_declaration_specifiers_mut(self, s)
    }

    /// Visits a type specifier or qualifier with mutable access.
    fn visit_type_specifier_qualifier_mut(&mut self, x: &'a mut TypeSpecifierQualifier) -> Self::Result {
        walk_type_specifier_qualifier_mut(self, x)
    }

    /// Visits a type specifier with mutable access.
    fn visit_type_specifier_mut(&mut self, ts: &'a mut TypeSpecifier) -> Self::Result {
        walk_type_specifier_mut(self, ts)
    }

    /// Visits an atomic type specifier with mutable access.
    fn visit_atomic_type_specifier_mut(&mut self, a: &'a mut AtomicTypeSpecifier) -> Self::Result {
        walk_atomic_type_specifier_mut(self, a)
    }

    /// Visits a typeof specifier with mutable access.
    fn visit_typeof_mut(&mut self, t: &'a mut TypeofSpecifier) -> Self::Result {
        walk_typeof_mut(self, t)
    }

    /// Visits a specifier qualifier list with mutable access.
    fn visit_specifier_qualifier_list_mut(&mut self, s: &'a mut SpecifierQualifierList) -> Self::Result {
        walk_specifier_qualifier_list_mut(self, s)
    }

    /// Visits a type name with mutable access.
    fn visit_type_name_mut(&mut self, tn: &'a mut TypeName) -> Self::Result {
        walk_type_name_mut(self, tn)
    }

    /// Visits an abstract declarator with mutable access.
    fn visit_abstract_declarator_mut(&mut self, a: &'a mut AbstractDeclarator) -> Self::Result {
        walk_abstract_declarator_mut(self, a)
    }

    /// Visits a direct abstract declarator with mutable access.
    fn visit_direct_abstract_declarator_mut(&mut self, d: &'a mut DirectAbstractDeclarator) -> Self::Result {
        walk_direct_abstract_declarator_mut(self, d)
    }
}

// ============================================================================
// Mutable Walker Functions
// ============================================================================

/// Walks a translation unit with mutable access.
pub fn walk_translation_unit_mut<'a, V: VisitorMut<'a> + ?Sized>(v: &mut V, tu: &'a mut TranslationUnit) -> V::Result {
    for ed in &mut tu.external_declarations {
        let br = v.visit_external_declaration_mut(ed).branch();
        if let ControlFlow::Break(res) = br {
            return V::Result::from_residual(res);
        }
    }
    V::Result::output()
}

/// Walks an external declaration with mutable access.
pub fn walk_external_declaration_mut<'a, V: VisitorMut<'a> + ?Sized>(
    v: &mut V,
    d: &'a mut ExternalDeclaration,
) -> V::Result {
    match d {
        ExternalDeclaration::Function(f) => v.visit_function_definition_mut(f),
        ExternalDeclaration::Declaration(d) => v.visit_declaration_mut(d),
    }
}

/// Walks a function definition with mutable access.
pub fn walk_function_definition_mut<'a, V: VisitorMut<'a> + ?Sized>(
    v: &mut V,
    f: &'a mut FunctionDefinition,
) -> V::Result {
    for attr in &mut f.attributes {
        let br = v.visit_attribute_specifier_mut(attr).branch();
        if let ControlFlow::Break(res) = br {
            return V::Result::from_residual(res);
        }
    }
    let br = v.visit_declaration_specifiers_mut(&mut f.specifiers).branch();
    if let ControlFlow::Break(res) = br {
        return V::Result::from_residual(res);
    }
    let br = v.visit_declarator_mut(&mut f.declarator).branch();
    if let ControlFlow::Break(res) = br {
        return V::Result::from_residual(res);
    }
    v.visit_compound_statement_mut(&mut f.body)
}

/// Walks a statement with mutable access.
pub fn walk_statement_mut<'a, V: VisitorMut<'a> + ?Sized>(v: &mut V, s: &'a mut Statement) -> V::Result {
    match s {
        Statement::Labeled(ls) => {
            match &mut ls.label {
                Label::Identifier { identifier, .. } => {
                    let br = v.visit_label_name_mut(identifier).branch();
                    if let ControlFlow::Break(res) = br {
                        return V::Result::from_residual(res);
                    }
                }
                Label::Case { .. } | Label::Default { .. } => {}
            }
            v.visit_statement_mut(&mut ls.statement)
        }
        Statement::Unlabeled(u) => v.visit_unlabeled_statement_mut(u),
    }
}

/// Walks an unlabeled statement with mutable access.
pub fn walk_unlabeled_statement_mut<'a, V: VisitorMut<'a> + ?Sized>(
    v: &mut V,
    s: &'a mut UnlabeledStatement,
) -> V::Result {
    match s {
        UnlabeledStatement::Expression(es) => {
            if let Some(expr) = &mut es.expression {
                v.visit_expression_mut(expr)
            } else {
                V::Result::output()
            }
        }
        UnlabeledStatement::Primary { block, .. } => match block {
            PrimaryBlock::Compound(c) => {
                for item in &mut c.items {
                    let br = match item {
                        BlockItem::Declaration(d) => v.visit_declaration_mut(d).branch(),
                        BlockItem::Statement(u) => v.visit_unlabeled_statement_mut(u).branch(),
                        BlockItem::Label(l) => match l {
                            Label::Identifier { identifier, .. } => v.visit_label_name_mut(identifier).branch(),
                            Label::Case { .. } | Label::Default { .. } => ControlFlow::Continue(()),
                        },
                    };
                    if let ControlFlow::Break(res) = br {
                        return V::Result::from_residual(res);
                    }
                }
                V::Result::output()
            }
            PrimaryBlock::Selection(sel) => match sel {
                SelectionStatement::If { condition, then_stmt, else_stmt } => {
                    let br = v.visit_expression_mut(condition).branch();
                    if let ControlFlow::Break(res) = br {
                        return V::Result::from_residual(res);
                    }
                    let br = v.visit_statement_mut(then_stmt).branch();
                    if let ControlFlow::Break(res) = br {
                        return V::Result::from_residual(res);
                    }
                    if let Some(else_stmt) = else_stmt {
                        v.visit_statement_mut(else_stmt)
                    } else {
                        V::Result::output()
                    }
                }
                SelectionStatement::Switch { expression, statement } => {
                    let br = v.visit_expression_mut(expression).branch();
                    if let ControlFlow::Break(res) = br {
                        return V::Result::from_residual(res);
                    }
                    v.visit_statement_mut(statement)
                }
            },
            PrimaryBlock::Iteration(iter) => match iter {
                IterationStatement::While { condition, body } => {
                    let br = v.visit_expression_mut(condition).branch();
                    if let ControlFlow::Break(res) = br {
                        return V::Result::from_residual(res);
                    }
                    v.visit_statement_mut(body)
                }
                IterationStatement::DoWhile { body, condition } => {
                    let br = v.visit_statement_mut(body).branch();
                    if let ControlFlow::Break(res) = br {
                        return V::Result::from_residual(res);
                    }
                    v.visit_expression_mut(condition)
                }
                IterationStatement::For { init, condition, update, body } => {
                    if let Some(init) = init {
                        let br = match init {
                            ForInit::Expression(e) => v.visit_expression_mut(e).branch(),
                            ForInit::Declaration(d) => v.visit_declaration_mut(d).branch(),
                        };
                        if let ControlFlow::Break(res) = br {
                            return V::Result::from_residual(res);
                        }
                    }
                    if let Some(condition) = condition {
                        let br = v.visit_expression_mut(condition).branch();
                        if let ControlFlow::Break(res) = br {
                            return V::Result::from_residual(res);
                        }
                    }
                    if let Some(update) = update {
                        let br = v.visit_expression_mut(update).branch();
                        if let ControlFlow::Break(res) = br {
                            return V::Result::from_residual(res);
                        }
                    }
                    v.visit_statement_mut(body)
                }
                IterationStatement::Error => V::Result::output(),
            },
        },
        UnlabeledStatement::Jump { statement, .. } => match statement {
            JumpStatement::Goto(id) => v.visit_label_name_mut(id),
            JumpStatement::Continue | JumpStatement::Break => V::Result::output(),
            JumpStatement::Return(expr) => {
                if let Some(e) = expr {
                    v.visit_expression_mut(e)
                } else {
                    V::Result::output()
                }
            }
        },
    }
}

/// Walks a compound statement with mutable access.
pub fn walk_compound_statement_mut<'a, V: VisitorMut<'a> + ?Sized>(
    v: &mut V,
    c: &'a mut CompoundStatement,
) -> V::Result {
    for item in &mut c.items {
        let br = match item {
            BlockItem::Declaration(d) => v.visit_declaration_mut(d).branch(),
            BlockItem::Statement(u) => v.visit_unlabeled_statement_mut(u).branch(),
            BlockItem::Label(l) => match l {
                Label::Identifier { identifier, .. } => v.visit_label_name_mut(identifier).branch(),
                Label::Case { .. } | Label::Default { .. } => ControlFlow::Continue(()),
            },
        };
        if let ControlFlow::Break(res) = br {
            return V::Result::from_residual(res);
        }
    }
    V::Result::output()
}

/// Walks an expression with mutable access.
pub fn walk_expression_mut<'a, V: VisitorMut<'a> + ?Sized>(v: &mut V, e: &'a mut Expression) -> V::Result {
    match e {
        Expression::Postfix(p) => v.visit_postfix_expression_mut(p),
        Expression::Unary(u) => v.visit_unary_expression_mut(u),
        Expression::Cast(c) => v.visit_cast_expression_mut(c),
        Expression::Binary(b) => {
            let br = v.visit_expression_mut(&mut b.left).branch();
            if let ControlFlow::Break(res) = br {
                return V::Result::from_residual(res);
            }
            v.visit_expression_mut(&mut b.right)
        }
        Expression::Conditional(c) => {
            let br = v.visit_expression_mut(&mut c.condition).branch();
            if let ControlFlow::Break(res) = br {
                return V::Result::from_residual(res);
            }
            let br = v.visit_expression_mut(&mut c.then_expr).branch();
            if let ControlFlow::Break(res) = br {
                return V::Result::from_residual(res);
            }
            v.visit_expression_mut(&mut c.else_expr)
        }
        Expression::Assignment(a) => {
            let br = v.visit_expression_mut(&mut a.left).branch();
            if let ControlFlow::Break(res) = br {
                return V::Result::from_residual(res);
            }
            v.visit_expression_mut(&mut a.right)
        }
        Expression::Comma(c) => {
            for e in &mut c.expressions {
                let br = v.visit_expression_mut(e).branch();
                if let ControlFlow::Break(res) = br {
                    return V::Result::from_residual(res);
                }
            }
            V::Result::output()
        }
        Expression::Error => V::Result::output(),
    }
}

/// Walks a postfix expression with mutable access.
pub fn walk_postfix_expression_mut<'a, V: VisitorMut<'a> + ?Sized>(
    v: &mut V,
    p: &'a mut PostfixExpression,
) -> V::Result {
    match p {
        PostfixExpression::Primary(pr) => match pr {
            PrimaryExpression::Identifier(id) => v.visit_variable_name_mut(id),
            PrimaryExpression::EnumerationConstant(id) => v.visit_enum_constant_mut(id),
            PrimaryExpression::Parenthesized(e) => v.visit_expression_mut(e),
            PrimaryExpression::Generic(g) => {
                let br = v.visit_expression_mut(&mut g.controlling_expression).branch();
                if let ControlFlow::Break(res) = br {
                    return V::Result::from_residual(res);
                }
                for assoc in &mut g.associations {
                    let br = match assoc {
                        GenericAssociation::Type { expression, .. } => v.visit_expression_mut(expression).branch(),
                        GenericAssociation::Default { expression } => v.visit_expression_mut(expression).branch(),
                    };
                    if let ControlFlow::Break(res) = br {
                        return V::Result::from_residual(res);
                    }
                }
                V::Result::output()
            }
            _ => V::Result::output(),
        },
        PostfixExpression::ArrayAccess { array, index } => {
            let br = v.visit_postfix_expression_mut(array).branch();
            if let ControlFlow::Break(res) = br {
                return V::Result::from_residual(res);
            }
            v.visit_expression_mut(index)
        }
        PostfixExpression::FunctionCall { function, arguments } => {
            let br = v.visit_postfix_expression_mut(function).branch();
            if let ControlFlow::Break(res) = br {
                return V::Result::from_residual(res);
            }
            for a in arguments {
                let br = v.visit_expression_mut(a).branch();
                if let ControlFlow::Break(res) = br {
                    return V::Result::from_residual(res);
                }
            }
            V::Result::output()
        }
        PostfixExpression::MemberAccess { object, member } => {
            let br = v.visit_postfix_expression_mut(object).branch();
            if let ControlFlow::Break(res) = br {
                return V::Result::from_residual(res);
            }
            v.visit_member_name_mut(member)
        }
        PostfixExpression::MemberAccessPtr { object, member } => {
            let br = v.visit_postfix_expression_mut(object).branch();
            if let ControlFlow::Break(res) = br {
                return V::Result::from_residual(res);
            }
            v.visit_member_name_mut(member)
        }
        PostfixExpression::PostIncrement(inner) | PostfixExpression::PostDecrement(inner) => {
            v.visit_postfix_expression_mut(inner)
        }
        PostfixExpression::CompoundLiteral(cl) => v.visit_type_name_mut(&mut cl.type_name),
    }
}

/// Walks a unary expression with mutable access.
pub fn walk_unary_expression_mut<'a, V: VisitorMut<'a> + ?Sized>(v: &mut V, u: &'a mut UnaryExpression) -> V::Result {
    match u {
        UnaryExpression::Postfix(p) => v.visit_postfix_expression_mut(p),
        UnaryExpression::PreIncrement(inner) | UnaryExpression::PreDecrement(inner) => {
            v.visit_unary_expression_mut(inner)
        }
        UnaryExpression::Unary { operand, .. } => v.visit_cast_expression_mut(operand),
        UnaryExpression::Sizeof(inner) => v.visit_unary_expression_mut(inner),
        UnaryExpression::SizeofType(tn) | UnaryExpression::Alignof(tn) => v.visit_type_name_mut(tn),
    }
}

/// Walks a cast expression with mutable access.
pub fn walk_cast_expression_mut<'a, V: VisitorMut<'a> + ?Sized>(v: &mut V, c: &'a mut CastExpression) -> V::Result {
    match c {
        CastExpression::Unary(u) => v.visit_unary_expression_mut(u),
        CastExpression::Cast { type_name, expression } => {
            let br = v.visit_type_name_mut(type_name).branch();
            if let ControlFlow::Break(res) = br {
                return V::Result::from_residual(res);
            }
            v.visit_cast_expression_mut(expression)
        }
    }
}

/// Walks a declaration with mutable access.
pub fn walk_declaration_mut<'a, V: VisitorMut<'a> + ?Sized>(v: &mut V, d: &'a mut Declaration) -> V::Result {
    match d {
        Declaration::Normal { attributes, specifiers, declarators } => {
            for attr in attributes {
                let br = v.visit_attribute_specifier_mut(attr).branch();
                if let ControlFlow::Break(res) = br {
                    return V::Result::from_residual(res);
                }
            }
            let br = v.visit_declaration_specifiers_mut(specifiers).branch();
            if let ControlFlow::Break(res) = br {
                return V::Result::from_residual(res);
            }
            for init in declarators {
                let br = v.visit_declarator_mut(&mut init.declarator).branch();
                if let ControlFlow::Break(res) = br {
                    return V::Result::from_residual(res);
                }
            }
            V::Result::output()
        }
        Declaration::Typedef { attributes, specifiers, declarators } => {
            for attr in attributes {
                let br = v.visit_attribute_specifier_mut(attr).branch();
                if let ControlFlow::Break(res) = br {
                    return V::Result::from_residual(res);
                }
            }
            let br = v.visit_declaration_specifiers_mut(specifiers).branch();
            if let ControlFlow::Break(res) = br {
                return V::Result::from_residual(res);
            }
            for d in declarators {
                let br = v.visit_declarator_mut(d).branch();
                if let ControlFlow::Break(res) = br {
                    return V::Result::from_residual(res);
                }
            }
            V::Result::output()
        }
        Declaration::Attribute(attrs) => {
            for attr in attrs {
                let br = v.visit_attribute_specifier_mut(attr).branch();
                if let ControlFlow::Break(res) = br {
                    return V::Result::from_residual(res);
                }
            }
            V::Result::output()
        }
        Declaration::StaticAssert(_) | Declaration::Error => V::Result::output(),
    }
}

/// Walks declaration specifiers with mutable access.
pub fn walk_declaration_specifiers_mut<'a, V: VisitorMut<'a> + ?Sized>(
    v: &mut V,
    s: &'a mut DeclarationSpecifiers,
) -> V::Result {
    for it in &mut s.specifiers {
        let br = match it {
            DeclarationSpecifier::StorageClass(_) => ControlFlow::Continue(()),
            DeclarationSpecifier::TypeSpecifierQualifier(tsq) => v.visit_type_specifier_qualifier_mut(tsq).branch(),
            DeclarationSpecifier::Function { .. } => ControlFlow::Continue(()),
        };
        if let ControlFlow::Break(res) = br {
            return V::Result::from_residual(res);
        }
    }
    V::Result::output()
}

/// Walks a type specifier or qualifier with mutable access.
pub fn walk_type_specifier_qualifier_mut<'a, V: VisitorMut<'a> + ?Sized>(
    v: &mut V,
    x: &'a mut TypeSpecifierQualifier,
) -> V::Result {
    match x {
        TypeSpecifierQualifier::TypeSpecifier(ts) => v.visit_type_specifier_mut(ts),
        TypeSpecifierQualifier::TypeQualifier(_) => V::Result::output(),
        TypeSpecifierQualifier::AlignmentSpecifier(a) => match a {
            AlignmentSpecifier::Type(tn) => v.visit_type_name_mut(tn),
            AlignmentSpecifier::Expression(_) => V::Result::output(),
        },
    }
}

/// Walks a type specifier with mutable access.
pub fn walk_type_specifier_mut<'a, V: VisitorMut<'a> + ?Sized>(v: &mut V, ts: &'a mut TypeSpecifier) -> V::Result {
    match ts {
        TypeSpecifier::Struct(s) => {
            if let Some(id) = &mut s.identifier {
                let br = v.visit_struct_name_mut(id).branch();
                if let ControlFlow::Break(res) = br {
                    return V::Result::from_residual(res);
                }
            }
            if let Some(members) = &mut s.members {
                for member in members {
                    let br = match member {
                        MemberDeclaration::Normal { specifiers, declarators, .. } => {
                            let br = v.visit_specifier_qualifier_list_mut(specifiers).branch();
                            if let ControlFlow::Break(res) = br {
                                return V::Result::from_residual(res);
                            }
                            for d in declarators {
                                let br = match d {
                                    MemberDeclarator::Declarator(dd) => v.visit_declarator_mut(dd).branch(),
                                    MemberDeclarator::BitField { declarator, .. } => {
                                        if let Some(dd) = declarator {
                                            v.visit_declarator_mut(dd).branch()
                                        } else {
                                            ControlFlow::Continue(())
                                        }
                                    }
                                };
                                if let ControlFlow::Break(res) = br {
                                    return V::Result::from_residual(res);
                                }
                            }
                            ControlFlow::Continue(())
                        }
                        MemberDeclaration::StaticAssert(_) => ControlFlow::Continue(()),
                        MemberDeclaration::Error => ControlFlow::Continue(()),
                    };
                    if let ControlFlow::Break(res) = br {
                        return V::Result::from_residual(res);
                    }
                }
            }
            V::Result::output()
        }
        TypeSpecifier::Enum(e) => {
            if let Some(id) = &mut e.identifier {
                let br = v.visit_enum_name_mut(id).branch();
                if let ControlFlow::Break(res) = br {
                    return V::Result::from_residual(res);
                }
            }
            if let Some(enumerators) = &mut e.enumerators {
                for enumerator in enumerators {
                    let br = v.visit_enumerator_name_mut(&mut enumerator.name).branch();
                    if let ControlFlow::Break(res) = br {
                        return V::Result::from_residual(res);
                    }
                }
            }
            V::Result::output()
        }
        TypeSpecifier::TypedefName(id) => v.visit_type_name_identifier_mut(id),
        TypeSpecifier::Atomic(a) => v.visit_atomic_type_specifier_mut(a),
        TypeSpecifier::Typeof(t) => v.visit_typeof_mut(t),
        TypeSpecifier::BitInt(_)
        | TypeSpecifier::Void
        | TypeSpecifier::Char
        | TypeSpecifier::Short
        | TypeSpecifier::Int
        | TypeSpecifier::Long
        | TypeSpecifier::Float
        | TypeSpecifier::Double
        | TypeSpecifier::Signed
        | TypeSpecifier::Unsigned
        | TypeSpecifier::Bool
        | TypeSpecifier::Complex
        | TypeSpecifier::Decimal32
        | TypeSpecifier::Decimal64
        | TypeSpecifier::Decimal128 => V::Result::output(),
    }
}

/// Walks an atomic type specifier with mutable access.
pub fn walk_atomic_type_specifier_mut<'a, V: VisitorMut<'a> + ?Sized>(
    v: &mut V,
    a: &'a mut AtomicTypeSpecifier,
) -> V::Result {
    v.visit_type_name_mut(&mut a.type_name)
}

/// Walks a typeof specifier with mutable access.
pub fn walk_typeof_mut<'a, V: VisitorMut<'a> + ?Sized>(v: &mut V, t: &'a mut TypeofSpecifier) -> V::Result {
    match t {
        TypeofSpecifier::Typeof(arg) | TypeofSpecifier::TypeofUnqual(arg) => match arg {
            TypeofSpecifierArgument::Expression(e) => v.visit_expression_mut(e),
            TypeofSpecifierArgument::TypeName(tn) => v.visit_type_name_mut(tn),
            TypeofSpecifierArgument::Error => V::Result::output(),
        },
    }
}

/// Walks a specifier qualifier list with mutable access.
pub fn walk_specifier_qualifier_list_mut<'a, V: VisitorMut<'a> + ?Sized>(
    v: &mut V,
    s: &'a mut SpecifierQualifierList,
) -> V::Result {
    for item in &mut s.items {
        let br = v.visit_type_specifier_qualifier_mut(item).branch();
        if let ControlFlow::Break(res) = br {
            return V::Result::from_residual(res);
        }
    }
    V::Result::output()
}

/// Walks a declarator with mutable access.
pub fn walk_declarator_mut<'a, V: VisitorMut<'a> + ?Sized>(v: &mut V, d: &'a mut Declarator) -> V::Result {
    match d {
        Declarator::Direct(dd) => v.visit_direct_declarator_mut(dd),
        Declarator::Pointer { declarator, .. } => v.visit_declarator_mut(declarator),
        Declarator::Error => V::Result::output(),
    }
}

/// Walks a direct declarator with mutable access.
pub fn walk_direct_declarator_mut<'a, V: VisitorMut<'a> + ?Sized>(v: &mut V, d: &'a mut DirectDeclarator) -> V::Result {
    match d {
        DirectDeclarator::Identifier { identifier, attributes } => {
            let br = v.visit_variable_name_mut(identifier).branch();
            if let ControlFlow::Break(res) = br {
                return V::Result::from_residual(res);
            }
            for attr in attributes {
                let br = v.visit_attribute_specifier_mut(attr).branch();
                if let ControlFlow::Break(res) = br {
                    return V::Result::from_residual(res);
                }
            }
            V::Result::output()
        }
        DirectDeclarator::Parenthesized(inner) => v.visit_declarator_mut(inner),
        DirectDeclarator::Array { declarator, attributes, .. } => {
            let br = v.visit_direct_declarator_mut(declarator).branch();
            if let ControlFlow::Break(res) = br {
                return V::Result::from_residual(res);
            }
            for attr in attributes {
                let br = v.visit_attribute_specifier_mut(attr).branch();
                if let ControlFlow::Break(res) = br {
                    return V::Result::from_residual(res);
                }
            }
            V::Result::output()
        }
        DirectDeclarator::Function { declarator, attributes, parameters } => {
            let br = v.visit_direct_declarator_mut(declarator).branch();
            if let ControlFlow::Break(res) = br {
                return V::Result::from_residual(res);
            }
            for attr in attributes {
                let br = v.visit_attribute_specifier_mut(attr).branch();
                if let ControlFlow::Break(res) = br {
                    return V::Result::from_residual(res);
                }
            }
            match parameters {
                ParameterTypeList::Parameters(params) | ParameterTypeList::Variadic(params) => {
                    for param in params {
                        let br = v.visit_declaration_specifiers_mut(&mut param.specifiers).branch();
                        if let ControlFlow::Break(res) = br {
                            return V::Result::from_residual(res);
                        }
                        if let Some(kind) = &mut param.declarator {
                            let br = match kind {
                                ParameterDeclarationKind::Declarator(d) => v.visit_declarator_mut(d).branch(),
                                ParameterDeclarationKind::Abstract(a) => v.visit_abstract_declarator_mut(a).branch(),
                            };
                            if let ControlFlow::Break(res) = br {
                                return V::Result::from_residual(res);
                            }
                        }
                    }
                }
                ParameterTypeList::OnlyVariadic => {}
            }
            V::Result::output()
        }
    }
}

/// Walks a type name with mutable access.
pub fn walk_type_name_mut<'a, V: VisitorMut<'a> + ?Sized>(v: &mut V, tn: &'a mut TypeName) -> V::Result {
    match tn {
        TypeName::TypeName { specifiers, abstract_declarator } => {
            let br = v.visit_specifier_qualifier_list_mut(specifiers).branch();
            if let ControlFlow::Break(res) = br {
                return V::Result::from_residual(res);
            }
            if let Some(ad) = abstract_declarator {
                v.visit_abstract_declarator_mut(ad)
            } else {
                V::Result::output()
            }
        }
        TypeName::Error => V::Result::output(),
    }
}

/// Walks an abstract declarator with mutable access.
pub fn walk_abstract_declarator_mut<'a, V: VisitorMut<'a> + ?Sized>(
    v: &mut V,
    a: &'a mut AbstractDeclarator,
) -> V::Result {
    match a {
        AbstractDeclarator::Direct(d) => v.visit_direct_abstract_declarator_mut(d),
        AbstractDeclarator::Pointer { abstract_declarator, .. } => {
            if let Some(ad) = abstract_declarator {
                v.visit_abstract_declarator_mut(ad)
            } else {
                V::Result::output()
            }
        }
        AbstractDeclarator::Error => V::Result::output(),
    }
}

/// Walks a direct abstract declarator with mutable access.
pub fn walk_direct_abstract_declarator_mut<'a, V: VisitorMut<'a> + ?Sized>(
    v: &mut V,
    d: &'a mut DirectAbstractDeclarator,
) -> V::Result {
    match d {
        DirectAbstractDeclarator::Parenthesized(ad) => v.visit_abstract_declarator_mut(ad),
        DirectAbstractDeclarator::Array { declarator, .. } => {
            if let Some(dd) = declarator {
                v.visit_direct_abstract_declarator_mut(dd)
            } else {
                V::Result::output()
            }
        }
        DirectAbstractDeclarator::Function { declarator, .. } => {
            if let Some(dd) = declarator {
                v.visit_direct_abstract_declarator_mut(dd)
            } else {
                V::Result::output()
            }
        }
    }
}

/// Walks an attribute specifier with mutable access.
pub fn walk_attribute_specifier_mut<'a, V: VisitorMut<'a> + ?Sized>(
    v: &mut V,
    a: &'a mut AttributeSpecifier,
) -> V::Result {
    match a {
        AttributeSpecifier::Attributes(attributes) => {
            for attr in attributes {
                let br = v.visit_attribute_mut(attr).branch();
                if let ControlFlow::Break(res) = br {
                    return V::Result::from_residual(res);
                }
            }
        }
        AttributeSpecifier::Asm(string_literals) => {
            let br = v.visit_asm_attribute_specifier_mut(string_literals).branch();
            if let ControlFlow::Break(res) = br {
                return V::Result::from_residual(res);
            }
        }
        AttributeSpecifier::Error => {}
    }
    V::Result::output()
}
