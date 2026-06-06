//! End-to-end tests for the red-green tree pipeline.

use chumsky::Parser;
use cgrammar::*;

/// Parse a simple C function and verify the full pipeline:
/// lex → parse (with green builder) → green tree → red tree → navigation
#[test]
fn test_red_green_pipeline() {
    let source = "int main(int argc, char *argv[]) { return 0; }";

    // Step 1: Lex
    let (tokens, _ctx_map) = lex(source, Some("test.c"));

    // Step 2: Parse with green tree construction
    let parser = translation_unit();
    let mut parse_state = ParseState::new();
    parse_state.ctx_mut().add_typedef_name("term".into());
    parse_state.ctx_mut().add_typedef_name("thm".into());
    let result = parser.parse_with_state(tokens.as_input(), &mut parse_state);
    let ast = result.output().expect("parse should succeed");

    // Step 3: Extract green tree from parse state
    let green = parse_state.green.build();

    // Step 4: Build red tree
    let tree = SyntaxTree::new(green);

    // Step 5: Navigate
    let root = tree.root();
    assert_eq!(tree.kind(root), SyntaxKind::TranslationUnit);

    // Should have at least the root plus some children
    assert!(tree.node_count() > 1, "tree should have more than just root");

    // Check we can find a TranslationUnit
    assert!(tree.children(root).count() > 0, "translation unit should have children");
}

/// Test with an empty input — should still produce a green tree.
#[test]
fn test_empty_translation_unit() {
    let source = "";

    let (tokens, _ctx_map) = lex(source, Some("test.c"));
    let parser = translation_unit();
    let mut parse_state = ParseState::new();
    let result = parser.parse_with_state(tokens.as_input(), &mut parse_state);
    let _ast = result.output().expect("parse should succeed");

    let green = parse_state.green.build();
    let tree = SyntaxTree::new(green);

    let root = tree.root();
    assert_eq!(tree.kind(root), SyntaxKind::TranslationUnit);
    assert!(tree.node_count() >= 1);
}

/// Test with a simple declaration — verify token recording works.
#[test]
fn test_simple_declaration() {
    let source = "int x;";
    let (tokens, _ctx_map) = lex(source, Some("test.c"));
    let parser = translation_unit();
    let mut parse_state = ParseState::new();
    let result = parser.parse_with_state(tokens.as_input(), &mut parse_state);
    let _ast = result.output().expect("parse should succeed");

    let green = parse_state.green.build();
    let tree = SyntaxTree::new(green);

    // Just verify the tree is non-trivial
    assert!(tree.node_count() > 1, "should have more than just root node");
    let root = tree.root();
    assert_eq!(tree.kind(root), SyntaxKind::TranslationUnit);
}

/// Test with a compound statement — verify nested structures.
#[test]
fn test_compound_statement_structure() {
    let source = "void f() { int a; int b; }";
    let (tokens, _ctx_map) = lex(source, Some("test.c"));
    let parser = translation_unit();
    let mut parse_state = ParseState::new();
    let result = parser.parse_with_state(tokens.as_input(), &mut parse_state);
    let _ast = result.output().expect("parse should succeed");

    let green = parse_state.green.build();
    let tree = SyntaxTree::new(green);

    let root = tree.root();
    // Walk the tree and collect all node kinds
    let mut kinds: Vec<SyntaxKind> = vec![];
    collect_kinds(&tree, root, &mut kinds);

    // Verify we have key tokens including bracket tokens
    assert!(kinds.contains(&SyntaxKind::Ident), "should contain identifiers");
    assert!(kinds.contains(&SyntaxKind::LeftBrace), "should contain left brace");
    assert!(kinds.contains(&SyntaxKind::RightBrace), "should contain right brace");
}

/// Test with binary expression.
#[test]
fn test_binary_expression() {
    let source = "int x = 1 + 2;";
    let (tokens, _ctx_map) = lex(source, Some("test.c"));
    let parser = translation_unit();
    let mut parse_state = ParseState::new();
    let result = parser.parse_with_state(tokens.as_input(), &mut parse_state);
    let _ast = result.output().expect("parse should succeed");

    let green = parse_state.green.build();
    let tree = SyntaxTree::new(green);

    let root = tree.root();
    let mut kinds: Vec<SyntaxKind> = vec![];
    collect_kinds(&tree, root, &mut kinds);

    // Should have + token
    assert!(kinds.contains(&SyntaxKind::Plus), "should contain plus operator");
    // Should have integer constants
    assert!(kinds.contains(&SyntaxKind::IntegerConst), "should contain integer constants");
}

fn collect_kinds(tree: &SyntaxTree, node: red::SyntaxNode, kinds: &mut Vec<SyntaxKind>) {
    kinds.push(tree.kind(node));
    for child in tree.children(node) {
        collect_kinds(tree, child, kinds);
    }
}
