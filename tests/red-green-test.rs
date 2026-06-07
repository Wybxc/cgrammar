//! End-to-end tests for the red-green tree pipeline.

use cgrammar::*;

fn assert_roundtrip(source: &str) {
    let (tree, ctx_map) = parse_tree_with_map(source);
    let reconstructed = print_lossless(&tree, ctx_map.source);
    assert_eq!(
        reconstructed, source,
        "lossless print should reconstruct source exactly"
    );
}

#[test]
fn test_red_green_pipeline() {
    let tree = parse_tree("int main(int argc, char *argv[]) { return 0; }");
    let root = tree.root();
    assert_eq!(tree.kind(root), SyntaxKind::TranslationUnit);
    assert!(tree.node_count() > 1);
    assert!(tree.children(root).count() > 0);
}

#[test]
fn test_empty_translation_unit() {
    let tree = parse_tree("");
    assert_eq!(tree.kind(tree.root()), SyntaxKind::TranslationUnit);
    assert!(tree.node_count() >= 1);
}

#[test]
fn test_simple_declaration() {
    let tree = parse_tree("int x;");
    assert!(tree.node_count() > 1);
    assert_eq!(tree.kind(tree.root()), SyntaxKind::TranslationUnit);
}

#[test]
fn test_compound_statement_structure() {
    let tree = parse_tree("void f() { int a; int b; }");
    let mut kinds: Vec<SyntaxKind> = vec![];
    collect_kinds(&tree, tree.root(), &mut kinds);
    assert!(kinds.contains(&SyntaxKind::Ident));
    assert!(kinds.contains(&SyntaxKind::LeftBrace));
    assert!(kinds.contains(&SyntaxKind::RightBrace));
}

#[test]
fn test_binary_expression() {
    let tree = parse_tree("int x = 1 + 2;");
    let mut kinds: Vec<SyntaxKind> = vec![];
    collect_kinds(&tree, tree.root(), &mut kinds);
    assert!(kinds.contains(&SyntaxKind::Plus));
    assert!(kinds.contains(&SyntaxKind::IntegerConst));
}

#[test]
fn test_lossless_token_content() {
    assert_roundtrip("int x;");
}

#[test]
fn test_lossless_function_tokens() {
    assert_roundtrip("int f(void){}");
}

#[test]
fn test_round_trip_if_else() {
    assert_roundtrip("int f(int x) { if (x) { return 1; } else { return 0; } }");
}

#[test]
fn test_round_trip_while() {
    assert_roundtrip("int f(int n) { int x = 0; while (x < n) { x = x + 1; } return x; }");
}

#[test]
fn test_round_trip_for() {
    assert_roundtrip("int f() { int s = 0; for (int i = 0; i < 10; i = i + 1) { s = s + i; } return s; }");
}

#[test]
fn test_round_trip_struct() {
    assert_roundtrip("struct point { int x; int y; }; int f(struct point *p) { return p->x + p->y; }");
}

#[test]
fn test_convenience_api() {
    let tree = parse_tree("int x;");
    assert_eq!(tree.kind(tree.root()), SyntaxKind::TranslationUnit);
    assert!(tree.node_count() > 1);
}

#[test]
fn test_round_trip_comments() {
    let source = "/* block comment */ int x; // line comment\nint y;\n";
    let (tree, ctx_map) = parse_tree_with_map(source);
    let reconstructed = print_lossless(&tree, ctx_map.source);
    assert_eq!(
        reconstructed, source,
        "comments and trailing newline should be preserved"
    );
}

#[test]
fn test_tree_structure() {
    let source = "int max(int a, int b) { if (a > b) { return a; } return b; }";
    let tree = parse_tree(source);
    let root = tree.root();
    let mut kinds: Vec<SyntaxKind> = vec![];
    collect_kinds(&tree, root, &mut kinds);

    assert!(kinds.contains(&SyntaxKind::TranslationUnit));
    assert!(kinds.contains(&SyntaxKind::ExternalDeclaration));
    assert!(kinds.contains(&SyntaxKind::FunctionDefinition));
    assert!(kinds.contains(&SyntaxKind::DeclarationSpecifiers));
    assert!(kinds.contains(&SyntaxKind::TypeSpecifier));
    assert!(kinds.contains(&SyntaxKind::Declarator));
    assert!(kinds.contains(&SyntaxKind::CompoundStatement));
    assert!(kinds.contains(&SyntaxKind::IfStatement));
    assert!(kinds.contains(&SyntaxKind::ReturnStatement));
    assert!(kinds.contains(&SyntaxKind::BinaryExpr));
    assert!(kinds.contains(&SyntaxKind::Ident));
}

fn collect_kinds(tree: &SyntaxTree, node: SyntaxNode, kinds: &mut Vec<SyntaxKind>) {
    kinds.push(tree.kind(node));
    for child in tree.children(node) {
        collect_kinds(tree, child, kinds);
    }
}
