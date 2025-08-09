use cgrammar::*;
use chumsky::prelude::*;
use rstest::rstest;

#[rstest]
#[case("int a = (1 1);")]
#[case("int a = b[?];")]
#[case("int a = sizeof();")]
#[case("int a = (*int);")]
#[case("int a = (*int)1;")]
#[case("_BitInt() a;")]
#[case("_Atomic(*int) a;")]
#[case("typeof(*int) a;")]
#[case("alignas(*int) float a[4];")]
#[case("int (*p 1);")]
#[case("int a[f f];")]
#[case("int a[1] = {[?]=1};")]
fn test_error_recovery(#[case] input: String) {
    let input = {
        let lexer = balanced_token_sequence();
        lexer.parse(&input).unwrap()
    };
    let output = {
        let parser = translation_unit();
        parser.parse(&input.0)
    };
    assert!(output.has_output());
    assert!(output.has_errors());
}
