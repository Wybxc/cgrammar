#![cfg(feature = "printer")]

use std::{io::Write, path::PathBuf};

use cgrammar::{
    printer::Context,
    visitor::{Visitor, VisitorMut, walk_declaration_mut, walk_declarator_mut, walk_direct_declarator_mut, walk_expression_mut, walk_statement_mut},
    *,
};
use elegance::Printer;
use pretty_assertions::assert_eq;
use rstest::rstest;

// Helper function to parse C code
fn parse_c(code: &str) -> TranslationUnit {
    let (tokens, _) = lex(code, None);
    let parser = translation_unit();
    let result = parser.parse(tokens.as_input());
    result.output().unwrap().clone()
}

// Helper function to print AST to string
fn print_ast(ast: &TranslationUnit) -> String {
    let mut printer = Printer::new_extra(String::new(), 80, Context::default());
    printer.visit_translation_unit(ast).unwrap();
    printer.finish().unwrap()
}

fn remove_spans(tokens: &mut BalancedTokenSequence) {
    tokens.eoi = Default::default();
    for token in &mut tokens.tokens {
        token.span = Default::default();
        match &mut token.value {
            BalancedToken::Parenthesized(tokens) | BalancedToken::Bracketed(tokens) | BalancedToken::Braced(tokens) => {
                remove_spans(tokens);
            }
            _ => {}
        }
    }
}

struct RemoveSpans;

impl VisitorMut<'_> for RemoveSpans {
    type Result = ();

    fn visit_attribute_mut(&mut self, attr: &'_ mut Attribute) -> Self::Result {
        if let Some(tokens) = attr.arguments.as_mut() {
            remove_spans(tokens);
        }
    }

    fn visit_expression_mut(&mut self, e: &'_ mut Expression) -> Self::Result {
        walk_expression_mut(self, e);
        e.span = Default::default();
    }

    fn visit_statement_mut(&mut self, s: &'_ mut Statement) -> Self::Result {
        walk_statement_mut(self, s);
        s.span = Default::default();
    }

    fn visit_declaration_mut(&mut self, d: &'_ mut Declaration) -> Self::Result {
        walk_declaration_mut(self, d);
        d.span = Default::default();
    }
}

struct RemoveParens;

impl VisitorMut<'_> for RemoveParens {
    type Result = ();

    fn visit_expression_mut(&mut self, e: &'_ mut Expression) -> Self::Result {
        walk_expression_mut(self, e);
        if let ExpressionKind::Postfix(PostfixExpression::Primary(PrimaryExpression::Parenthesized(inner))) = &mut e.kind {
            let inner = std::mem::replace(inner.as_mut(), Expression::dummy(ExpressionKind::Error));
            *e = inner;
        }
    }

    fn visit_declarator_mut(&mut self, d: &'_ mut Declarator) -> Self::Result {
        walk_declarator_mut(self, d);
        if let Declarator::Direct(DirectDeclarator::Parenthesized(inner)) = d {
            let inner = std::mem::replace(inner.as_mut(), Declarator::Error);
            *d = inner;
        }
    }

    fn visit_direct_declarator_mut(&mut self, d: &'_ mut DirectDeclarator) -> Self::Result {
        walk_direct_declarator_mut(self, d);
        if let DirectDeclarator::Parenthesized(boxed) = d {
            let inner = std::mem::replace(boxed.as_mut(), Declarator::Error);
            if let Declarator::Direct(dd) = inner {
                *d = dd;
            } else {
                *boxed.as_mut() = inner;
            }
        }
    }
}

// ============================================================================
// Round-trip Tests (Parse -> Print -> Parse)
// ============================================================================

// Helper function to perform roundtrip parsing and verify AST is valid
fn verify_roundtrip(code: &str) {
    // First parse
    let mut ast1 = parse_c(code);
    RemoveSpans.visit_translation_unit_mut(&mut ast1);
    RemoveParens.visit_translation_unit_mut(&mut ast1);

    // Print
    let printed = print_ast(&ast1);

    // Re-tokenize and parse the printed output
    let mut ast2 = parse_c(&printed);
    RemoveSpans.visit_translation_unit_mut(&mut ast2);
    RemoveParens.visit_translation_unit_mut(&mut ast2);

    // Verify that the ASTs are equivalent
    assert_eq!(ast1, ast2);
}

#[rstest]
fn test_printer(#[files("tests/test-cases/unit-tests/**/*.c")] path: PathBuf) {
    use std::path::Path;

    const FAILED_TESTS: &str = "tests/failed-tests.txt";
    let path = pathdiff::diff_paths(path, Path::new(".").canonicalize().unwrap()).unwrap();
    if std::fs::read_to_string(FAILED_TESTS)
        .unwrap_or_default()
        .contains(path.to_string_lossy().as_ref())
    {
        println!("Skipping already failed test: {}", path.to_string_lossy());
        return;
    }

    let input = std::fs::read_to_string(&path).unwrap();
    let mut prepocessor = std::process::Command::new("cc")
        .args([
            "-E",
            "-C",
            "-x",
            "c",
            "--std=c2x",
            "-D__extension__=",
            "-U__GNUC__",
            "-",
        ])
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .spawn()
        .unwrap();
    prepocessor.stdin.as_mut().unwrap().write_all(input.as_bytes()).unwrap();
    let output = prepocessor.wait_with_output().unwrap();
    let input = String::from_utf8(output.stdout).unwrap();

    verify_roundtrip(&input);
}
