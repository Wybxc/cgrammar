//! Parse into custom attributes from a C source file.
//!
//! Usage: `cargo run --example custom_attribute --all-features -- path/to/source.c`

use cgrammar::*;
use chumsky::Parser;

fn main() {
    let src = std::fs::read_to_string(std::env::args().nth(1).unwrap()).unwrap();

    let mut init_state = State::new();
    init_state.ctx_mut().add_typedef_name("term".into());
    init_state.ctx_mut().add_typedef_name("thm".into());
    let init_state = init_state;

    let lexer = balanced_token_sequence();
    let tokens = lexer.parse(src.as_str());
    if tokens.has_errors() {
        for error in tokens.errors() {
            println!("{}", error);
        }
    }
    let tokens = tokens.output().unwrap();

    let parser = translation_unit();
    let ast = parser.parse_with_state(tokens.as_input(), &mut init_state.clone());
    let (ast, errors) = ast.into_output_errors();

    for error in errors {
        report(error);
    }

    let ast = ast.expect("Parse failed!");
    ast.external_declarations
        .iter()
        .filter_map(|decl| match decl {
            ExternalDeclaration::Declaration(Declaration::Attribute(attrs)) => Some(attrs),
            _ => None,
        })
        .flatten()
        .filter_map(|attr| match attr {
            AttributeSpecifier::Attributes(attrs) => Some(attrs),
            _ => None,
        })
        .flatten()
        .filter(|attr| attr.token.get_identifier("cst").is_some_and(|n| n.0 == "gl"))
        .filter_map(|attr| attr.arguments.as_ref())
        .for_each(|gl| {
            let parser = statement();
            let parsed = parser.parse_with_state(gl.as_input(), &mut init_state.clone());
            let (stmt, errors) = parsed.into_output_errors();
            for error in errors {
                report(error);
            }
            let Some(stmt) = stmt else {
                eprintln!("Failed to parse gl statement");
                return;
            };
            println!("{}", dbg_pls::pretty(&stmt));
        });
}
