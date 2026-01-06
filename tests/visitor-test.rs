use std::ops::ControlFlow;

use cgrammar::{visitor::*, *};
use chumsky::prelude::*;
use rstest::rstest;

// Helper function to parse C code
fn parse_c(code: &str) -> TranslationUnit {
    let lexer = balanced_token_sequence();
    let tokens = lexer.parse(code).unwrap();
    let parser = translation_unit();
    let result = parser.parse(tokens.as_input());
    result.output().unwrap().clone()
}

// ============================================================================
// Basic Visitor Tests - Counting Nodes
// ============================================================================

/// A visitor that counts variable names
struct VariableCounter {
    count: usize,
}

impl<'a> Visitor<'a> for VariableCounter {
    type Result = ();

    fn visit_variable_name(&mut self, _: &'a Identifier) {
        self.count += 1;
    }
}

#[rstest]
#[case("int x;", 1)]
#[case("int x, y, z;", 3)]
#[case("int x; int y;", 2)]
#[case("int add(int a, int b) { return a + b; }", 5)] // add, a, b, a, b
fn test_variable_counter(#[case] code: &str, #[case] expected: usize) {
    let ast = parse_c(code);
    let mut visitor = VariableCounter { count: 0 };
    visitor.visit_translation_unit(&ast);
    assert_eq!(visitor.count, expected, "Failed for code: {}", code);
}

// ============================================================================
// Collecting Identifiers by Type
// ============================================================================

/// A visitor that collects different types of identifiers
#[derive(Default)]
struct IdentifierCollector {
    variables: Vec<String>,
    type_names: Vec<String>,
    struct_names: Vec<String>,
    enum_names: Vec<String>,
    enumerators: Vec<String>,
    labels: Vec<String>,
    members: Vec<String>,
}

impl<'a> Visitor<'a> for IdentifierCollector {
    type Result = ();

    fn visit_variable_name(&mut self, id: &'a Identifier) {
        self.variables.push(id.0.clone());
    }

    fn visit_type_name_identifier(&mut self, id: &'a Identifier) {
        self.type_names.push(id.0.clone());
    }

    fn visit_struct_name(&mut self, id: &'a Identifier) {
        self.struct_names.push(id.0.clone());
    }

    fn visit_enum_name(&mut self, id: &'a Identifier) {
        self.enum_names.push(id.0.clone());
    }

    fn visit_enumerator_name(&mut self, id: &'a Identifier) {
        self.enumerators.push(id.0.clone());
    }

    fn visit_label_name(&mut self, id: &'a Identifier) {
        self.labels.push(id.0.clone());
    }

    fn visit_member_name(&mut self, id: &'a Identifier) {
        self.members.push(id.0.clone());
    }
}

#[test]
fn test_struct_names() {
    let code = "struct Point { int x; int y; }; struct Point p;";
    let ast = parse_c(code);
    let mut visitor = IdentifierCollector::default();
    visitor.visit_translation_unit(&ast);

    assert_eq!(visitor.struct_names.len(), 2);
    assert!(visitor.struct_names.iter().all(|n| n == "Point"));
}

#[test]
fn test_enum_collection() {
    let code = "enum Color { RED, GREEN, BLUE }; enum Color c;";
    let ast = parse_c(code);
    let mut visitor = IdentifierCollector::default();
    visitor.visit_translation_unit(&ast);

    assert_eq!(visitor.enum_names.len(), 2);
    assert_eq!(visitor.enumerators, vec!["RED", "GREEN", "BLUE"]);
}

#[test]
fn test_member_access() {
    let code = r#"
        struct Point { int x; int y; };
        void f() {
            struct Point p;
            p.x = 1;
            p.y = 2;
        }
    "#;
    let ast = parse_c(code);
    let mut visitor = IdentifierCollector::default();
    visitor.visit_translation_unit(&ast);

    // Member access should be collected
    assert!(
        visitor.members.len() >= 2,
        "Expected at least 2 member accesses, got {}",
        visitor.members.len()
    );
    assert!(
        visitor.members.contains(&"x".to_string()),
        "Expected to find member 'x'"
    );
    assert!(
        visitor.members.contains(&"y".to_string()),
        "Expected to find member 'y'"
    );
}

#[test]
fn test_label_collection() {
    let code = r#"
        void f() {
            goto end;
            end: return;
        }
    "#;
    let ast = parse_c(code);
    let mut visitor = IdentifierCollector::default();
    visitor.visit_translation_unit(&ast);

    assert_eq!(visitor.labels, vec!["end", "end"]);
}

#[test]
fn test_typedef_names() {
    let code = "typedef int MyInt; MyInt x;";
    let ast = parse_c(code);
    let mut visitor = IdentifierCollector::default();
    visitor.visit_translation_unit(&ast);

    // MyInt appears as variable in typedef declaration, and as type_name in usage
    assert_eq!(visitor.type_names, vec!["MyInt"]);
    assert!(visitor.variables.contains(&"MyInt".to_string()));
    assert!(visitor.variables.contains(&"x".to_string()));
}

