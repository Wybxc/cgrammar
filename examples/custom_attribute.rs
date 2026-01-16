//! Parse into custom attributes from a C source file.
//!
//! Usage: `cargo run --example custom_attribute --all-features --
//! path/to/source.c`

#[cfg(feature = "dbg-pls")]
fn main() {
    use cgrammar::*;
    use chumsky::Parser;

    let file = std::env::args().nth(1).unwrap();
    let src = std::fs::read_to_string(file.as_str()).unwrap();

    let mut init_state = State::new();
    init_state.ctx_mut().add_typedef_name("term".into());
    init_state.ctx_mut().add_typedef_name("thm".into());
    let init_state = init_state;

    let lex_result = lex(src.as_str(), Some(&file));
    if lex_result.has_errors() {
        for error in &lex_result.errors {
            println!("{}", error);
        }
    }
    let tokens = lex_result.output.as_ref().unwrap();

    let parser = translation_unit();
    let ast = parser.parse_with_state(tokens.as_input(), &mut init_state.clone());
    let (ast, errors) = ast.into_output_errors();

    for error in errors {
        report(error, &lex_result.contexts);
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
        .filter(|attr| attr.token.get_identifier("cst").is_some_and(|n| n.as_ref() == "gl"))
        .filter_map(|attr| attr.arguments.as_ref())
        .for_each(|gl| {
            let parser = statement();
            let parsed = parser.parse_with_state(gl.as_input(), &mut init_state.clone());
            let (stmt, errors) = parsed.into_output_errors();
            for error in errors {
                report(error, &lex_result.contexts);
            }
            let Some(stmt) = stmt else {
                eprintln!("Failed to parse gl statement");
                return;
            };
            println!("{}", dbg_pls::pretty(&stmt));
        });
}

#[cfg(not(feature = "dbg-pls"))]
fn main() {
    println!("Please run with --features dbg-pls");
}
