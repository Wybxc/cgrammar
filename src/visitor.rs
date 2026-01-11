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
    /// operators, or in designators of initializers.
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

    /// Visits an initializer.
    fn visit_initializer(&mut self, i: &'a Initializer) -> Self::Result {
        walk_initializer(self, i)
    }

    /// Visits a braced initializer.
    fn visit_braced_initializer(&mut self, b: &'a BracedInitializer) -> Self::Result {
        walk_braced_initializer(self, b)
    }

    /// Visits a designated initializer.
    fn visit_designated_initializer(&mut self, d: &'a DesignatedInitializer) -> Self::Result {
        walk_designated_initializer(self, d)
    }

    /// Visits a designation.
    fn visit_designation(&mut self, d: &'a Designation) -> Self::Result {
        walk_designation(self, d)
    }

    /// Visits a designator.
    fn visit_designator(&mut self, d: &'a Designator) -> Self::Result {
        walk_designator(self, d)
    }

    /// Visits a constant expression.
    fn visit_constant_expression(&mut self, c: &'a ConstantExpression) -> Self::Result {
        walk_constant_expression(self, c)
    }

    /// Visits a postfix expression.
    fn visit_postfix_expression(&mut self, p: &'a PostfixExpression) -> Self::Result {
        walk_postfix_expression(self, p)
    }

    /// Visits a primary expression.
    fn visit_primary_expression(&mut self, p: &'a PrimaryExpression) -> Self::Result {
        walk_primary_expression(self, p)
    }

    /// Visits a generic selection.
    fn visit_generic_selection(&mut self, g: &'a GenericSelection) -> Self::Result {
        walk_generic_selection(self, g)
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

    /// Visits a labeled statement.
    fn visit_labeled_statement(&mut self, ls: &'a LabeledStatement) -> Self::Result {
        walk_labeled_statement(self, ls)
    }

    /// Visits a label.
    fn visit_label(&mut self, l: &'a Label) -> Self::Result {
        walk_label(self, l)
    }

    /// Visits an expression statement.
    fn visit_expression_statement(&mut self, e: &'a ExpressionStatement) -> Self::Result {
        walk_expression_statement(self, e)
    }

    /// Visits a primary block.
    fn visit_primary_block(&mut self, pb: &'a PrimaryBlock) -> Self::Result {
        walk_primary_block(self, pb)
    }

    /// Visits a jump statement.
    fn visit_jump_statement(&mut self, j: &'a JumpStatement) -> Self::Result {
        walk_jump_statement(self, j)
    }

    /// Visits a selection statement.
    fn visit_selection_statement(&mut self, s: &'a SelectionStatement) -> Self::Result {
        walk_selection_statement(self, s)
    }

    /// Visits an iteration statement.
    fn visit_iteration_statement(&mut self, i: &'a IterationStatement) -> Self::Result {
        walk_iteration_statement(self, i)
    }

    /// Visits a for init clause.
    fn visit_for_init(&mut self, fi: &'a ForInit) -> Self::Result {
        walk_for_init(self, fi)
    }

    /// Visits a binary expression.
    fn visit_binary_expression(&mut self, b: &'a BinaryExpression) -> Self::Result {
        walk_binary_expression(self, b)
    }

    /// Visits a conditional expression.
    fn visit_conditional_expression(&mut self, c: &'a ConditionalExpression) -> Self::Result {
        walk_conditional_expression(self, c)
    }

    /// Visits an assignment expression.
    fn visit_assignment_expression(&mut self, a: &'a AssignmentExpression) -> Self::Result {
        walk_assignment_expression(self, a)
    }

    /// Visits a comma expression.
    fn visit_comma_expression(&mut self, c: &'a CommaExpression) -> Self::Result {
        walk_comma_expression(self, c)
    }

    /// Visits a binary operator.
    fn visit_binary_operator(&mut self, _: &'a BinaryOperator) -> Self::Result {
        Self::Result::output()
    }

    /// Visits an assignment operator.
    fn visit_assignment_operator(&mut self, _: &'a AssignmentOperator) -> Self::Result {
        Self::Result::output()
    }

    /// Visits a compound literal.
    fn visit_compound_literal(&mut self, cl: &'a CompoundLiteral) -> Self::Result {
        walk_compound_literal(self, cl)
    }

    /// Visits a storage class specifier.
    fn visit_storage_class_specifier(&mut self, _: &'a StorageClassSpecifier) -> Self::Result {
        Self::Result::output()
    }

    /// Visits a unary operator.
    fn visit_unary_operator(&mut self, _: &'a UnaryOperator) -> Self::Result {
        Self::Result::output()
    }

    /// Visits an init declarator.
    fn visit_init_declarator(&mut self, id: &'a InitDeclarator) -> Self::Result {
        walk_init_declarator(self, id)
    }

    /// Visits a static assert declaration.
    fn visit_static_assert_declaration(&mut self, sa: &'a StaticAssertDeclaration) -> Self::Result {
        walk_static_assert_declaration(self, sa)
    }

    /// Visits a static assert message.
    fn visit_static_assert_message(&mut self, _: &'a StringLiterals) -> Self::Result {
        Self::Result::output()
    }

    /// Visits a declaration specifier.
    fn visit_declaration_specifier(&mut self, ds: &'a DeclarationSpecifier) -> Self::Result {
        walk_declaration_specifier(self, ds)
    }

    /// Visits a function specifier.
    fn visit_function_specifier(&mut self, _: &'a FunctionSpecifier) -> Self::Result {
        Self::Result::output()
    }

    /// Visits a type qualifier.
    fn visit_type_qualifier(&mut self, _: &'a TypeQualifier) -> Self::Result {
        Self::Result::output()
    }

    /// Visits an alignment specifier.
    fn visit_alignment_specifier(&mut self, a: &'a AlignmentSpecifier) -> Self::Result {
        walk_alignment_specifier(self, a)
    }

    /// Visits a struct or union specifier.
    fn visit_struct_union_specifier(&mut self, s: &'a StructOrUnionSpecifier) -> Self::Result {
        walk_struct_union_specifier(self, s)
    }

    /// Visits an enum specifier.
    fn visit_enum_specifier(&mut self, e: &'a EnumSpecifier) -> Self::Result {
        walk_enum_specifier(self, e)
    }

    /// Visits a struct or union keyword.
    fn visit_struct_or_union(&mut self, _: &'a StructOrUnion) -> Self::Result {
        Self::Result::output()
    }

    /// Visits a member declaration.
    fn visit_member_declaration(&mut self, md: &'a MemberDeclaration) -> Self::Result {
        walk_member_declaration(self, md)
    }

    /// Visits a member declarator.
    fn visit_member_declarator(&mut self, md: &'a MemberDeclarator) -> Self::Result {
        walk_member_declarator(self, md)
    }

    /// Visits an enumerator.
    fn visit_enumerator(&mut self, e: &'a Enumerator) -> Self::Result {
        walk_enumerator(self, e)
    }

    /// Visits a pointer.
    fn visit_pointer(&mut self, p: &'a Pointer) -> Self::Result {
        walk_pointer(self, p)
    }

    /// Visits a pointer or block indicator.
    fn visit_pointer_or_block(&mut self, _: &'a PointerOrBlock) -> Self::Result {
        Self::Result::output()
    }

    /// Visits an array declarator.
    fn visit_array_declarator(&mut self, _: &'a ArrayDeclarator) -> Self::Result {
        Self::Result::output()
    }

    /// Visits a parameter type list.
    fn visit_parameter_type_list(&mut self, ptl: &'a ParameterTypeList) -> Self::Result {
        walk_parameter_type_list(self, ptl)
    }

    /// Visits a parameter declaration.
    fn visit_parameter_declaration(&mut self, pd: &'a ParameterDeclaration) -> Self::Result {
        walk_parameter_declaration(self, pd)
    }

    /// Visits a parameter declaration kind.
    fn visit_parameter_declaration_kind(&mut self, pdk: &'a ParameterDeclarationKind) -> Self::Result {
        walk_parameter_declaration_kind(self, pdk)
    }
}

macro_rules! tr {
    ($e:expr) => {{
        let br = $e.branch();
        if let ControlFlow::Break(res) = br {
            return V::Result::from_residual(res);
        }
    }};
}

/// Walk a translation unit.
pub fn walk_translation_unit<'a, V: Visitor<'a> + ?Sized>(v: &mut V, tu: &'a TranslationUnit) -> V::Result {
    for ed in &tu.external_declarations {
        tr!(v.visit_external_declaration(ed));
    }
    V::Result::output()
}

/// Walk an external declaration.
pub fn walk_external_declaration<'a, V: Visitor<'a> + ?Sized>(v: &mut V, d: &'a ExternalDeclaration) -> V::Result {
    match d {
        ExternalDeclaration::Function(f) => v.visit_function_definition(f),
        ExternalDeclaration::Declaration(d) => v.visit_declaration(d),
    }
}

/// Walk a function definition.
pub fn walk_function_definition<'a, V: Visitor<'a> + ?Sized>(v: &mut V, f: &'a FunctionDefinition) -> V::Result {
    for attr in &f.attributes {
        tr!(v.visit_attribute_specifier(attr));
    }
    tr!(v.visit_declaration_specifiers(&f.specifiers));
    tr!(v.visit_declarator(&f.declarator));
    v.visit_compound_statement(&f.body)
}

/// Walk a statement.
pub fn walk_statement<'a, V: Visitor<'a> + ?Sized>(v: &mut V, s: &'a Statement) -> V::Result {
    match s {
        Statement::Labeled(ls) => v.visit_labeled_statement(ls),
        Statement::Unlabeled(u) => v.visit_unlabeled_statement(u),
    }
}

/// Walk a labeled statement.
pub fn walk_labeled_statement<'a, V: Visitor<'a> + ?Sized>(v: &mut V, ls: &'a LabeledStatement) -> V::Result {
    tr!(v.visit_label(&ls.label));
    v.visit_statement(&ls.statement)
}

