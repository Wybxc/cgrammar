use cgrammar::*;
use chumsky::prelude::*;

const SRC: &str = r#"
int a = (1 1);
int b = a[?];
"#;

#[test]
fn test_error_recovery() {
    let input = {
        let lexer = balanced_token_sequence();
        lexer.parse(SRC).unwrap()
    };
    let output = {
        let parser = translation_unit();
        parser.parse(&input.0)
    };
    assert!(output.has_output());
    assert!(output.has_errors());
}
