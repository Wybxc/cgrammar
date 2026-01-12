//! Dump the AST of a C source file.
//!
//! Usage: `cargo run --example pretty_printer --all-features --
//! path/to/source.c`

#[cfg(feature = "printer")]
fn main() {
    use cgrammar::*;
    use chumsky::Parser;

    let file = std::env::args().nth(1).unwrap();
    let src = std::fs::read_to_string(file.as_str()).unwrap();

    let lexer = balanced_token_sequence();
    let mut lexer_state = LexerState::new(Some(&file));
    let tokens = lexer.parse_with_state(src.as_str(), &mut lexer_state);
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
        let mut pp = elegance::Printer::new(String::new(), 80);
        pp.visit_translation_unit(&ast.output().unwrap());
        println!("{}", pp.finish().unwrap());
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

#[cfg(not(feature = "printer"))]
fn main() {
    println!("Please run with --features printer");
}