/// Walk a label.
pub fn walk_label<'a, V: Visitor<'a> + ?Sized>(v: &mut V, l: &'a Label) -> V::Result {
    match l {
        Label::Identifier { attributes, identifier } => {
            for attribute in attributes {
                tr!(v.visit_attribute_specifier(attribute));
            }
            v.visit_label_name(identifier)
        }
        Label::Case { attributes, expression } => {
            for attribute in attributes {
                tr!(v.visit_attribute_specifier(attribute));
            }
            v.visit_constant_expression(expression)
        }
        Label::Default { attributes } => {
            for attribute in attributes {
                tr!(v.visit_attribute_specifier(attribute));
            }
            V::Result::output()
        }
    }
}

/// Walk an unlabeled statement.
pub fn walk_unlabeled_statement<'a, V: Visitor<'a> + ?Sized>(v: &mut V, s: &'a UnlabeledStatement) -> V::Result {
    match s {
        UnlabeledStatement::Expression(es) => v.visit_expression_statement(es),
        UnlabeledStatement::Primary { attributes, block } => {
            for attribute in attributes {
                tr!(v.visit_attribute_specifier(attribute));
            }
            v.visit_primary_block(block)
        }
        UnlabeledStatement::Jump { attributes, statement } => {
            for attribute in attributes {
                tr!(v.visit_attribute_specifier(attribute));
            }
            v.visit_jump_statement(statement)
        }
    }
}

/// Walk an expression statement.
pub fn walk_expression_statement<'a, V: Visitor<'a> + ?Sized>(v: &mut V, e: &'a ExpressionStatement) -> V::Result {
    if let Some(expr) = &e.expression {
        tr!(v.visit_expression(expr));
    }
    V::Result::output()
}

/// Walk a primary block.
pub fn walk_primary_block<'a, V: Visitor<'a> + ?Sized>(v: &mut V, pb: &'a PrimaryBlock) -> V::Result {
    match pb {
        PrimaryBlock::Compound(c) => v.visit_compound_statement(c),
        PrimaryBlock::Selection(sel) => v.visit_selection_statement(sel),
        PrimaryBlock::Iteration(iter) => v.visit_iteration_statement(iter),
    }
}

/// Walk a compound statement.
pub fn walk_compound_statement<'a, V: Visitor<'a> + ?Sized>(v: &mut V, c: &'a CompoundStatement) -> V::Result {
    for item in &c.items {
        match item {
            BlockItem::Declaration(d) => tr!(v.visit_declaration(d)),
            BlockItem::Statement(u) => tr!(v.visit_unlabeled_statement(u)),
            BlockItem::Label(l) => tr!(v.visit_label(l)),
        }
    }
    V::Result::output()
}

/// Walk a selection statement.
pub fn walk_selection_statement<'a, V: Visitor<'a> + ?Sized>(v: &mut V, s: &'a SelectionStatement) -> V::Result {
    match s {
        SelectionStatement::If { condition, then_stmt, else_stmt } => {
            tr!(v.visit_expression(condition));
            tr!(v.visit_statement(then_stmt));
            if let Some(e) = else_stmt {
                tr!(v.visit_statement(e));
            }
            V::Result::output()
        }
        SelectionStatement::Switch { expression, statement } => {
            tr!(v.visit_expression(expression));
            v.visit_statement(statement)
        }
    }
}

/// Walk an iteration statement.
pub fn walk_iteration_statement<'a, V: Visitor<'a> + ?Sized>(v: &mut V, i: &'a IterationStatement) -> V::Result {
    match i {
        IterationStatement::While { condition, body } => {
            tr!(v.visit_expression(condition));
            v.visit_statement(body)
        }
        IterationStatement::DoWhile { body, condition } => {
            tr!(v.visit_statement(body));
            v.visit_expression(condition)
        }
        IterationStatement::For { init, condition, update, body } => {
            if let Some(i) = init {
                v.visit_for_init(i);
            }
            if let Some(c) = condition {
                tr!(v.visit_expression(c));
            }
            if let Some(u) = update {
                tr!(v.visit_expression(u));
            }
            v.visit_statement(body)
        }
        IterationStatement::Error => V::Result::output(),
    }
}

/// Walk a for init.
pub fn walk_for_init<'a, V: Visitor<'a> + ?Sized>(v: &mut V, fi: &'a ForInit) -> V::Result {
    match fi {
        ForInit::Expression(e) => v.visit_expression(e),
        ForInit::Declaration(d) => v.visit_declaration(d),
    }
}

/// Walk a jump statement.
pub fn walk_jump_statement<'a, V: Visitor<'a> + ?Sized>(v: &mut V, j: &'a JumpStatement) -> V::Result {
    match j {
        JumpStatement::Goto(id) => v.visit_label_name(id),
        JumpStatement::Continue | JumpStatement::Break => V::Result::output(),
        JumpStatement::Return(expr) => {
            if let Some(e) = expr {
                tr!(v.visit_expression(e));
            }
            V::Result::output()
        }
    }
}

/// Walk an expression.
pub fn walk_expression<'a, V: Visitor<'a> + ?Sized>(v: &mut V, e: &'a Expression) -> V::Result {
    match e {
        Expression::Postfix(p) => v.visit_postfix_expression(p),
        Expression::Unary(u) => v.visit_unary_expression(u),
        Expression::Cast(c) => v.visit_cast_expression(c),
        Expression::Binary(b) => v.visit_binary_expression(b),
        Expression::Conditional(c) => v.visit_conditional_expression(c),
        Expression::Assignment(a) => v.visit_assignment_expression(a),
        Expression::Comma(c) => v.visit_comma_expression(c),
        Expression::Error => V::Result::output(),
    }
}

/// Walk a binary expression.
pub fn walk_binary_expression<'a, V: Visitor<'a> + ?Sized>(v: &mut V, b: &'a BinaryExpression) -> V::Result {
    tr!(v.visit_expression(&b.left));
    tr!(v.visit_binary_operator(&b.operator));
    v.visit_expression(&b.right)
}

/// Walk a conditional expression.
pub fn walk_conditional_expression<'a, V: Visitor<'a> + ?Sized>(v: &mut V, c: &'a ConditionalExpression) -> V::Result {
    tr!(v.visit_expression(&c.condition));
    tr!(v.visit_expression(&c.then_expr));
    v.visit_expression(&c.else_expr)
}

/// Walk an assignment expression.
pub fn walk_assignment_expression<'a, V: Visitor<'a> + ?Sized>(v: &mut V, a: &'a AssignmentExpression) -> V::Result {
    tr!(v.visit_expression(&a.left));
    tr!(v.visit_assignment_operator(&a.operator));
    v.visit_expression(&a.right)
}

/// Walk a comma expression.
pub fn walk_comma_expression<'a, V: Visitor<'a> + ?Sized>(v: &mut V, c: &'a CommaExpression) -> V::Result {
    for e in &c.expressions {
        tr!(v.visit_expression(e));
    }
    V::Result::output()
}

/// Walk a postfix expression.
pub fn walk_postfix_expression<'a, V: Visitor<'a> + ?Sized>(v: &mut V, p: &'a PostfixExpression) -> V::Result {
    match p {
        PostfixExpression::Primary(pr) => v.visit_primary_expression(pr),
        PostfixExpression::ArrayAccess { array, index } => {
            tr!(v.visit_postfix_expression(array));
            v.visit_expression(index)
        }
        PostfixExpression::FunctionCall { function, arguments } => {
            tr!(v.visit_postfix_expression(function));
            for a in arguments {
                tr!(v.visit_expression(a));
            }
            V::Result::output()
        }
        PostfixExpression::MemberAccess { object, member } => {
            tr!(v.visit_postfix_expression(object));
            v.visit_member_name(member)
        }
        PostfixExpression::MemberAccessPtr { object, member } => {
            tr!(v.visit_postfix_expression(object));
            v.visit_member_name(member)
        }
        PostfixExpression::PostIncrement(inner) | PostfixExpression::PostDecrement(inner) => {
            v.visit_postfix_expression(inner)
        }
        PostfixExpression::CompoundLiteral(cl) => v.visit_compound_literal(cl),
    }
}

/// Walk a compound literal.
pub fn walk_compound_literal<'a, V: Visitor<'a> + ?Sized>(v: &mut V, cl: &'a CompoundLiteral) -> V::Result {
    for specifier in &cl.storage_class_specifiers {
        tr!(v.visit_storage_class_specifier(specifier));
    }
    tr!(v.visit_type_name(&cl.type_name));
    v.visit_braced_initializer(&cl.initializer)
}

/// Walk a primary expression.
fn walk_primary_expression<'a, V: Visitor<'a> + ?Sized>(v: &mut V, pr: &'a PrimaryExpression) -> V::Result {
    match pr {
        PrimaryExpression::Identifier(id) => v.visit_variable_name(id),
        PrimaryExpression::EnumerationConstant(id) => v.visit_enum_constant(id),
        PrimaryExpression::Parenthesized(e) => v.visit_expression(e),
        PrimaryExpression::Generic(g) => v.visit_generic_selection(g),
        _ => V::Result::output(),
    }
}

/// Walk a generic selection.
fn walk_generic_selection<'a, V: Visitor<'a> + ?Sized>(v: &mut V, g: &'a GenericSelection) -> V::Result {
    tr!(v.visit_expression(&g.controlling_expression));
    for assoc in &g.associations {
        match assoc {
            GenericAssociation::Type { type_name, expression } => {
                tr!(v.visit_type_name(type_name));
                tr!(v.visit_expression(expression))
            }
            GenericAssociation::Default { expression } => tr!(v.visit_expression(expression)),
        };
    }
    V::Result::output()
}

/// Walk a unary expression.
pub fn walk_unary_expression<'a, V: Visitor<'a> + ?Sized>(v: &mut V, u: &'a UnaryExpression) -> V::Result {
    match u {
        UnaryExpression::Postfix(p) => v.visit_postfix_expression(p),
        UnaryExpression::PreIncrement(inner) | UnaryExpression::PreDecrement(inner) => v.visit_unary_expression(inner),
        UnaryExpression::Unary { operator, operand } => {
            tr!(v.visit_unary_operator(operator));
            v.visit_cast_expression(operand)
        }
        UnaryExpression::Sizeof(inner) => v.visit_unary_expression(inner),
        UnaryExpression::SizeofType(tn) | UnaryExpression::Alignof(tn) => v.visit_type_name(tn),
    }
}

