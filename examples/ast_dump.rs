use cgrammar::*;
use chumsky::Parser;

fn main() {
    let src = std::fs::read_to_string(std::env::args().nth(1).unwrap()).unwrap();

    let lexer = balanced_token_sequence();
    let tokens = lexer.parse(src.as_str()).unwrap();

    let parser = translation_unit();
    let ast = parser.parse(tokens.as_input());
    if ast.has_output() {
        println!("{}", dbg_pls::pretty(&ast.output().unwrap()));
    } else {
        eprintln!("Parse failed!");
    }
    if ast.has_errors() {
        let mut cache = FileCache::new(src.as_str().into());
        for error in ast.into_errors() {
            let report = report(error, &mut cache).unwrap();
            report.eprint(&mut cache).unwrap();
        }
        std::process::exit(1);
    }
}
