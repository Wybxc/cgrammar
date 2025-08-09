use cgrammar::*;
use chumsky::Parser;

fn main() {
    let src = std::fs::read_to_string(std::env::args().nth(1).unwrap()).unwrap();

    let lexer = balanced_token_sequence();
    let input = lexer.parse(src.as_str()).unwrap();

    let parser = translation_unit();
    let ast = parser.parse(&input.0);
    if ast.has_output() {
        println!("{}", dbg_pls::pretty(&ast.output().unwrap()));
    }
    if ast.has_errors() {
        for error in ast.errors() {
            println!("{error:?}");
        }
        std::process::exit(1);
    }
}