/// Walk a cast expression.
pub fn walk_cast_expression<'a, V: Visitor<'a> + ?Sized>(v: &mut V, c: &'a CastExpression) -> V::Result {
    match c {
        CastExpression::Unary(u) => v.visit_unary_expression(u),
        CastExpression::Cast { type_name, expression } => {
            tr!(v.visit_type_name(type_name));
            v.visit_cast_expression(expression)
        }
    }
}

/// Walk a declaration.
pub fn walk_declaration<'a, V: Visitor<'a> + ?Sized>(v: &mut V, d: &'a Declaration) -> V::Result {
    match d {
        Declaration::Normal { attributes, specifiers, declarators } => {
            for attr in attributes {
                tr!(v.visit_attribute_specifier(attr));
            }
            tr!(v.visit_declaration_specifiers(specifiers));
            for init in declarators {
                tr!(v.visit_init_declarator(init));
            }
            V::Result::output()
        }
        Declaration::Typedef { attributes, specifiers, declarators } => {
            for attr in attributes {
                tr!(v.visit_attribute_specifier(attr));
            }
            tr!(v.visit_declaration_specifiers(specifiers));
            for d in declarators {
                tr!(v.visit_declarator(d));
            }
            V::Result::output()
        }
        Declaration::StaticAssert(sa) => v.visit_static_assert_declaration(sa),
        Declaration::Attribute(attrs) => {
            for attr in attrs {
                tr!(v.visit_attribute_specifier(attr));
            }
            V::Result::output()
        }
        Declaration::Error => V::Result::output(),
    }
}

/// Walk an init declarator.
pub fn walk_init_declarator<'a, V: Visitor<'a> + ?Sized>(v: &mut V, id: &'a InitDeclarator) -> V::Result {
    tr!(v.visit_declarator(&id.declarator));
    if let Some(init) = &id.initializer {
        tr!(v.visit_initializer(init));
    }
    V::Result::output()
}

/// Walk a static assert declaration.
pub fn walk_static_assert_declaration<'a, V: Visitor<'a> + ?Sized>(
    v: &mut V,
    sa: &'a StaticAssertDeclaration,
) -> V::Result {
    tr!(v.visit_constant_expression(&sa.condition));
    if let Some(msg) = &sa.message {
        tr!(v.visit_static_assert_message(msg));
    }
    V::Result::output()
}

/// Walk declaration specifiers.
pub fn walk_declaration_specifiers<'a, V: Visitor<'a> + ?Sized>(v: &mut V, s: &'a DeclarationSpecifiers) -> V::Result {
    for it in &s.specifiers {
        tr!(v.visit_declaration_specifier(it));
    }
    for attr in &s.attributes {
        tr!(v.visit_attribute_specifier(attr));
    }
    V::Result::output()
}

/// Walk a declaration specifier.
pub fn walk_declaration_specifier<'a, V: Visitor<'a> + ?Sized>(v: &mut V, ds: &'a DeclarationSpecifier) -> V::Result {
    match ds {
        DeclarationSpecifier::StorageClass(scs) => v.visit_storage_class_specifier(scs),
        DeclarationSpecifier::TypeSpecifierQualifier(tsq) => v.visit_type_specifier_qualifier(tsq),
        DeclarationSpecifier::Function(fs) => v.visit_function_specifier(fs),
    }
}

/// Walk a type specifier or qualifier.
pub fn walk_type_specifier_qualifier<'a, V: Visitor<'a> + ?Sized>(
    v: &mut V,
    x: &'a TypeSpecifierQualifier,
) -> V::Result {
    match x {
        TypeSpecifierQualifier::TypeSpecifier(ts) => v.visit_type_specifier(ts),
        TypeSpecifierQualifier::TypeQualifier(tq) => v.visit_type_qualifier(tq),
        TypeSpecifierQualifier::AlignmentSpecifier(a) => v.visit_alignment_specifier(a),
    }
}

/// Walk an alignment specifier.
pub fn walk_alignment_specifier<'a, V: Visitor<'a> + ?Sized>(v: &mut V, a: &'a AlignmentSpecifier) -> V::Result {
    match a {
        AlignmentSpecifier::Type(tn) => v.visit_type_name(tn),
        AlignmentSpecifier::Expression(e) => v.visit_constant_expression(e),
    }
}

