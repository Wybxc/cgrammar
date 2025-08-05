use cgrammar::*;
use chumsky::prelude::*;
use rstest::rstest;
use std::path::PathBuf;

#[rstest]
fn test_lexer(#[files("tests/test-cases/**/*.c")] path: PathBuf) {
    let input = std::fs::read_to_string(&path).unwrap();
    let lexer = balanced_token_sequence();

    let result = lexer.parse(&input);
    if result.has_errors() {
        for error in result.errors() {
            println!("{error:?}");
        }
        panic!("Parsing failed with errors: {}", path.display());
    }
}
