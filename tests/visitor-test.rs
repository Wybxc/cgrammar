use std::ops::ControlFlow;

use cgrammar::{visitor::*, *};
use rstest::rstest;

// Helper function to parse C code
fn parse_c(code: &str) -> TranslationUnit {
    let (tokens, _) = lex(code, None);
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
        self.variables.push(id.0.to_string());
    }

    fn visit_type_name_identifier(&mut self, id: &'a Identifier) {
        self.type_names.push(id.0.to_string());
    }

    fn visit_struct_name(&mut self, id: &'a Identifier) {
        self.struct_names.push(id.0.to_string());
    }

    fn visit_enum_name(&mut self, id: &'a Identifier) {
        self.enum_names.push(id.0.to_string());
    }

    fn visit_enumerator_name(&mut self, id: &'a Identifier) {
        self.enumerators.push(id.0.to_string());
    }

    fn visit_label_name(&mut self, id: &'a Identifier) {
        self.labels.push(id.0.to_string());
    }

    fn visit_member_name(&mut self, id: &'a Identifier) {
        self.members.push(id.0.to_string());
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
        if id.0.as_ref() == self.target {
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
            ExternalDeclaration::Declaration(decl) => match &decl.kind {
                DeclarationKind::Normal { .. } => {
                    self.normal_declarations += 1;
                }
                DeclarationKind::Typedef { .. } => {
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
        DirectDeclarator::Identifier { identifier, .. } => Some(identifier.0.to_string()),
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

// ============================================================================
// Mutable Visitor Tests
// ============================================================================

/// A mutable visitor that renames all variables from old_name to new_name
struct VariableRenamer {
    old_name: String,
    new_name: String,
    rename_count: usize,
}

impl VariableRenamer {
    fn new(old_name: &str, new_name: &str) -> Self {
        Self {
            old_name: old_name.to_string(),
            new_name: new_name.to_string(),
            rename_count: 0,
        }
    }
}

impl<'a> VisitorMut<'a> for VariableRenamer {
    type Result = ();

    fn visit_variable_name_mut(&mut self, id: &'a mut Identifier) {
        if id.0.as_ref() == self.old_name {
            id.0 = self.new_name.as_str().into();
            self.rename_count += 1;
        }
    }
}

#[test]
fn test_variable_renaming() {
    let code = "int x; int y; void f() { x = y + x; }";
    let mut ast = parse_c(code);
    let mut visitor = VariableRenamer::new("x", "renamed_x");
    visitor.visit_translation_unit_mut(&mut ast);

    assert_eq!(visitor.rename_count, 3); // declaration + 2 uses

    // Verify the rename worked by collecting identifiers
    let mut collector = IdentifierCollector::default();
    collector.visit_translation_unit(&ast);
    assert!(collector.variables.contains(&"renamed_x".to_string()));
    assert!(
        !collector.variables.contains(&"x".to_string())
            || collector.variables.iter().filter(|v| *v == "x").count() == 0
    );
}

#[test]
fn test_rename_multiple_occurrences() {
    let code = "int a, a, a; void f(int a) { int a; }";
    let mut ast = parse_c(code);
    let mut visitor = VariableRenamer::new("a", "b");
    visitor.visit_translation_unit_mut(&mut ast);

    assert_eq!(visitor.rename_count, 5);

    let mut collector = IdentifierCollector::default();
    collector.visit_translation_unit(&ast);
    assert_eq!(collector.variables.iter().filter(|v| *v == "b").count(), 5);
}

/// A mutable visitor that prefixes all struct names
struct StructNamePrefixer {
    prefix: String,
    count: usize,
}

impl StructNamePrefixer {
    fn new(prefix: &str) -> Self {
        Self { prefix: prefix.to_string(), count: 0 }
    }
}

impl<'a> VisitorMut<'a> for StructNamePrefixer {
    type Result = ();

    fn visit_struct_name_mut(&mut self, id: &'a mut Identifier) {
        id.0 = format!("{}{}", self.prefix, id.0).into();
        self.count += 1;
    }
}

#[test]
fn test_struct_name_prefixing() {
    let code = "struct Point { int x; }; struct Point p;";
    let mut ast = parse_c(code);
    let mut visitor = StructNamePrefixer::new("My");
    visitor.visit_translation_unit_mut(&mut ast);

    assert_eq!(visitor.count, 2);

    let mut collector = IdentifierCollector::default();
    collector.visit_translation_unit(&ast);
    assert!(collector.struct_names.iter().all(|n| n == "MyPoint"));
}

/// A mutable visitor that renames enum constants
struct EnumConstantRenamer {
    mapping: std::collections::HashMap<String, String>,
    rename_count: usize,
}

impl EnumConstantRenamer {
    fn new() -> Self {
        let mut mapping = std::collections::HashMap::new();
        mapping.insert("RED".to_string(), "COLOR_RED".to_string());
        mapping.insert("GREEN".to_string(), "COLOR_GREEN".to_string());
        mapping.insert("BLUE".to_string(), "COLOR_BLUE".to_string());
        Self { mapping, rename_count: 0 }
    }
}

impl<'a> VisitorMut<'a> for EnumConstantRenamer {
    type Result = ();

    fn visit_enumerator_name_mut(&mut self, id: &'a mut Identifier) {
        if let Some(new_name) = self.mapping.get(id.0.as_ref()) {
            id.0 = new_name.as_str().into();
            self.rename_count += 1;
        }
    }

    fn visit_enum_constant_mut(&mut self, id: &'a mut Identifier) {
        if let Some(new_name) = self.mapping.get(id.0.as_ref()) {
            id.0 = new_name.as_str().into();
            self.rename_count += 1;
        }
    }
}

#[test]
fn test_enum_constant_renaming() {
    let code = r#"
        enum Color { RED, GREEN, BLUE };
        void f() {
            enum Color c = RED;
        }
    "#;
    let mut ast = parse_c(code);
    let mut visitor = EnumConstantRenamer::new();
    visitor.visit_translation_unit_mut(&mut ast);

    // 3 in definition, usage may or may not be counted depending on parser
    assert!(visitor.rename_count >= 3);

    let mut collector = IdentifierCollector::default();
    collector.visit_translation_unit(&ast);
    assert_eq!(collector.enumerators, vec!["COLOR_RED", "COLOR_GREEN", "COLOR_BLUE"]);
}

/// A mutable visitor that uppercases all member names
struct MemberNameUppercaser {
    count: usize,
}

impl<'a> VisitorMut<'a> for MemberNameUppercaser {
    type Result = ();

    fn visit_member_name_mut(&mut self, id: &'a mut Identifier) {
        id.0 = id.0.to_uppercase().into();
        self.count += 1;
    }
}

#[test]
fn test_member_name_uppercasing() {
    let code = r#"
        struct Point { int x; int y; };
        void f() {
            struct Point p;
            p.x = 1;
            p.y = 2;
        }
    "#;
    let mut ast = parse_c(code);
    let mut visitor = MemberNameUppercaser { count: 0 };
    visitor.visit_translation_unit_mut(&mut ast);

    assert!(visitor.count >= 2);

    let mut collector = IdentifierCollector::default();
    collector.visit_translation_unit(&ast);
    assert!(collector.members.contains(&"X".to_string()));
    assert!(collector.members.contains(&"Y".to_string()));
}

/// A mutable visitor that counts modifications with early termination
struct LimitedRenamer {
    old_name: String,
    new_name: String,
    max_renames: usize,
    rename_count: usize,
}

impl LimitedRenamer {
    fn new(old_name: &str, new_name: &str, max_renames: usize) -> Self {
        Self {
            old_name: old_name.to_string(),
            new_name: new_name.to_string(),
            max_renames,
            rename_count: 0,
        }
    }
}

impl<'a> VisitorMut<'a> for LimitedRenamer {
    type Result = ControlFlow<()>;

    fn visit_variable_name_mut(&mut self, id: &'a mut Identifier) -> Self::Result {
        if id.0.as_ref() == self.old_name {
            id.0 = self.new_name.as_str().into();
            self.rename_count += 1;
            if self.rename_count >= self.max_renames {
                return ControlFlow::Break(());
            }
        }
        ControlFlow::Continue(())
    }
}

#[test]
fn test_limited_renaming_with_early_termination() {
    let code = "int x, x, x, x, x;";
    let mut ast = parse_c(code);
    let mut visitor = LimitedRenamer::new("x", "y", 3);
    let _ = visitor.visit_translation_unit_mut(&mut ast);

    assert_eq!(visitor.rename_count, 3);

    // Verify that exactly 3 were renamed
    let mut collector = IdentifierCollector::default();
    collector.visit_translation_unit(&ast);
    let y_count = collector.variables.iter().filter(|v| *v == "y").count();
    let x_count = collector.variables.iter().filter(|v| *v == "x").count();
    assert_eq!(y_count, 3);
    assert_eq!(x_count, 2);
}

/// A mutable visitor that modifies typedef names
struct TypedefRenamer {
    old_name: String,
    new_name: String,
    count: usize,
}

impl TypedefRenamer {
    fn new(old_name: &str, new_name: &str) -> Self {
        Self {
            old_name: old_name.to_string(),
            new_name: new_name.to_string(),
            count: 0,
        }
    }
}

impl<'a> VisitorMut<'a> for TypedefRenamer {
    type Result = ();

    fn visit_type_name_identifier_mut(&mut self, id: &'a mut Identifier) {
        if id.0.as_ref() == self.old_name {
            id.0 = self.new_name.as_str().into();
            self.count += 1;
        }
    }

    fn visit_variable_name_mut(&mut self, id: &'a mut Identifier) {
        // Also rename in typedef declaration
        if id.0.as_ref() == self.old_name {
            id.0 = self.new_name.as_str().into();
            self.count += 1;
        }
    }
}

#[test]
fn test_typedef_renaming() {
    let code = "typedef int MyInt; MyInt x; MyInt y;";
    let mut ast = parse_c(code);
    let mut visitor = TypedefRenamer::new("MyInt", "Integer");
    visitor.visit_translation_unit_mut(&mut ast);

    assert!(visitor.count >= 3); // typedef name + 2 uses

    let mut collector = IdentifierCollector::default();
    collector.visit_translation_unit(&ast);
    assert!(collector.type_names.contains(&"Integer".to_string()));
    assert!(collector.variables.contains(&"Integer".to_string()));
}

/// A mutable visitor that renames labels
struct LabelRenamer {
    old_name: String,
    new_name: String,
    count: usize,
}

impl LabelRenamer {
    fn new(old_name: &str, new_name: &str) -> Self {
        Self {
            old_name: old_name.to_string(),
            new_name: new_name.to_string(),
            count: 0,
        }
    }
}

impl<'a> VisitorMut<'a> for LabelRenamer {
    type Result = ();

    fn visit_label_name_mut(&mut self, id: &'a mut Identifier) {
        if id.0.as_ref() == self.old_name {
            id.0 = self.new_name.as_str().into();
            self.count += 1;
        }
    }
}

#[test]
fn test_label_renaming() {
    let code = r#"
        void f() {
            goto end;
            end: return;
        }
    "#;
    let mut ast = parse_c(code);
    let mut visitor = LabelRenamer::new("end", "exit_point");
    visitor.visit_translation_unit_mut(&mut ast);

    assert_eq!(visitor.count, 2); // goto target + label definition

    let mut collector = IdentifierCollector::default();
    collector.visit_translation_unit(&ast);
    assert!(collector.labels.iter().all(|l| l == "exit_point"));
}

/// A mutable visitor that counts variables (read-only operation on mutable
/// visitor)
struct MutableVariableCounter {
    count: usize,
}

impl<'a> VisitorMut<'a> for MutableVariableCounter {
    type Result = ();

    fn visit_variable_name_mut(&mut self, _: &'a mut Identifier) {
        self.count += 1;
    }
}

#[test]
fn test_mutable_visitor_counting() {
    let code = "int x, y, z; void f(int a) { int b; }";
    let mut ast = parse_c(code);
    let mut visitor = MutableVariableCounter { count: 0 };
    visitor.visit_translation_unit_mut(&mut ast);

    assert_eq!(visitor.count, 6); // x, y, z, f, a, b
}

/// A comprehensive mutable visitor that performs multiple transformations
struct ComprehensiveTransformer {
    variable_prefix: String,
    struct_suffix: String,
    modifications: usize,
}

impl ComprehensiveTransformer {
    fn new(variable_prefix: &str, struct_suffix: &str) -> Self {
        Self {
            variable_prefix: variable_prefix.to_string(),
            struct_suffix: struct_suffix.to_string(),
            modifications: 0,
        }
    }
}

impl<'a> VisitorMut<'a> for ComprehensiveTransformer {
    type Result = ();

    fn visit_variable_name_mut(&mut self, id: &'a mut Identifier) {
        if !id.0.starts_with(&self.variable_prefix) {
            id.0 = format!("{}{}", self.variable_prefix, id.0).into();
            self.modifications += 1;
        }
    }

    fn visit_struct_name_mut(&mut self, id: &'a mut Identifier) {
        if !id.0.ends_with(&self.struct_suffix) {
            id.0 = format!("{}{}", id.0, self.struct_suffix).into();
            self.modifications += 1;
        }
    }
}

#[test]
fn test_comprehensive_transformation() {
    let code = r#"
        struct Point { int x; };
        void f() {
            struct Point p;
        }
    "#;
    let mut ast = parse_c(code);
    let mut visitor = ComprehensiveTransformer::new("var_", "_t");
    visitor.visit_translation_unit_mut(&mut ast);

    assert!(visitor.modifications > 0);

    let mut collector = IdentifierCollector::default();
    collector.visit_translation_unit(&ast);

    // Check that transformations were applied
    assert!(collector.variables.iter().any(|v| v.starts_with("var_")));
    assert!(collector.struct_names.iter().any(|s| s.ends_with("_t")));
}

#[test]
fn test_mutable_visitor_with_complex_code() {
    let code = r#"
        typedef int Int;
        struct Data { int value; };

        Int process(Int x) {
            struct Data d;
            d.value = x;
            return d.value;
        }
    "#;
    let mut ast = parse_c(code);
    let mut visitor = VariableRenamer::new("x", "input");
    visitor.visit_translation_unit_mut(&mut ast);

    assert_eq!(visitor.rename_count, 2); // parameter + use

    let mut collector = IdentifierCollector::default();
    collector.visit_translation_unit(&ast);
    assert!(collector.variables.contains(&"input".to_string()));
    assert!(!collector.variables.iter().any(|v| v == "x"));
}

/// Test that mutable visitor can work with Result type
struct FallibleRenamer {
    old_name: String,
    new_name: String,
    max_renames: usize,
    rename_count: usize,
}

impl FallibleRenamer {
    fn new(old_name: &str, new_name: &str, max_renames: usize) -> Self {
        Self {
            old_name: old_name.to_string(),
            new_name: new_name.to_string(),
            max_renames,
            rename_count: 0,
        }
    }
}

impl<'a> VisitorMut<'a> for FallibleRenamer {
    type Result = Result<(), String>;

    fn visit_variable_name_mut(&mut self, id: &'a mut Identifier) -> Self::Result {
        if id.0.as_ref() == self.old_name {
            if self.rename_count >= self.max_renames {
                return Err(format!("Maximum renames ({}) exceeded", self.max_renames));
            }
            id.0 = self.new_name.as_str().into();
            self.rename_count += 1;
        }
        Ok(())
    }
}

#[test]
fn test_fallible_renamer_success() {
    let code = "int x, x;";
    let mut ast = parse_c(code);
    let mut visitor = FallibleRenamer::new("x", "y", 5);
    let result = visitor.visit_translation_unit_mut(&mut ast);

    assert!(result.is_ok());
    assert_eq!(visitor.rename_count, 2);
}

#[test]
fn test_fallible_renamer_failure() {
    let code = "int x, x, x, x;";
    let mut ast = parse_c(code);
    let mut visitor = FallibleRenamer::new("x", "y", 2);
    let result = visitor.visit_translation_unit_mut(&mut ast);

    assert!(result.is_err());
    assert_eq!(visitor.rename_count, 2);
}
