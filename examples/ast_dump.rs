//! Dump the AST of a C source file.
//!
//! Usage: `cargo run --example ast_dump --all-features -- path/to/source.c`

use cgrammar::*;
use chumsky::Parser;

fn main() {
    let src = std::fs::read_to_string(std::env::args().nth(1).unwrap()).unwrap();

    let lexer = balanced_token_sequence();
    let tokens = lexer.parse(src.as_str());
    if tokens.has_errors() {
        for error in tokens.errors() {
            println!("{}", error);
        }
    }
    let tokens = tokens.output().unwrap();

    let parser = translation_unit();
    let mut init_state = State::new();
    init_state.ctx_mut().add_typedef_name("term".into());
    init_state.ctx_mut().add_typedef_name("thm".into());
    let ast = parser.parse_with_state(tokens.as_input(), &mut init_state);
    if ast.has_output() {
        println!("{}", dbg_pls::pretty(&ast.output().unwrap()));
    } else {
        eprintln!("Parse failed!");
    }
    if ast.has_errors() {
        for error in ast.into_errors() {
            report(error);
        }
        std::process::exit(1);
    }
}
