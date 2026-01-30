//! Dump the AST of a C source file.
//!
//! Usage: `cargo run --example ast_dump --all-features -- path/to/source.c`

#[cfg(feature = "dbg-pls")]
fn main() {
    use cgrammar::*;
    use chumsky::Parser;

    let file = std::env::args().nth(1).unwrap();
    let src = std::fs::read_to_string(file.as_str()).unwrap();

    let lex_result = lex(src.as_str(), Some(&file));
    if lex_result.has_errors() {
        for report in lex_result.report_errors() {
            report.eprint(ariadne::Source::from(&src)).unwrap();
        }
    }
    let tokens = lex_result.output.as_ref().unwrap();

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
            report(error, &lex_result.contexts);
        }
        std::process::exit(1);
    }
}

#[cfg(not(feature = "dbg-pls"))]
fn main() {
    println!("Please run with --features dbg-pls");
}
