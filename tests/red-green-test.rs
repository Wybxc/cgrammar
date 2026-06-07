//! End-to-end tests for the red-green tree pipeline.

use cgrammar::*;
use chumsky::Parser;

fn parse_source(source: &str) -> SyntaxTree {
    let (tokens, _) = lex(source, Some("test.c"));
    let mut state = ParseState::new();
    state.ctx_mut().add_typedef_name("term".into());
    state.ctx_mut().add_typedef_name("thm".into());
    let result = translation_unit().parse_with_state(tokens.as_input(), &mut state);
    assert!(result.has_output(), "parse should succeed: {source}");
    SyntaxTree::new(state.green.build())
}

#[test]
fn test_red_green_pipeline() {
    let tree = parse_source("int main(int argc, char *argv[]) { return 0; }");
    let root = tree.root();
    assert_eq!(tree.kind(root), SyntaxKind::TranslationUnit);
    assert!(tree.node_count() > 1);
    assert!(tree.children(root).count() > 0);
}

#[test]
fn test_empty_translation_unit() {
    let tree = parse_source("");
    assert_eq!(tree.kind(tree.root()), SyntaxKind::TranslationUnit);
    assert!(tree.node_count() >= 1);
}

#[test]
fn test_simple_declaration() {
    let tree = parse_source("int x;");
    assert!(tree.node_count() > 1);
    assert_eq!(tree.kind(tree.root()), SyntaxKind::TranslationUnit);
}

#[test]
fn test_compound_statement_structure() {
    let tree = parse_source("void f() { int a; int b; }");
    let mut kinds: Vec<SyntaxKind> = vec![];
    collect_kinds(&tree, tree.root(), &mut kinds);
    assert!(kinds.contains(&SyntaxKind::Ident));
    assert!(kinds.contains(&SyntaxKind::LeftBrace));
    assert!(kinds.contains(&SyntaxKind::RightBrace));
}

#[test]
fn test_binary_expression() {
    let tree = parse_source("int x = 1 + 2;");
    let mut kinds: Vec<SyntaxKind> = vec![];
    collect_kinds(&tree, tree.root(), &mut kinds);
    assert!(kinds.contains(&SyntaxKind::Plus));
    assert!(kinds.contains(&SyntaxKind::IntegerConst));
}

/// Verify key tokens appear in print_lossless output.
#[test]
fn test_lossless_token_content() {
    let source = "int x;";
    let (tokens, ctx_map) = lex(source, Some("test.c"));
    let mut state = ParseState::new();
    let result = translation_unit().parse_with_state(tokens.as_input(), &mut state);
    assert!(result.has_output());
    let tree = SyntaxTree::new(state.green.build());

    let reconstructed = print_lossless(&tree, ctx_map.source);
    assert_eq!(
        reconstructed, "int x;",
        "lossless print should reconstruct source exactly"
    );
}

/// Verify function body tokens.
#[test]
fn test_lossless_function_tokens() {
    let source = "int f(void){}";
    let (tokens, ctx_map) = lex(source, Some("test.c"));
    let mut state = ParseState::new();
    let result = translation_unit().parse_with_state(tokens.as_input(), &mut state);
    assert!(result.has_output());
    let tree = SyntaxTree::new(state.green.build());
    let reconstructed = print_lossless(&tree, ctx_map.source);
    assert_eq!(
        reconstructed, "int f(void){}",
        "lossless print should reconstruct source exactly"
    );
}

/// Round-trip with if/else.
#[test]
fn test_round_trip_if_else() {
    let source = "int f(int x) { if (x) { return 1; } else { return 0; } }";
    let (tokens, ctx_map) = lex(source, Some("test.c"));
    let mut state = ParseState::new();
    let result = translation_unit().parse_with_state(tokens.as_input(), &mut state);
    assert!(result.has_output());
    let tree = SyntaxTree::new(state.green.build());
    let reconstructed = print_lossless(&tree, ctx_map.source);
    assert_eq!(
        reconstructed, source,
        "lossless print should reconstruct source exactly"
    );
}

/// Round-trip with while loop.
#[test]
fn test_round_trip_while() {
    let source = "int f(int n) { int x = 0; while (x < n) { x = x + 1; } return x; }";
    let (tokens, ctx_map) = lex(source, Some("test.c"));
    let mut state = ParseState::new();
    let result = translation_unit().parse_with_state(tokens.as_input(), &mut state);
    assert!(result.has_output());
    let tree = SyntaxTree::new(state.green.build());
    let reconstructed = print_lossless(&tree, ctx_map.source);
    assert_eq!(
        reconstructed, source,
        "lossless print should reconstruct source exactly"
    );
}

