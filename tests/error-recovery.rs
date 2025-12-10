use cgrammar::*;
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
    let tokens = CLexer::lex(&input).unwrap();
    let result = CParser::parse(&tokens);
    assert!(result.has_output());
    assert!(result.has_errors());
}
