use std::{
    io::Write,
    path::{Path, PathBuf},
};

use cgrammar::*;
use chumsky::prelude::*;
use rstest::rstest;

#[rstest]
fn test_parser(#[files("tests/test-cases/**/*.c")] path: PathBuf) {
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
        .args(["-E", "-C", "-x", "c", "--std=c2x", "-D__extension__=", "-U__GNUC__", "-"])
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .spawn()
        .unwrap();
    prepocessor.stdin.as_mut().unwrap().write_all(input.as_bytes()).unwrap();
    let output = prepocessor.wait_with_output().unwrap();
    let input = String::from_utf8(output.stdout).unwrap();

    let lexer = balanced_token_sequence();
    let tokens = lexer.parse(&input).unwrap();

    let parser = translation_unit();
    let result = parser.parse(tokens.as_input());
    if result.has_errors() {
        let mut file = std::fs::OpenOptions::new()
            .append(true)
            .create(true)
            .open(FAILED_TESTS)
            .unwrap();
        writeln!(file, "{}", path.to_string_lossy()).unwrap();

        if std::env::var("GITHUB_ACTIONS").is_ok() {
            println!("::group::{}", path.to_string_lossy());
            println!("{}", input);
            println!("::endgroup::");
        }

        for error in result.errors() {
            println!("{error:?}");
        }
        panic!("Parsing failed with errors");
    }
}
