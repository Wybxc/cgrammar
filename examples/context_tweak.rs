//! Dump the AST of a C source file (with context tweaking).
//!
//! Usage: `cargo run --example context_tweak --all-features -- path/to/source.c`

use cgrammar::*;

struct CustomTweaker;

impl ContextTweaker for CustomTweaker {
    fn new() -> Self {
        Self
    }

    fn init(&mut self, context: &mut Context) {
        context.add_typedef_name("term".into());
        context.add_typedef_name("thm".into());
        context.add_enum_constant("GN".into());
    }
}

fn main() {
    let src = std::fs::read_to_string(std::env::args().nth(1).unwrap()).unwrap();

    let tokens = CLexer::lex(src.as_str());
    if tokens.has_errors() {
        for error in tokens.errors() {
            println!("{}", error);
        }
    }
    let tokens = tokens.output().unwrap();

    let ast = CParserWithTweaker::<CustomTweaker>::parse(tokens);
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