/// Walk a type specifier.
pub fn walk_type_specifier<'a, V: Visitor<'a> + ?Sized>(v: &mut V, ts: &'a TypeSpecifier) -> V::Result {
    match ts {
        TypeSpecifier::Struct(s) => v.visit_struct_union_specifier(s),
        TypeSpecifier::Enum(e) => v.visit_enum_specifier(e),
        TypeSpecifier::TypedefName(id) => v.visit_type_name_identifier(id),
        TypeSpecifier::Atomic(a) => v.visit_atomic_type_specifier(a),
        TypeSpecifier::Typeof(t) => v.visit_typeof(t),
        TypeSpecifier::BitInt(c) => v.visit_constant_expression(c),
        TypeSpecifier::Void
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

/// Walk a struct or union specifier.
pub fn walk_struct_union_specifier<'a, V: Visitor<'a> + ?Sized>(v: &mut V, s: &'a StructOrUnionSpecifier) -> V::Result {
    tr!(v.visit_struct_or_union(&s.kind));
    for attr in &s.attributes {
        tr!(v.visit_attribute_specifier(attr));
    }
    if let Some(id) = &s.identifier {
        tr!(v.visit_struct_name(id));
    }
    if let Some(members) = &s.members {
        for m in members {
            tr!(v.visit_member_declaration(m));
        }
    }
    V::Result::output()
}

/// Walk a member declaration.
pub fn walk_member_declaration<'a, V: Visitor<'a> + ?Sized>(v: &mut V, md: &'a MemberDeclaration) -> V::Result {
    match md {
        MemberDeclaration::Normal { attributes, specifiers, declarators } => {
            for attr in attributes {
                tr!(v.visit_attribute_specifier(attr));
            }
            tr!(v.visit_specifier_qualifier_list(specifiers));
            for d in declarators {
                tr!(v.visit_member_declarator(d));
            }
            V::Result::output()
        }
        MemberDeclaration::StaticAssert(sa) => v.visit_static_assert_declaration(sa),
        MemberDeclaration::Error => V::Result::output(),
    }
}

/// Walk a member declarator.
pub fn walk_member_declarator<'a, V: Visitor<'a> + ?Sized>(v: &mut V, md: &'a MemberDeclarator) -> V::Result {
    match md {
        MemberDeclarator::Declarator(d) => v.visit_declarator(d),
        MemberDeclarator::BitField { declarator, width } => {
            if let Some(d) = declarator {
                tr!(v.visit_declarator(d));
            }
            v.visit_constant_expression(width)
        }
    }
}

/// Walk an enum specifier.
pub fn walk_enum_specifier<'a, V: Visitor<'a> + ?Sized>(v: &mut V, e: &'a EnumSpecifier) -> V::Result {
    for attr in &e.attributes {
        tr!(v.visit_attribute_specifier(attr));
    }
    if let Some(id) = &e.identifier {
        tr!(v.visit_enum_name(id));
    }
    if let Some(sql) = &e.type_specifier {
        tr!(v.visit_specifier_qualifier_list(sql));
    }
    if let Some(enumerators) = &e.enumerators {
        for en in enumerators {
            tr!(v.visit_enumerator(en));
        }
    }
    V::Result::output()
}

/// Walk an enumerator.
pub fn walk_enumerator<'a, V: Visitor<'a> + ?Sized>(v: &mut V, e: &'a Enumerator) -> V::Result {
    tr!(v.visit_enumerator_name(&e.name));
    for attr in &e.attributes {
        tr!(v.visit_attribute_specifier(attr));
    }
    if let Some(value) = &e.value {
        tr!(v.visit_constant_expression(value));
    }
    V::Result::output()
}

/// Walk an atomic type specifier.
pub fn walk_atomic_type_specifier<'a, V: Visitor<'a> + ?Sized>(v: &mut V, a: &'a AtomicTypeSpecifier) -> V::Result {
    v.visit_type_name(&a.type_name)
}

/// Walk a typeof specifier.
pub fn walk_typeof<'a, V: Visitor<'a> + ?Sized>(v: &mut V, t: &'a TypeofSpecifier) -> V::Result {
    match t {
        TypeofSpecifier::Typeof(arg) | TypeofSpecifier::TypeofUnqual(arg) => match arg {
            TypeofSpecifierArgument::Expression(e) => v.visit_expression(e),
            TypeofSpecifierArgument::TypeName(tn) => v.visit_type_name(tn),
            TypeofSpecifierArgument::Error => V::Result::output(),
        },
    }
}

/// Walk a specifier qualifier list.
pub fn walk_specifier_qualifier_list<'a, V: Visitor<'a> + ?Sized>(
    v: &mut V,
    s: &'a SpecifierQualifierList,
) -> V::Result {
    for item in &s.items {
        tr!(v.visit_type_specifier_qualifier(item));
    }
    for attr in &s.attributes {
        tr!(v.visit_attribute_specifier(attr));
    }
    V::Result::output()
}

/// Walk a declarator.
pub fn walk_declarator<'a, V: Visitor<'a> + ?Sized>(v: &mut V, d: &'a Declarator) -> V::Result {
    match d {
        Declarator::Direct(dd) => v.visit_direct_declarator(dd),
        Declarator::Pointer { pointer, declarator } => {
            tr!(v.visit_pointer(pointer));
            v.visit_declarator(declarator)
        }
        Declarator::Error => V::Result::output(),
    }
}

/// Walk a pointer.
pub fn walk_pointer<'a, V: Visitor<'a> + ?Sized>(v: &mut V, p: &'a Pointer) -> V::Result {
    tr!(v.visit_pointer_or_block(&p.pointer_or_block));
    for attr in &p.attributes {
        tr!(v.visit_attribute_specifier(attr));
    }
    for tq in &p.type_qualifiers {
        tr!(v.visit_type_qualifier(tq));
    }
    V::Result::output()
}

/// Walk a direct declarator.
pub fn walk_direct_declarator<'a, V: Visitor<'a> + ?Sized>(v: &mut V, d: &'a DirectDeclarator) -> V::Result {
    match d {
        DirectDeclarator::Identifier { identifier, attributes } => {
            tr!(v.visit_variable_name(identifier));
            for attr in attributes {
                tr!(v.visit_attribute_specifier(attr));
            }
            V::Result::output()
        }
        DirectDeclarator::Parenthesized(inner) => v.visit_declarator(inner),
        DirectDeclarator::Array { declarator, attributes, array_declarator } => {
            tr!(v.visit_direct_declarator(declarator));
            for attr in attributes {
                tr!(v.visit_attribute_specifier(attr));
            }
            v.visit_array_declarator(array_declarator)
        }
        DirectDeclarator::Function { declarator, attributes, parameters } => {
            tr!(v.visit_direct_declarator(declarator));
            for attr in attributes {
                tr!(v.visit_attribute_specifier(attr));
            }
            v.visit_parameter_type_list(parameters)
        }
    }
}

/// Walk a parameter type list.
pub fn walk_parameter_type_list<'a, V: Visitor<'a> + ?Sized>(v: &mut V, ptl: &'a ParameterTypeList) -> V::Result {
    match ptl {
        ParameterTypeList::Parameters(params) | ParameterTypeList::Variadic(params) => {
            for p in params {
                tr!(v.visit_parameter_declaration(p));
            }
            V::Result::output()
        }
        ParameterTypeList::OnlyVariadic => V::Result::output(),
    }
}

/// Walk a parameter declaration.
pub fn walk_parameter_declaration<'a, V: Visitor<'a> + ?Sized>(v: &mut V, pd: &'a ParameterDeclaration) -> V::Result {
    for attr in &pd.attributes {
        tr!(v.visit_attribute_specifier(attr));
    }
    tr!(v.visit_declaration_specifiers(&pd.specifiers));
    if let Some(kind) = &pd.declarator {
        tr!(v.visit_parameter_declaration_kind(kind));
    }
    V::Result::output()
}

/// Walk a parameter declaration kind.
pub fn walk_parameter_declaration_kind<'a, V: Visitor<'a> + ?Sized>(
    v: &mut V,
    pdk: &'a ParameterDeclarationKind,
) -> V::Result {
    match pdk {
        ParameterDeclarationKind::Declarator(d) => v.visit_declarator(d),
        ParameterDeclarationKind::Abstract(a) => v.visit_abstract_declarator(a),
    }
}

/// Walk an initializer.
pub fn walk_initializer<'a, V: Visitor<'a> + ?Sized>(v: &mut V, i: &'a Initializer) -> V::Result {
    match i {
        Initializer::Expression(e) => v.visit_expression(e),
        Initializer::Braced(b) => v.visit_braced_initializer(b),
    }
}

/// Walk a braced initializer.
fn walk_braced_initializer<'a, V: Visitor<'a> + ?Sized>(v: &mut V, b: &'a BracedInitializer) -> V::Result {
    for init in &b.initializers {
        tr!(v.visit_designated_initializer(init));
    }
    V::Result::output()
}

/// Walk a designated initializer.
pub fn walk_designated_initializer<'a, V: Visitor<'a> + ?Sized>(v: &mut V, d: &'a DesignatedInitializer) -> V::Result {
    if let Some(designation) = &d.designation {
        tr!(v.visit_designation(designation));
    }
    v.visit_initializer(&d.initializer)
}

/// Walk a designation.
pub fn walk_designation<'a, V: Visitor<'a> + ?Sized>(v: &mut V, d: &'a Designation) -> V::Result {
    tr!(v.visit_designator(&d.designator));
    if let Some(designation) = &d.designation {
        tr!(v.visit_designation(designation));
    }
    V::Result::output()
}

/// Walk a designator.
pub fn walk_designator<'a, V: Visitor<'a> + ?Sized>(v: &mut V, d: &'a Designator) -> V::Result {
    match d {
        Designator::Array(expr) => v.visit_constant_expression(expr),
        Designator::Member(identifier) => v.visit_member_name(identifier),
    }
}

/// Walk a constant expression.
pub fn walk_constant_expression<'a, V: Visitor<'a> + ?Sized>(v: &mut V, e: &'a ConstantExpression) -> V::Result {
    match e {
        ConstantExpression::Expression(expr) => v.visit_expression(expr),
        ConstantExpression::Error => V::Result::output(),
    }
}

/// Walk a type name (used in casts, sizeof, alignof, etc.).
pub fn walk_type_name<'a, V: Visitor<'a> + ?Sized>(v: &mut V, tn: &'a TypeName) -> V::Result {
    match tn {
        TypeName::TypeName { specifiers, abstract_declarator } => {
            tr!(v.visit_specifier_qualifier_list(specifiers));
            if let Some(ad) = abstract_declarator {
                tr!(v.visit_abstract_declarator(ad));
            }
            V::Result::output()
        }
        TypeName::Error => V::Result::output(),
    }
}

/// Walk an abstract declarator.
pub fn walk_abstract_declarator<'a, V: Visitor<'a> + ?Sized>(v: &mut V, a: &'a AbstractDeclarator) -> V::Result {
    match a {
        AbstractDeclarator::Direct(d) => v.visit_direct_abstract_declarator(d),
        AbstractDeclarator::Pointer { pointer, abstract_declarator } => {
            tr!(v.visit_pointer(pointer));
            if let Some(ad) = abstract_declarator {
                tr!(v.visit_abstract_declarator(ad));
            }
            V::Result::output()
        }
        AbstractDeclarator::Error => V::Result::output(),
    }
}

/// Walk a direct abstract declarator.
pub fn walk_direct_abstract_declarator<'a, V: Visitor<'a> + ?Sized>(
    v: &mut V,
    d: &'a DirectAbstractDeclarator,
) -> V::Result {
    match d {
        DirectAbstractDeclarator::Parenthesized(ad) => v.visit_abstract_declarator(ad),
        DirectAbstractDeclarator::Array { declarator, attributes, array_declarator } => {
            if let Some(dd) = declarator {
                tr!(v.visit_direct_abstract_declarator(dd));
            }
            for attr in attributes {
                tr!(v.visit_attribute_specifier(attr));
            }
            v.visit_array_declarator(array_declarator)
        }
        DirectAbstractDeclarator::Function { declarator, attributes, parameters } => {
            if let Some(dd) = declarator {
                tr!(v.visit_direct_abstract_declarator(dd))
            }
            for attr in attributes {
                tr!(v.visit_attribute_specifier(attr));
            }
            v.visit_parameter_type_list(parameters)
        }
    }
}

/// Walk an attribute specifier.
pub fn walk_attribute_specifier<'a, V: Visitor<'a> + ?Sized>(v: &mut V, a: &'a AttributeSpecifier) -> V::Result {
    match a {
        AttributeSpecifier::Attributes(attributes) => {
            for attr in attributes {
                tr!(v.visit_attribute(attr));
            }
        }
        AttributeSpecifier::Asm(string_literals) => {
            tr!(v.visit_asm_attribute_specifier(string_literals));
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

    /// Visits an initializer with mutable access.
    fn visit_initializer_mut(&mut self, i: &'a mut Initializer) -> Self::Result {
        walk_initializer_mut(self, i)
    }

    /// Visits a braced initializer with mutable access.
    fn visit_braced_initializer_mut(&mut self, b: &'a mut BracedInitializer) -> Self::Result {
        walk_braced_initializer_mut(self, b)
    }

    /// Visits a designated initializer with mutable access.
    fn visit_designated_initializer_mut(&mut self, d: &'a mut DesignatedInitializer) -> Self::Result {
        walk_designated_initializer_mut(self, d)
    }

    /// Visits a designation with mutable access.
    fn visit_designation_mut(&mut self, d: &'a mut Designation) -> Self::Result {
        walk_designation_mut(self, d)
    }

    /// Visits a designator with mutable access.
    fn visit_designator_mut(&mut self, d: &'a mut Designator) -> Self::Result {
        walk_designator_mut(self, d)
    }

    /// Visits a constant expression with mutable access.
    fn visit_constant_expression_mut(&mut self, c: &'a mut ConstantExpression) -> Self::Result {
        walk_constant_expression_mut(self, c)
    }

    /// Visits a postfix expression with mutable access.
    fn visit_postfix_expression_mut(&mut self, p: &'a mut PostfixExpression) -> Self::Result {
        walk_postfix_expression_mut(self, p)
    }

    /// Visits a primary expression with mutable access.
    fn visit_primary_expression_mut(&mut self, p: &'a mut PrimaryExpression) -> Self::Result {
        walk_primary_expression_mut(self, p)
    }

    /// Visits a generic selection with mutable access.
    fn visit_generic_selection_mut(&mut self, g: &'a mut GenericSelection) -> Self::Result {
        walk_generic_selection_mut(self, g)
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

    /// Visits a labeled statement with mutable access.
    fn visit_labeled_statement_mut(&mut self, ls: &'a mut LabeledStatement) -> Self::Result {
        walk_labeled_statement_mut(self, ls)
    }

    /// Visits a label with mutable access.
    fn visit_label_mut(&mut self, l: &'a mut Label) -> Self::Result {
        walk_label_mut(self, l)
    }

    /// Visits an expression statement with mutable access.
    fn visit_expression_statement_mut(&mut self, e: &'a mut ExpressionStatement) -> Self::Result {
        walk_expression_statement_mut(self, e)
    }

    /// Visits a primary block with mutable access.
    fn visit_primary_block_mut(&mut self, pb: &'a mut PrimaryBlock) -> Self::Result {
        walk_primary_block_mut(self, pb)
    }

    /// Visits a jump statement with mutable access.
    fn visit_jump_statement_mut(&mut self, j: &'a mut JumpStatement) -> Self::Result {
        walk_jump_statement_mut(self, j)
    }

    /// Visits a selection statement with mutable access.
    fn visit_selection_statement_mut(&mut self, s: &'a mut SelectionStatement) -> Self::Result {
        walk_selection_statement_mut(self, s)
    }

    /// Visits an iteration statement with mutable access.
    fn visit_iteration_statement_mut(&mut self, i: &'a mut IterationStatement) -> Self::Result {
        walk_iteration_statement_mut(self, i)
    }

    /// Visits a for init clause with mutable access.
    fn visit_for_init_mut(&mut self, fi: &'a mut ForInit) -> Self::Result {
        walk_for_init_mut(self, fi)
    }

    /// Visits a binary expression with mutable access.
    fn visit_binary_expression_mut(&mut self, b: &'a mut BinaryExpression) -> Self::Result {
        walk_binary_expression_mut(self, b)
    }

    /// Visits a conditional expression with mutable access.
    fn visit_conditional_expression_mut(&mut self, c: &'a mut ConditionalExpression) -> Self::Result {
        walk_conditional_expression_mut(self, c)
    }

    /// Visits an assignment expression with mutable access.
    fn visit_assignment_expression_mut(&mut self, a: &'a mut AssignmentExpression) -> Self::Result {
        walk_assignment_expression_mut(self, a)
    }

    /// Visits a comma expression with mutable access.
    fn visit_comma_expression_mut(&mut self, c: &'a mut CommaExpression) -> Self::Result {
        walk_comma_expression_mut(self, c)
    }

    /// Visits a binary operator with mutable access.
    fn visit_binary_operator_mut(&mut self, _: &'a mut BinaryOperator) -> Self::Result {
        Self::Result::output()
    }

    /// Visits an assignment operator with mutable access.
    fn visit_assignment_operator_mut(&mut self, _: &'a mut AssignmentOperator) -> Self::Result {
        Self::Result::output()
    }

    /// Visits a compound literal with mutable access.
    fn visit_compound_literal_mut(&mut self, cl: &'a mut CompoundLiteral) -> Self::Result {
        walk_compound_literal_mut(self, cl)
    }

    /// Visits a storage class specifier with mutable access.
    fn visit_storage_class_specifier_mut(&mut self, _: &'a mut StorageClassSpecifier) -> Self::Result {
        Self::Result::output()
    }

    /// Visits a unary operator with mutable access.
    fn visit_unary_operator_mut(&mut self, _: &'a mut UnaryOperator) -> Self::Result {
        Self::Result::output()
    }

    /// Visits an init declarator with mutable access.
    fn visit_init_declarator_mut(&mut self, id: &'a mut InitDeclarator) -> Self::Result {
        walk_init_declarator_mut(self, id)
    }

    /// Visits a static assert declaration with mutable access.
    fn visit_static_assert_declaration_mut(&mut self, sa: &'a mut StaticAssertDeclaration) -> Self::Result {
        walk_static_assert_declaration_mut(self, sa)
    }

    /// Visits a static assert message with mutable access.
    fn visit_static_assert_message_mut(&mut self, _: &'a mut StringLiterals) -> Self::Result {
        Self::Result::output()
    }

    /// Visits a declaration specifier with mutable access.
    fn visit_declaration_specifier_mut(&mut self, ds: &'a mut DeclarationSpecifier) -> Self::Result {
        walk_declaration_specifier_mut(self, ds)
    }

    /// Visits a function specifier with mutable access.
    fn visit_function_specifier_mut(&mut self, _: &'a mut FunctionSpecifier) -> Self::Result {
        Self::Result::output()
    }

    /// Visits a type qualifier with mutable access.
    fn visit_type_qualifier_mut(&mut self, _: &'a mut TypeQualifier) -> Self::Result {
        Self::Result::output()
    }

    /// Visits an alignment specifier with mutable access.
    fn visit_alignment_specifier_mut(&mut self, a: &'a mut AlignmentSpecifier) -> Self::Result {
        walk_alignment_specifier_mut(self, a)
    }

    /// Visits a struct or union specifier with mutable access.
    fn visit_struct_union_specifier_mut(&mut self, s: &'a mut StructOrUnionSpecifier) -> Self::Result {
        walk_struct_union_specifier_mut(self, s)
    }

    /// Visits an enum specifier with mutable access.
    fn visit_enum_specifier_mut(&mut self, e: &'a mut EnumSpecifier) -> Self::Result {
        walk_enum_specifier_mut(self, e)
    }

    /// Visits a struct or union keyword with mutable access.
    fn visit_struct_or_union_mut(&mut self, _: &'a mut StructOrUnion) -> Self::Result {
        Self::Result::output()
    }

    /// Visits a member declaration with mutable access.
    fn visit_member_declaration_mut(&mut self, md: &'a mut MemberDeclaration) -> Self::Result {
        walk_member_declaration_mut(self, md)
    }

    /// Visits a member declarator with mutable access.
    fn visit_member_declarator_mut(&mut self, md: &'a mut MemberDeclarator) -> Self::Result {
        walk_member_declarator_mut(self, md)
    }

    /// Visits an enumerator with mutable access.
    fn visit_enumerator_mut(&mut self, e: &'a mut Enumerator) -> Self::Result {
        walk_enumerator_mut(self, e)
    }

    /// Visits a pointer with mutable access.
    fn visit_pointer_mut(&mut self, p: &'a mut Pointer) -> Self::Result {
        walk_pointer_mut(self, p)
    }

    /// Visits a pointer or block indicator with mutable access.
    fn visit_pointer_or_block_mut(&mut self, _: &'a mut PointerOrBlock) -> Self::Result {
        Self::Result::output()
    }

    /// Visits an array declarator with mutable access.
    fn visit_array_declarator_mut(&mut self, _: &'a mut ArrayDeclarator) -> Self::Result {
        Self::Result::output()
    }

    /// Visits a parameter type list with mutable access.
    fn visit_parameter_type_list_mut(&mut self, ptl: &'a mut ParameterTypeList) -> Self::Result {
        walk_parameter_type_list_mut(self, ptl)
    }

    /// Visits a parameter declaration with mutable access.
    fn visit_parameter_declaration_mut(&mut self, pd: &'a mut ParameterDeclaration) -> Self::Result {
        walk_parameter_declaration_mut(self, pd)
    }

    /// Visits a parameter declaration kind with mutable access.
    fn visit_parameter_declaration_kind_mut(&mut self, pdk: &'a mut ParameterDeclarationKind) -> Self::Result {
        walk_parameter_declaration_kind_mut(self, pdk)
    }
}

// ============================================================================
// Mutable Walker Functions
// ============================================================================

/// Walk a translation unit with mutable access.
pub fn walk_translation_unit_mut<'a, V: VisitorMut<'a> + ?Sized>(v: &mut V, tu: &'a mut TranslationUnit) -> V::Result {
    for ed in &mut tu.external_declarations {
        tr!(v.visit_external_declaration_mut(ed));
    }
    V::Result::output()
}

/// Walk an external declaration with mutable access.
pub fn walk_external_declaration_mut<'a, V: VisitorMut<'a> + ?Sized>(
    v: &mut V,
    d: &'a mut ExternalDeclaration,
) -> V::Result {
    match d {
        ExternalDeclaration::Function(f) => v.visit_function_definition_mut(f),
        ExternalDeclaration::Declaration(d) => v.visit_declaration_mut(d),
    }
}

/// Walk a function definition with mutable access.
pub fn walk_function_definition_mut<'a, V: VisitorMut<'a> + ?Sized>(
    v: &mut V,
    f: &'a mut FunctionDefinition,
) -> V::Result {
    for attr in &mut f.attributes {
        tr!(v.visit_attribute_specifier_mut(attr));
    }
    tr!(v.visit_declaration_specifiers_mut(&mut f.specifiers));
    tr!(v.visit_declarator_mut(&mut f.declarator));
    v.visit_compound_statement_mut(&mut f.body)
}

/// Walk a statement with mutable access.
pub fn walk_statement_mut<'a, V: VisitorMut<'a> + ?Sized>(v: &mut V, s: &'a mut Statement) -> V::Result {
    match s {
        Statement::Labeled(ls) => v.visit_labeled_statement_mut(ls),
        Statement::Unlabeled(u) => v.visit_unlabeled_statement_mut(u),
    }
}

/// Walk a labeled statement with mutable access.
pub fn walk_labeled_statement_mut<'a, V: VisitorMut<'a> + ?Sized>(
    v: &mut V,
    ls: &'a mut LabeledStatement,
) -> V::Result {
    tr!(v.visit_label_mut(&mut ls.label));
    v.visit_statement_mut(&mut ls.statement)
}

/// Walk a label with mutable access.
pub fn walk_label_mut<'a, V: VisitorMut<'a> + ?Sized>(v: &mut V, l: &'a mut Label) -> V::Result {
    match l {
        Label::Identifier { attributes, identifier } => {
            for attribute in attributes {
                tr!(v.visit_attribute_specifier_mut(attribute));
            }
            v.visit_label_name_mut(identifier)
        }
        Label::Case { attributes, expression } => {
            for attribute in attributes {
                tr!(v.visit_attribute_specifier_mut(attribute));
            }
            v.visit_constant_expression_mut(expression)
        }
        Label::Default { attributes } => {
            for attribute in attributes {
                tr!(v.visit_attribute_specifier_mut(attribute));
            }
            V::Result::output()
        }
    }
}

/// Walk an unlabeled statement with mutable access.
pub fn walk_unlabeled_statement_mut<'a, V: VisitorMut<'a> + ?Sized>(
    v: &mut V,
    s: &'a mut UnlabeledStatement,
) -> V::Result {
    match s {
        UnlabeledStatement::Expression(es) => v.visit_expression_statement_mut(es),
        UnlabeledStatement::Primary { attributes, block } => {
            for attribute in attributes {
                tr!(v.visit_attribute_specifier_mut(attribute));
            }
            v.visit_primary_block_mut(block)
        }
        UnlabeledStatement::Jump { attributes, statement } => {
            for attribute in attributes {
                tr!(v.visit_attribute_specifier_mut(attribute));
            }
            v.visit_jump_statement_mut(statement)
        }
    }
}

/// Walk an expression statement with mutable access.
pub fn walk_expression_statement_mut<'a, V: VisitorMut<'a> + ?Sized>(
    v: &mut V,
    e: &'a mut ExpressionStatement,
) -> V::Result {
    if let Some(expr) = &mut e.expression {
        tr!(v.visit_expression_mut(expr));
    }
    V::Result::output()
}

/// Walk a primary block with mutable access.
pub fn walk_primary_block_mut<'a, V: VisitorMut<'a> + ?Sized>(v: &mut V, pb: &'a mut PrimaryBlock) -> V::Result {
    match pb {
        PrimaryBlock::Compound(c) => v.visit_compound_statement_mut(c),
        PrimaryBlock::Selection(sel) => v.visit_selection_statement_mut(sel),
        PrimaryBlock::Iteration(iter) => v.visit_iteration_statement_mut(iter),
    }
}

/// Walk a compound statement with mutable access.
pub fn walk_compound_statement_mut<'a, V: VisitorMut<'a> + ?Sized>(
    v: &mut V,
    c: &'a mut CompoundStatement,
) -> V::Result {
    for item in &mut c.items {
        match item {
            BlockItem::Declaration(d) => tr!(v.visit_declaration_mut(d)),
            BlockItem::Statement(u) => tr!(v.visit_unlabeled_statement_mut(u)),
            BlockItem::Label(l) => tr!(v.visit_label_mut(l)),
        }
    }
    V::Result::output()
}

/// Walk a selection statement with mutable access.
pub fn walk_selection_statement_mut<'a, V: VisitorMut<'a> + ?Sized>(
    v: &mut V,
    s: &'a mut SelectionStatement,
) -> V::Result {
    match s {
        SelectionStatement::If { condition, then_stmt, else_stmt } => {
            tr!(v.visit_expression_mut(condition));
            tr!(v.visit_statement_mut(then_stmt));
            if let Some(e) = else_stmt {
                tr!(v.visit_statement_mut(e));
            }
            V::Result::output()
        }
        SelectionStatement::Switch { expression, statement } => {
            tr!(v.visit_expression_mut(expression));
            v.visit_statement_mut(statement)
        }
    }
}

/// Walk an iteration statement with mutable access.
pub fn walk_iteration_statement_mut<'a, V: VisitorMut<'a> + ?Sized>(
    v: &mut V,
    i: &'a mut IterationStatement,
) -> V::Result {
    match i {
        IterationStatement::While { condition, body } => {
            tr!(v.visit_expression_mut(condition));
            v.visit_statement_mut(body)
        }
        IterationStatement::DoWhile { body, condition } => {
            tr!(v.visit_statement_mut(body));
            v.visit_expression_mut(condition)
        }
        IterationStatement::For { init, condition, update, body } => {
            if let Some(i) = init {
                v.visit_for_init_mut(i);
            }
            if let Some(c) = condition {
                tr!(v.visit_expression_mut(c));
            }
            if let Some(u) = update {
                tr!(v.visit_expression_mut(u));
            }
            v.visit_statement_mut(body)
        }
        IterationStatement::Error => V::Result::output(),
    }
}

/// Walk a for init with mutable access.
pub fn walk_for_init_mut<'a, V: VisitorMut<'a> + ?Sized>(v: &mut V, fi: &'a mut ForInit) -> V::Result {
    match fi {
        ForInit::Expression(e) => v.visit_expression_mut(e),
        ForInit::Declaration(d) => v.visit_declaration_mut(d),
    }
}

/// Walk a jump statement with mutable access.
pub fn walk_jump_statement_mut<'a, V: VisitorMut<'a> + ?Sized>(v: &mut V, j: &'a mut JumpStatement) -> V::Result {
    match j {
        JumpStatement::Goto(id) => v.visit_label_name_mut(id),
        JumpStatement::Continue | JumpStatement::Break => V::Result::output(),
        JumpStatement::Return(expr) => {
            if let Some(e) = expr {
                tr!(v.visit_expression_mut(e));
            }
            V::Result::output()
        }
    }
}

/// Walk an expression with mutable access.
pub fn walk_expression_mut<'a, V: VisitorMut<'a> + ?Sized>(v: &mut V, e: &'a mut Expression) -> V::Result {
    match e {
        Expression::Postfix(p) => v.visit_postfix_expression_mut(p),
        Expression::Unary(u) => v.visit_unary_expression_mut(u),
        Expression::Cast(c) => v.visit_cast_expression_mut(c),
        Expression::Binary(b) => v.visit_binary_expression_mut(b),
        Expression::Conditional(c) => v.visit_conditional_expression_mut(c),
        Expression::Assignment(a) => v.visit_assignment_expression_mut(a),
        Expression::Comma(c) => v.visit_comma_expression_mut(c),
        Expression::Error => V::Result::output(),
    }
}

/// Walk a binary expression with mutable access.
pub fn walk_binary_expression_mut<'a, V: VisitorMut<'a> + ?Sized>(v: &mut V, b: &'a mut BinaryExpression) -> V::Result {
    tr!(v.visit_expression_mut(&mut b.left));
    tr!(v.visit_binary_operator_mut(&mut b.operator));
    v.visit_expression_mut(&mut b.right)
}

/// Walk a conditional expression with mutable access.
pub fn walk_conditional_expression_mut<'a, V: VisitorMut<'a> + ?Sized>(
    v: &mut V,
    c: &'a mut ConditionalExpression,
) -> V::Result {
    tr!(v.visit_expression_mut(&mut c.condition));
    tr!(v.visit_expression_mut(&mut c.then_expr));
    v.visit_expression_mut(&mut c.else_expr)
}

/// Walk an assignment expression with mutable access.
pub fn walk_assignment_expression_mut<'a, V: VisitorMut<'a> + ?Sized>(
    v: &mut V,
    a: &'a mut AssignmentExpression,
) -> V::Result {
    tr!(v.visit_expression_mut(&mut a.left));
    tr!(v.visit_assignment_operator_mut(&mut a.operator));
    v.visit_expression_mut(&mut a.right)
}

/// Walk a comma expression with mutable access.
pub fn walk_comma_expression_mut<'a, V: VisitorMut<'a> + ?Sized>(v: &mut V, c: &'a mut CommaExpression) -> V::Result {
    for e in &mut c.expressions {
        tr!(v.visit_expression_mut(e));
    }
    V::Result::output()
}

/// Walk a postfix expression with mutable access.
pub fn walk_postfix_expression_mut<'a, V: VisitorMut<'a> + ?Sized>(
    v: &mut V,
    p: &'a mut PostfixExpression,
) -> V::Result {
    match p {
        PostfixExpression::Primary(pr) => v.visit_primary_expression_mut(pr),
        PostfixExpression::ArrayAccess { array, index } => {
            tr!(v.visit_postfix_expression_mut(array));
            v.visit_expression_mut(index)
        }
        PostfixExpression::FunctionCall { function, arguments } => {
            tr!(v.visit_postfix_expression_mut(function));
            for a in arguments {
                tr!(v.visit_expression_mut(a));
            }
            V::Result::output()
        }
        PostfixExpression::MemberAccess { object, member } => {
            tr!(v.visit_postfix_expression_mut(object));
            v.visit_member_name_mut(member)
        }
        PostfixExpression::MemberAccessPtr { object, member } => {
            tr!(v.visit_postfix_expression_mut(object));
            v.visit_member_name_mut(member)
        }
        PostfixExpression::PostIncrement(inner) | PostfixExpression::PostDecrement(inner) => {
            v.visit_postfix_expression_mut(inner)
        }
        PostfixExpression::CompoundLiteral(cl) => v.visit_compound_literal_mut(cl),
    }
}

/// Walk a compound literal with mutable access.
pub fn walk_compound_literal_mut<'a, V: VisitorMut<'a> + ?Sized>(v: &mut V, cl: &'a mut CompoundLiteral) -> V::Result {
    for specifier in &mut cl.storage_class_specifiers {
        tr!(v.visit_storage_class_specifier_mut(specifier));
    }
    tr!(v.visit_type_name_mut(&mut cl.type_name));
    v.visit_braced_initializer_mut(&mut cl.initializer)
}

/// Walk a primary expression with mutable access.
pub fn walk_primary_expression_mut<'a, V: VisitorMut<'a> + ?Sized>(
    v: &mut V,
    pr: &'a mut PrimaryExpression,
) -> V::Result {
    match pr {
        PrimaryExpression::Identifier(id) => v.visit_variable_name_mut(id),
        PrimaryExpression::EnumerationConstant(id) => v.visit_enum_constant_mut(id),
        PrimaryExpression::Parenthesized(e) => v.visit_expression_mut(e),
        PrimaryExpression::Generic(g) => v.visit_generic_selection_mut(g),
        _ => V::Result::output(),
    }
}

/// Walk a generic selection with mutable access.
pub fn walk_generic_selection_mut<'a, V: VisitorMut<'a> + ?Sized>(v: &mut V, g: &'a mut GenericSelection) -> V::Result {
    tr!(v.visit_expression_mut(&mut g.controlling_expression));
    for assoc in &mut g.associations {
        match assoc {
            GenericAssociation::Type { type_name, expression } => {
                tr!(v.visit_type_name_mut(type_name));
                tr!(v.visit_expression_mut(expression))
            }
            GenericAssociation::Default { expression } => tr!(v.visit_expression_mut(expression)),
        };
    }
    V::Result::output()
}

/// Walk a unary expression with mutable access.
pub fn walk_unary_expression_mut<'a, V: VisitorMut<'a> + ?Sized>(v: &mut V, u: &'a mut UnaryExpression) -> V::Result {
    match u {
        UnaryExpression::Postfix(p) => v.visit_postfix_expression_mut(p),
        UnaryExpression::PreIncrement(inner) | UnaryExpression::PreDecrement(inner) => {
            v.visit_unary_expression_mut(inner)
        }
        UnaryExpression::Unary { operator, operand } => {
            tr!(v.visit_unary_operator_mut(operator));
            v.visit_cast_expression_mut(operand)
        }
        UnaryExpression::Sizeof(inner) => v.visit_unary_expression_mut(inner),
        UnaryExpression::SizeofType(tn) | UnaryExpression::Alignof(tn) => v.visit_type_name_mut(tn),
    }
}

/// Walk a cast expression with mutable access.
pub fn walk_cast_expression_mut<'a, V: VisitorMut<'a> + ?Sized>(v: &mut V, c: &'a mut CastExpression) -> V::Result {
    match c {
        CastExpression::Unary(u) => v.visit_unary_expression_mut(u),
        CastExpression::Cast { type_name, expression } => {
            tr!(v.visit_type_name_mut(type_name));
            v.visit_cast_expression_mut(expression)
        }
    }
}

/// Walk a declaration with mutable access.
pub fn walk_declaration_mut<'a, V: VisitorMut<'a> + ?Sized>(v: &mut V, d: &'a mut Declaration) -> V::Result {
    match d {
        Declaration::Normal { attributes, specifiers, declarators } => {
            for attr in attributes {
                tr!(v.visit_attribute_specifier_mut(attr));
            }
            tr!(v.visit_declaration_specifiers_mut(specifiers));
            for init in declarators {
                tr!(v.visit_init_declarator_mut(init));
            }
            V::Result::output()
        }
        Declaration::Typedef { attributes, specifiers, declarators } => {
            for attr in attributes {
                tr!(v.visit_attribute_specifier_mut(attr));
            }
            tr!(v.visit_declaration_specifiers_mut(specifiers));
            for d in declarators {
                tr!(v.visit_declarator_mut(d));
            }
            V::Result::output()
        }
        Declaration::StaticAssert(sa) => v.visit_static_assert_declaration_mut(sa),
        Declaration::Attribute(attrs) => {
            for attr in attrs {
                tr!(v.visit_attribute_specifier_mut(attr));
            }
            V::Result::output()
        }
        Declaration::Error => V::Result::output(),
    }
}

/// Walk an init declarator with mutable access.
pub fn walk_init_declarator_mut<'a, V: VisitorMut<'a> + ?Sized>(v: &mut V, id: &'a mut InitDeclarator) -> V::Result {
    tr!(v.visit_declarator_mut(&mut id.declarator));
    if let Some(init) = &mut id.initializer {
        tr!(v.visit_initializer_mut(init));
    }
    V::Result::output()
}

/// Walk a static assert declaration with mutable access.
pub fn walk_static_assert_declaration_mut<'a, V: VisitorMut<'a> + ?Sized>(
    v: &mut V,
    sa: &'a mut StaticAssertDeclaration,
) -> V::Result {
    tr!(v.visit_constant_expression_mut(&mut sa.condition));
    if let Some(msg) = &mut sa.message {
        tr!(v.visit_static_assert_message_mut(msg));
    }
    V::Result::output()
}

/// Walk declaration specifiers with mutable access.
pub fn walk_declaration_specifiers_mut<'a, V: VisitorMut<'a> + ?Sized>(
    v: &mut V,
    s: &'a mut DeclarationSpecifiers,
) -> V::Result {
    for it in &mut s.specifiers {
        tr!(v.visit_declaration_specifier_mut(it));
    }
    for attr in &mut s.attributes {
        tr!(v.visit_attribute_specifier_mut(attr));
    }
    V::Result::output()
}

/// Walk a declaration specifier with mutable access.
pub fn walk_declaration_specifier_mut<'a, V: VisitorMut<'a> + ?Sized>(
    v: &mut V,
    ds: &'a mut DeclarationSpecifier,
) -> V::Result {
    match ds {
        DeclarationSpecifier::StorageClass(scs) => v.visit_storage_class_specifier_mut(scs),
        DeclarationSpecifier::TypeSpecifierQualifier(tsq) => v.visit_type_specifier_qualifier_mut(tsq),
        DeclarationSpecifier::Function(fs) => v.visit_function_specifier_mut(fs),
    }
}

/// Walk a type specifier or qualifier with mutable access.
pub fn walk_type_specifier_qualifier_mut<'a, V: VisitorMut<'a> + ?Sized>(
    v: &mut V,
    x: &'a mut TypeSpecifierQualifier,
) -> V::Result {
    match x {
        TypeSpecifierQualifier::TypeSpecifier(ts) => v.visit_type_specifier_mut(ts),
        TypeSpecifierQualifier::TypeQualifier(tq) => v.visit_type_qualifier_mut(tq),
        TypeSpecifierQualifier::AlignmentSpecifier(a) => v.visit_alignment_specifier_mut(a),
    }
}

/// Walk an alignment specifier with mutable access.
pub fn walk_alignment_specifier_mut<'a, V: VisitorMut<'a> + ?Sized>(
    v: &mut V,
    a: &'a mut AlignmentSpecifier,
) -> V::Result {
    match a {
        AlignmentSpecifier::Type(tn) => v.visit_type_name_mut(tn),
        AlignmentSpecifier::Expression(e) => v.visit_constant_expression_mut(e),
    }
}

/// Walk a type specifier with mutable access.
pub fn walk_type_specifier_mut<'a, V: VisitorMut<'a> + ?Sized>(v: &mut V, ts: &'a mut TypeSpecifier) -> V::Result {
    match ts {
        TypeSpecifier::Struct(s) => v.visit_struct_union_specifier_mut(s),
        TypeSpecifier::Enum(e) => v.visit_enum_specifier_mut(e),
        TypeSpecifier::TypedefName(id) => v.visit_type_name_identifier_mut(id),
        TypeSpecifier::Atomic(a) => v.visit_atomic_type_specifier_mut(a),
        TypeSpecifier::Typeof(t) => v.visit_typeof_mut(t),
        TypeSpecifier::BitInt(c) => v.visit_constant_expression_mut(c),
        TypeSpecifier::Void
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

/// Walk a struct or union specifier with mutable access.
pub fn walk_struct_union_specifier_mut<'a, V: VisitorMut<'a> + ?Sized>(
    v: &mut V,
    s: &'a mut StructOrUnionSpecifier,
) -> V::Result {
    tr!(v.visit_struct_or_union_mut(&mut s.kind));
    for attr in &mut s.attributes {
        tr!(v.visit_attribute_specifier_mut(attr));
    }
    if let Some(id) = &mut s.identifier {
        tr!(v.visit_struct_name_mut(id));
    }
    if let Some(members) = &mut s.members {
        for m in members {
            tr!(v.visit_member_declaration_mut(m));
        }
    }
    V::Result::output()
}

/// Walk a member declaration with mutable access.
pub fn walk_member_declaration_mut<'a, V: VisitorMut<'a> + ?Sized>(
    v: &mut V,
    md: &'a mut MemberDeclaration,
) -> V::Result {
    match md {
        MemberDeclaration::Normal { attributes, specifiers, declarators } => {
            for attr in attributes {
                tr!(v.visit_attribute_specifier_mut(attr));
            }
            tr!(v.visit_specifier_qualifier_list_mut(specifiers));
            for d in declarators {
                tr!(v.visit_member_declarator_mut(d));
            }
            V::Result::output()
        }
        MemberDeclaration::StaticAssert(sa) => v.visit_static_assert_declaration_mut(sa),
        MemberDeclaration::Error => V::Result::output(),
    }
}

/// Walk a member declarator with mutable access.
pub fn walk_member_declarator_mut<'a, V: VisitorMut<'a> + ?Sized>(
    v: &mut V,
    md: &'a mut MemberDeclarator,
) -> V::Result {
    match md {
        MemberDeclarator::Declarator(d) => v.visit_declarator_mut(d),
        MemberDeclarator::BitField { declarator, width } => {
            if let Some(d) = declarator {
                tr!(v.visit_declarator_mut(d));
            }
            v.visit_constant_expression_mut(width)
        }
    }
}

/// Walk an enum specifier with mutable access.
pub fn walk_enum_specifier_mut<'a, V: VisitorMut<'a> + ?Sized>(v: &mut V, e: &'a mut EnumSpecifier) -> V::Result {
    for attr in &mut e.attributes {
        tr!(v.visit_attribute_specifier_mut(attr));
    }
    if let Some(id) = &mut e.identifier {
        tr!(v.visit_enum_name_mut(id));
    }
    if let Some(sql) = &mut e.type_specifier {
        tr!(v.visit_specifier_qualifier_list_mut(sql));
    }
    if let Some(enumerators) = &mut e.enumerators {
        for en in enumerators {
            tr!(v.visit_enumerator_mut(en));
        }
    }
    V::Result::output()
}

/// Walk an enumerator with mutable access.
pub fn walk_enumerator_mut<'a, V: VisitorMut<'a> + ?Sized>(v: &mut V, e: &'a mut Enumerator) -> V::Result {
    tr!(v.visit_enumerator_name_mut(&mut e.name));
    for attr in &mut e.attributes {
        tr!(v.visit_attribute_specifier_mut(attr));
    }
    if let Some(value) = &mut e.value {
        tr!(v.visit_constant_expression_mut(value));
    }
    V::Result::output()
}

/// Walk an atomic type specifier with mutable access.
pub fn walk_atomic_type_specifier_mut<'a, V: VisitorMut<'a> + ?Sized>(
    v: &mut V,
    a: &'a mut AtomicTypeSpecifier,
) -> V::Result {
    v.visit_type_name_mut(&mut a.type_name)
}

/// Walk a typeof specifier with mutable access.
pub fn walk_typeof_mut<'a, V: VisitorMut<'a> + ?Sized>(v: &mut V, t: &'a mut TypeofSpecifier) -> V::Result {
    match t {
        TypeofSpecifier::Typeof(arg) | TypeofSpecifier::TypeofUnqual(arg) => match arg {
            TypeofSpecifierArgument::Expression(e) => v.visit_expression_mut(e),
            TypeofSpecifierArgument::TypeName(tn) => v.visit_type_name_mut(tn),
            TypeofSpecifierArgument::Error => V::Result::output(),
        },
    }
}

/// Walk a specifier qualifier list with mutable access.
pub fn walk_specifier_qualifier_list_mut<'a, V: VisitorMut<'a> + ?Sized>(
    v: &mut V,
    s: &'a mut SpecifierQualifierList,
) -> V::Result {
    for item in &mut s.items {
        tr!(v.visit_type_specifier_qualifier_mut(item));
    }
    for attr in &mut s.attributes {
        tr!(v.visit_attribute_specifier_mut(attr));
    }
    V::Result::output()
}

/// Walk a declarator with mutable access.
pub fn walk_declarator_mut<'a, V: VisitorMut<'a> + ?Sized>(v: &mut V, d: &'a mut Declarator) -> V::Result {
    match d {
        Declarator::Direct(dd) => v.visit_direct_declarator_mut(dd),
        Declarator::Pointer { pointer, declarator } => {
            tr!(v.visit_pointer_mut(pointer));
            v.visit_declarator_mut(declarator)
        }
        Declarator::Error => V::Result::output(),
    }
}

/// Walk a pointer with mutable access.
pub fn walk_pointer_mut<'a, V: VisitorMut<'a> + ?Sized>(v: &mut V, p: &'a mut Pointer) -> V::Result {
    tr!(v.visit_pointer_or_block_mut(&mut p.pointer_or_block));
    for attr in &mut p.attributes {
        tr!(v.visit_attribute_specifier_mut(attr));
    }
    for tq in &mut p.type_qualifiers {
        tr!(v.visit_type_qualifier_mut(tq));
    }
    V::Result::output()
}

/// Walk a direct declarator with mutable access.
pub fn walk_direct_declarator_mut<'a, V: VisitorMut<'a> + ?Sized>(v: &mut V, d: &'a mut DirectDeclarator) -> V::Result {
    match d {
        DirectDeclarator::Identifier { identifier, attributes } => {
            tr!(v.visit_variable_name_mut(identifier));
            for attr in attributes {
                tr!(v.visit_attribute_specifier_mut(attr));
            }
            V::Result::output()
        }
        DirectDeclarator::Parenthesized(inner) => v.visit_declarator_mut(inner),
        DirectDeclarator::Array { declarator, attributes, array_declarator } => {
            tr!(v.visit_direct_declarator_mut(declarator));
            for attr in attributes {
                tr!(v.visit_attribute_specifier_mut(attr));
            }
            v.visit_array_declarator_mut(array_declarator)
        }
        DirectDeclarator::Function { declarator, attributes, parameters } => {
            tr!(v.visit_direct_declarator_mut(declarator));
            for attr in attributes {
                tr!(v.visit_attribute_specifier_mut(attr));
            }
            v.visit_parameter_type_list_mut(parameters)
        }
    }
}

/// Walk a parameter type list with mutable access.
pub fn walk_parameter_type_list_mut<'a, V: VisitorMut<'a> + ?Sized>(
    v: &mut V,
    ptl: &'a mut ParameterTypeList,
) -> V::Result {
    match ptl {
        ParameterTypeList::Parameters(params) | ParameterTypeList::Variadic(params) => {
            for p in params {
                tr!(v.visit_parameter_declaration_mut(p));
            }
            V::Result::output()
        }
        ParameterTypeList::OnlyVariadic => V::Result::output(),
    }
}

/// Walk a parameter declaration with mutable access.
pub fn walk_parameter_declaration_mut<'a, V: VisitorMut<'a> + ?Sized>(
    v: &mut V,
    pd: &'a mut ParameterDeclaration,
) -> V::Result {
    for attr in &mut pd.attributes {
        tr!(v.visit_attribute_specifier_mut(attr));
    }
    tr!(v.visit_declaration_specifiers_mut(&mut pd.specifiers));
    if let Some(kind) = &mut pd.declarator {
        tr!(v.visit_parameter_declaration_kind_mut(kind));
    }
    V::Result::output()
}

/// Walk a parameter declaration kind with mutable access.
pub fn walk_parameter_declaration_kind_mut<'a, V: VisitorMut<'a> + ?Sized>(
    v: &mut V,
    pdk: &'a mut ParameterDeclarationKind,
) -> V::Result {
    match pdk {
        ParameterDeclarationKind::Declarator(d) => v.visit_declarator_mut(d),
        ParameterDeclarationKind::Abstract(a) => v.visit_abstract_declarator_mut(a),
    }
}

/// Walk an initializer with mutable access.
pub fn walk_initializer_mut<'a, V: VisitorMut<'a> + ?Sized>(v: &mut V, i: &'a mut Initializer) -> V::Result {
    match i {
        Initializer::Expression(e) => v.visit_expression_mut(e),
        Initializer::Braced(b) => v.visit_braced_initializer_mut(b),
    }
}

/// Walk a braced initializer with mutable access.
pub fn walk_braced_initializer_mut<'a, V: VisitorMut<'a> + ?Sized>(
    v: &mut V,
    b: &'a mut BracedInitializer,
) -> V::Result {
    for init in &mut b.initializers {
        tr!(v.visit_designated_initializer_mut(init));
    }
    V::Result::output()
}

/// Walk a designated initializer with mutable access.
pub fn walk_designated_initializer_mut<'a, V: VisitorMut<'a> + ?Sized>(
    v: &mut V,
    d: &'a mut DesignatedInitializer,
) -> V::Result {
    if let Some(designation) = &mut d.designation {
        tr!(v.visit_designation_mut(designation));
    }
    v.visit_initializer_mut(&mut d.initializer)
}

/// Walk a designation with mutable access.
pub fn walk_designation_mut<'a, V: VisitorMut<'a> + ?Sized>(v: &mut V, d: &'a mut Designation) -> V::Result {
    tr!(v.visit_designator_mut(&mut d.designator));
    if let Some(designation) = &mut d.designation {
        tr!(v.visit_designation_mut(designation));
    }
    V::Result::output()
}

/// Walk a designator with mutable access.
pub fn walk_designator_mut<'a, V: VisitorMut<'a> + ?Sized>(v: &mut V, d: &'a mut Designator) -> V::Result {
    match d {
        Designator::Array(expr) => v.visit_constant_expression_mut(expr),
        Designator::Member(identifier) => v.visit_member_name_mut(identifier),
    }
}

/// Walk a constant expression with mutable access.
pub fn walk_constant_expression_mut<'a, V: VisitorMut<'a> + ?Sized>(
    v: &mut V,
    e: &'a mut ConstantExpression,
) -> V::Result {
    match e {
        ConstantExpression::Expression(expr) => v.visit_expression_mut(expr),
        ConstantExpression::Error => V::Result::output(),
    }
}

/// Walk a type name with mutable access.
pub fn walk_type_name_mut<'a, V: VisitorMut<'a> + ?Sized>(v: &mut V, tn: &'a mut TypeName) -> V::Result {
    match tn {
        TypeName::TypeName { specifiers, abstract_declarator } => {
            tr!(v.visit_specifier_qualifier_list_mut(specifiers));
            if let Some(ad) = abstract_declarator {
                tr!(v.visit_abstract_declarator_mut(ad));
            }
            V::Result::output()
        }
        TypeName::Error => V::Result::output(),
    }
}

/// Walk an abstract declarator with mutable access.
pub fn walk_abstract_declarator_mut<'a, V: VisitorMut<'a> + ?Sized>(
    v: &mut V,
    a: &'a mut AbstractDeclarator,
) -> V::Result {
    match a {
        AbstractDeclarator::Direct(d) => v.visit_direct_abstract_declarator_mut(d),
        AbstractDeclarator::Pointer { pointer, abstract_declarator } => {
            tr!(v.visit_pointer_mut(pointer));
            if let Some(ad) = abstract_declarator {
                tr!(v.visit_abstract_declarator_mut(ad));
            }
            V::Result::output()
        }
        AbstractDeclarator::Error => V::Result::output(),
    }
}

/// Walk a direct abstract declarator with mutable access.
pub fn walk_direct_abstract_declarator_mut<'a, V: VisitorMut<'a> + ?Sized>(
    v: &mut V,
    d: &'a mut DirectAbstractDeclarator,
) -> V::Result {
    match d {
        DirectAbstractDeclarator::Parenthesized(ad) => v.visit_abstract_declarator_mut(ad),
        DirectAbstractDeclarator::Array { declarator, attributes, array_declarator } => {
            if let Some(dd) = declarator {
                tr!(v.visit_direct_abstract_declarator_mut(dd));
            }
            for attr in attributes {
                tr!(v.visit_attribute_specifier_mut(attr));
            }
            v.visit_array_declarator_mut(array_declarator)
        }
        DirectAbstractDeclarator::Function { declarator, attributes, parameters } => {
            if let Some(dd) = declarator {
                tr!(v.visit_direct_abstract_declarator_mut(dd))
            }
            for attr in attributes {
                tr!(v.visit_attribute_specifier_mut(attr));
            }
            v.visit_parameter_type_list_mut(parameters)
        }
    }
}

/// Walk an attribute specifier with mutable access.
pub fn walk_attribute_specifier_mut<'a, V: VisitorMut<'a> + ?Sized>(
    v: &mut V,
    a: &'a mut AttributeSpecifier,
) -> V::Result {
    match a {
        AttributeSpecifier::Attributes(attributes) => {
            for attr in attributes {
                tr!(v.visit_attribute_mut(attr));
            }
        }
        AttributeSpecifier::Asm(string_literals) => {
            tr!(v.visit_asm_attribute_specifier_mut(string_literals));
        }
        AttributeSpecifier::Error => {}
    }
    V::Result::output()
}