// ============================================================================
// Early Termination with ControlFlow
// ============================================================================

/// A visitor that stops after finding a specific identifier
struct IdentifierFinder {
    target: String,
    found: bool,
}

impl IdentifierFinder {
    fn new(target: &str) -> Self {
        Self { target: target.to_string(), found: false }
    }
}

impl<'a> Visitor<'a> for IdentifierFinder {
    type Result = ControlFlow<()>;

    fn visit_variable_name(&mut self, id: &'a Identifier) -> Self::Result {
        if id.0 == self.target {
            self.found = true;
            ControlFlow::Break(())
        } else {
            ControlFlow::Continue(())
        }
    }
}

#[test]
fn test_early_termination() {
    let code = "int a; int b; int target; int c; int d;";
    let ast = parse_c(code);
    let mut visitor = IdentifierFinder::new("target");
    let _ = visitor.visit_translation_unit(&ast);

    assert!(visitor.found);
}

#[test]
fn test_not_found() {
    let code = "int a; int b; int c;";
    let ast = parse_c(code);
    let mut visitor = IdentifierFinder::new("target");
    let _ = visitor.visit_translation_unit(&ast);

    assert!(!visitor.found);
}

// ============================================================================
// Complex AST Traversal Tests
// ============================================================================

/// Count all expression nodes
struct ExpressionCounter {
    count: usize,
}

impl<'a> Visitor<'a> for ExpressionCounter {
    type Result = ();

    fn visit_expression(&mut self, e: &'a Expression) -> Self::Result {
        self.count += 1;
        walk_expression(self, e)
    }
}

#[rstest]
#[case("void f() { x = 1 + 2; }", 5)] // assignment + left(x) + binary + left(1) + right(2)
#[case("void f() { x = a ? b : c; }", 6)] // assignment + left(x) + conditional + cond(a) + then(b) + else(c)
#[case("void f() { x = (a + b) * c; }", 8)] // assignment + left(x) + binary(*) + binary(+) + a + b + c
fn test_expression_counter(#[case] code: &str, #[case] expected: usize) {
    let ast = parse_c(code);
    let mut visitor = ExpressionCounter { count: 0 };
    visitor.visit_translation_unit(&ast);
    assert_eq!(visitor.count, expected, "Failed for code: {}", code);
}

/// Count function definitions
struct FunctionCounter {
    count: usize,
}

impl<'a> Visitor<'a> for FunctionCounter {
    type Result = ();

    fn visit_function_definition(&mut self, f: &'a FunctionDefinition) -> Self::Result {
        self.count += 1;
        walk_function_definition(self, f)
    }
}

#[test]
fn test_function_counter() {
    let code = r#"
        void f1() {}
        int f2(int x) { return x; }
        void f3(void) {}
    "#;
    let ast = parse_c(code);
    let mut visitor = FunctionCounter { count: 0 };
    visitor.visit_translation_unit(&ast);

    assert_eq!(visitor.count, 3);
}

// ============================================================================
// Statement Traversal Tests
// ============================================================================

/// Count different statement types
#[derive(Default)]
struct StatementCounter {
    if_count: usize,
    while_count: usize,
    for_count: usize,
    return_count: usize,
}

impl<'a> Visitor<'a> for StatementCounter {
    type Result = ();

    fn visit_unlabeled_statement(&mut self, s: &'a UnlabeledStatement) -> Self::Result {
        match s {
            UnlabeledStatement::Primary { block, .. } => match block {
                PrimaryBlock::Selection(SelectionStatement::If { .. }) => {
                    self.if_count += 1;
                }
                PrimaryBlock::Iteration(IterationStatement::While { .. }) => {
                    self.while_count += 1;
                }
                PrimaryBlock::Iteration(IterationStatement::For { .. }) => {
                    self.for_count += 1;
                }
                _ => {}
            },
            UnlabeledStatement::Jump { statement, .. } => {
                if matches!(statement, JumpStatement::Return(_)) {
                    self.return_count += 1;
                }
            }
            _ => {}
        }
        walk_unlabeled_statement(self, s)
    }
}

#[test]
fn test_statement_counter() {
    let code = r#"
        void f() {
            if (1) {}
            while (1) {}
            for (;;) {}
            return;
            if (x) return;
        }
    "#;
    let ast = parse_c(code);
    let mut visitor = StatementCounter::default();
    visitor.visit_translation_unit(&ast);

    assert_eq!(visitor.if_count, 2);
    assert_eq!(visitor.while_count, 1);
    assert_eq!(visitor.for_count, 1);
    assert_eq!(visitor.return_count, 2);
}

// ============================================================================
// Declaration Tests
// ============================================================================

/// Count declarations vs function definitions
#[derive(Default)]
struct DeclarationTypeCounter {
    normal_declarations: usize,
    typedef_declarations: usize,
    function_definitions: usize,
}

