use cgrammar::*;
use chumsky::Parser;

fn main() {
    let src = std::fs::read_to_string(std::env::args().nth(1).unwrap()).unwrap();

    let lexer = balanced_token_sequence();
    let input = lexer.parse(src.as_str()).unwrap();

    let parser = attribute_specifier_sequence();
    let ast = parser.parse(&input.tokens);
    if ast.has_errors() {
        for error in ast.errors() {
            println!("{error:?}");
        }
        std::process::exit(1);
    } else {
        println!("{}", dbg_pls::pretty(&ast.unwrap()));
    }
}
