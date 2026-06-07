use std::{
    io::Write,
    path::{Path, PathBuf},
};

use cgrammar::*;
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

    let (tokens, _) = lex(&input, None);

    let mut state = ParseState::new();
    state.green.set_source_len(input.len() as u32);
    let parser = translation_unit();
    let result = parser.parse_with_state(tokens.as_input(), &mut state);
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

    // Verify lossless round-trip (skip files with only line directives)
    let tree = SyntaxTree::new(state.green.build());
    let reconstructed = print_lossless(&tree, &input);
    let has_code = input.lines().any(|l| {
        let t = l.trim();
        !t.starts_with('#') && !t.is_empty() && !t.starts_with("//") && !t.starts_with("/*")
    });
    if has_code && reconstructed != input {
        let mut file = std::fs::OpenOptions::new()
            .append(true)
            .create(true)
            .open(FAILED_TESTS)
            .unwrap();
        writeln!(file, "{} (round-trip)", path.to_string_lossy()).unwrap();
        panic!("Round-trip mismatch: expected {input:?}, got {reconstructed:?}");
    }
}
