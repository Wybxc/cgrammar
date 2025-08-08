use cgrammar::*;
use chumsky::prelude::*;
use rstest::rstest;
use std::path::PathBuf;

#[rstest]
fn test_lexer(#[files("tests/test-cases/**/*.c")] path: PathBuf) {
    let input = std::fs::read_to_string(&path).unwrap();
    let lexer = balanced_token_sequence();

    println!("Testing lexer on: {}", path.display());
    let result = lexer.parse(&input);
    if result.has_errors() {
        for error in result.errors() {
            println!("{error:?}");
        }
        panic!("Parsing failed with errors");
    }
}

#[rstest]
fn test_parser(#[files("tests/test-cases/**/*.c")] path: PathBuf) {
    use std::io::Write;

    let input = std::fs::read_to_string(&path).unwrap();
    let mut prepocessor = std::process::Command::new("cc")
        .args(["-E", "-x", "c", "--std=c2x", "-"])
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .spawn()
        .unwrap();
    prepocessor.stdin.as_mut().unwrap().write_all(input.as_bytes()).unwrap();
    let output = prepocessor.wait_with_output().unwrap();
    let input = String::from_utf8(output.stdout).unwrap();

    let lexer = balanced_token_sequence();
    let input = lexer.parse(&input).unwrap();

    println!("Testing parser on: {}", path.display());
    let parser = translation_unit();
    let result = parser.parse(&input.0);
    if result.has_errors() {
        for error in result.errors() {
            println!("{error:?}");
        }
        panic!("Parsing failed with errors");
    }
}