impl<'a> Visitor<'a> for DeclarationTypeCounter {
    type Result = ();

    fn visit_external_declaration(&mut self, d: &'a ExternalDeclaration) -> Self::Result {
        match d {
            ExternalDeclaration::Function(_) => {
                self.function_definitions += 1;
            }
            ExternalDeclaration::Declaration(decl) => match decl {
                Declaration::Normal { .. } => {
                    self.normal_declarations += 1;
                }
                Declaration::Typedef { .. } => {
                    self.typedef_declarations += 1;
                }
                _ => {}
            },
        }
        walk_external_declaration(self, d)
    }
}

#[test]
fn test_declaration_types() {
    let code = r#"
        int x;
        typedef int MyInt;
        extern int y;
        void f() {}
        int z;
    "#;
    let ast = parse_c(code);
    let mut visitor = DeclarationTypeCounter::default();
    visitor.visit_translation_unit(&ast);

    assert_eq!(visitor.normal_declarations, 3); // x, y, z
    assert_eq!(visitor.typedef_declarations, 1);
    assert_eq!(visitor.function_definitions, 1);
}

// ============================================================================
// Nested Structure Tests
// ============================================================================

#[test]
fn test_nested_function_calls() {
    let code = "void f() { a(b(c(d()))); }";
    let ast = parse_c(code);
    let mut visitor = VariableCounter { count: 0 };
    visitor.visit_translation_unit(&ast);

    // f (function name), a, b, c, d
    assert_eq!(visitor.count, 5);
}

#[test]
fn test_nested_structs() {
    let code = r#"
        struct Outer {
            struct Inner {
                int x;
            } inner;
        };
    "#;
    let ast = parse_c(code);
    let mut visitor = IdentifierCollector::default();
    visitor.visit_translation_unit(&ast);

    assert_eq!(visitor.struct_names, vec!["Outer", "Inner"]);
}

// ============================================================================
// Control Flow Break in Different Contexts
// ============================================================================

/// Visitor that stops after finding the first function
struct FirstFunctionFinder {
    found_name: Option<String>,
}

impl<'a> Visitor<'a> for FirstFunctionFinder {
    type Result = ControlFlow<()>;

    fn visit_function_definition(&mut self, f: &'a FunctionDefinition) -> Self::Result {
        // Extract function name from declarator
        if let Some(name) = extract_declarator_name(&f.declarator) {
            self.found_name = Some(name.clone());
            return ControlFlow::Break(());
        }
        ControlFlow::Continue(())
    }
}

fn extract_declarator_name(d: &Declarator) -> Option<String> {
    match d {
        Declarator::Direct(dd) => extract_direct_declarator_name(dd),
        Declarator::Pointer { declarator, .. } => extract_declarator_name(declarator),
        Declarator::Error => None,
    }
}

fn extract_direct_declarator_name(dd: &DirectDeclarator) -> Option<String> {
    match dd {
        DirectDeclarator::Identifier { identifier, .. } => Some(identifier.0.clone()),
        DirectDeclarator::Parenthesized(d) => extract_declarator_name(d),
        DirectDeclarator::Array { declarator, .. } => extract_direct_declarator_name(declarator),
        DirectDeclarator::Function { declarator, .. } => extract_direct_declarator_name(declarator),
    }
}

#[test]
fn test_break_after_first_function() {
    let code = r#"
        void first() {}
        void second() {}
        void third() {}
    "#;
    let ast = parse_c(code);
    let mut visitor = FirstFunctionFinder { found_name: None };
    let _ = visitor.visit_translation_unit(&ast);

    assert_eq!(visitor.found_name.as_deref(), Some("first"));
}

// ============================================================================
// Comprehensive Integration Test
// ============================================================================

#[test]
fn test_comprehensive_visitor() {
    let code = r#"
        typedef int Int;

        struct Point {
            int x;
            int y;
        };

        enum Color {
            RED,
            GREEN,
            BLUE
        };

        Int add(Int a, Int b) {
            return a + b;
        }

        void use_struct() {
            struct Point p;
            p.x = 10;
            p.y = 20;

            if (p.x > 0) {
                goto end;
            }

            end:
            return;
        }
    "#;

    let ast = parse_c(code);
    let mut visitor = IdentifierCollector::default();
    visitor.visit_translation_unit(&ast);

    // Verify we collected the right types of identifiers
    assert!(visitor.type_names.contains(&"Int".to_string()));
    assert!(visitor.struct_names.contains(&"Point".to_string()));
    assert!(visitor.enum_names.contains(&"Color".to_string()));
    assert_eq!(visitor.enumerators, vec!["RED", "GREEN", "BLUE"]);
    assert!(visitor.variables.contains(&"add".to_string()));
    assert!(visitor.variables.contains(&"use_struct".to_string()));
    assert!(visitor.members.contains(&"x".to_string()));
    assert!(visitor.members.contains(&"y".to_string()));
    assert_eq!(visitor.labels, vec!["end", "end"]); // goto and label definition
}
