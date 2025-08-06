use cgrammar::*;
use chumsky::Parser;

fn main() {
    let src = std::fs::read_to_string(std::env::args().nth(1).unwrap()).unwrap();

    let lexer = balanced_token_sequence();
    let input = lexer.parse(src.as_str()).unwrap();

    let parser = expression();
    let ast = parser.parse(&input.tokens).unwrap();

    println!("{ast:?}");
}