/// Round-trip with for loop.
#[test]
fn test_round_trip_for() {
    let source = "int f() { int s = 0; for (int i = 0; i < 10; i = i + 1) { s = s + i; } return s; }";
    let (tokens, ctx_map) = lex(source, Some("test.c"));
    let mut state = ParseState::new();
    let result = translation_unit().parse_with_state(tokens.as_input(), &mut state);
    assert!(result.has_output());
    let tree = SyntaxTree::new(state.green.build());
    let reconstructed = print_lossless(&tree, ctx_map.source);
    assert_eq!(
        reconstructed, source,
        "lossless print should reconstruct source exactly"
    );
}

/// Round-trip with struct and pointer.
#[test]
fn test_round_trip_struct() {
    let source = "struct point { int x; int y; }; int f(struct point *p) { return p->x + p->y; }";
    let (tokens, ctx_map) = lex(source, Some("test.c"));
    let mut state = ParseState::new();
    let result = translation_unit().parse_with_state(tokens.as_input(), &mut state);
    assert!(result.has_output());
    let tree = SyntaxTree::new(state.green.build());
    let reconstructed = print_lossless(&tree, ctx_map.source);
    assert_eq!(
        reconstructed, source,
        "lossless print should reconstruct source exactly"
    );
}

/// Test convenience API parse_tree.
#[test]
fn test_convenience_api() {
    let (tree, ast) = parse_tree("int x;");
    assert!(ast.is_some());
    assert_eq!(tree.kind(tree.root()), SyntaxKind::TranslationUnit);
    assert!(tree.node_count() > 1);
}

/// Round-trip with comments preserved as trivia.
#[test]
fn test_round_trip_comments() {
    let source = "/* block comment */ int x; // line comment\nint y;\n";
    let (tree, ctx_map, _) = parse_tree_with_map(source);
    let reconstructed = print_lossless(&tree, ctx_map.source);
    assert_eq!(
        reconstructed, source,
        "comments and trailing newline should be preserved"
    );
}

/// Verify tree structure for a complete C function.
#[test]
fn test_tree_structure() {
    let source = "int max(int a, int b) { if (a > b) { return a; } return b; }";
    let (tree, _) = parse_tree(source);
    let root = tree.root();
    let mut kinds: Vec<SyntaxKind> = vec![];
    collect_kinds(&tree, root, &mut kinds);

    // Top-level structure
    assert!(kinds.contains(&SyntaxKind::TranslationUnit));
    assert!(kinds.contains(&SyntaxKind::ExternalDeclaration));
    assert!(kinds.contains(&SyntaxKind::FunctionDefinition));

    // Declaration components
    assert!(kinds.contains(&SyntaxKind::DeclarationSpecifiers));
    assert!(kinds.contains(&SyntaxKind::TypeSpecifier));
    assert!(kinds.contains(&SyntaxKind::Declarator));
    assert!(kinds.contains(&SyntaxKind::DirectDeclarator));
    assert!(kinds.contains(&SyntaxKind::ParameterTypeList));
    assert!(kinds.contains(&SyntaxKind::ParameterDeclaration));

    // Statement components
    assert!(kinds.contains(&SyntaxKind::CompoundStatement));
    assert!(kinds.contains(&SyntaxKind::SelectionStatement));
    assert!(kinds.contains(&SyntaxKind::IfStatement));
    assert!(kinds.contains(&SyntaxKind::JumpStatement));
    assert!(kinds.contains(&SyntaxKind::ReturnStatement));

    // Expression components
    assert!(kinds.contains(&SyntaxKind::Expr));
    assert!(kinds.contains(&SyntaxKind::BinaryExpr));

    // Tokens
    assert!(kinds.contains(&SyntaxKind::Ident));
    assert!(kinds.contains(&SyntaxKind::LeftParen));
    assert!(kinds.contains(&SyntaxKind::RightParen));
    assert!(kinds.contains(&SyntaxKind::LeftBrace));
    assert!(kinds.contains(&SyntaxKind::RightBrace));
}

fn collect_kinds(tree: &SyntaxTree, node: red::SyntaxNode, kinds: &mut Vec<SyntaxKind>) {
    kinds.push(tree.kind(node));
    for child in tree.children(node) {
        collect_kinds(tree, child, kinds);
    }
}
