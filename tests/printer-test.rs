#![cfg(feature = "printer")]

use std::{io::Write, path::PathBuf};

use cgrammar::{visitor::Visitor, *};
use chumsky::prelude::*;
use elegance::Printer;
use rstest::rstest;

// Helper function to parse C code
fn parse_c(code: &str) -> TranslationUnit {
    let lexer = balanced_token_sequence();
    let tokens = lexer.parse(code).unwrap();
    let parser = translation_unit();
    let result = parser.parse(tokens.as_input());
    result.output().unwrap().clone()
}

// Helper function to print AST to string
fn print_ast(ast: &TranslationUnit) -> String {
    let mut printer = Printer::new(String::new(), 80);
    printer.visit_translation_unit(ast).unwrap();
    printer.finish().unwrap()
}

// ============================================================================
// Round-trip Tests (Parse -> Print -> Parse)
// ============================================================================

// Helper function to perform roundtrip parsing and verify AST is valid
fn verify_roundtrip(code: &str) {
    // First parse
    let ast1 = parse_c(code);

    // Print
    let printed = print_ast(&ast1);

    // Re-tokenize and parse the printed output
    let ast2 = parse_c(&printed);

    // Verify that the ASTs are equivalent
    assert_eq!(ast1, ast2);
}

#[rstest]
fn test_printer(#[files("tests/test-cases/unit-tests/**/*.c")] path: PathBuf) {
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
