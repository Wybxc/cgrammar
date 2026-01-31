#[cfg(all(feature = "quasi-quote", feature = "printer"))]
fn main() {
    use cgrammar::*;

    macro_rules! quote {
        ($parser:path : $code:expr) => {{
            let (tokens, _) = lex($code, None);
            $parser().parse(tokens.as_input()).unwrap()
        }};
        ($parser:path : $code:expr, $($name:ident => $value:expr),* $(,)?) => {{
            let (mut tokens, _) = lex($code, None);
            tokens.interpolate(&interpolate! { $($name => $value),* }).unwrap();
            $parser().parse(tokens.as_input()).unwrap()
        }};
    }

    let items = ["foo", "bar", "baz"];

    let ast = quote! {
        translation_unit:
        r#"
            int foo(const char* s[], int n);

            int main() {
                int @var = foo(@args, @len);
            }
        "#,
        var => Identifier("bar".into()),
        args => Expression::Postfix(PostfixExpression::CompoundLiteral(CompoundLiteral {
            storage_class_specifiers: vec![],
            type_name: quote!(type_name: "const char*[]"),
            initializer: BracedInitializer {
                initializers: items.into_iter().map(|s| {
                    quote!{designated_initializer: "@s", s => StringLiterals::from(s.to_string())}
                }).collect()
            }
        })),
        len => Constant::Integer((items.len() as i128).into()),
    };

    let mut pp = elegance::Printer::new_extra(String::new(), 80, Default::default());
    pp.visit_translation_unit(&ast).unwrap();
    println!("{}", pp.finish().unwrap())
}

#[cfg(not(all(feature = "quasi-quote", feature = "printer")))]
fn main() {
    println!("Please run with --features dbg-pls --features quasi-quote");
}
