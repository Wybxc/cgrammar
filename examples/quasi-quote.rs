#[cfg(all(feature = "quasi-quote", feature = "printer"))]
fn main() {
    use cgrammar::*;

    macro_rules! quote {
        ($parser:path : $code:expr) => {{
            let tokens = balanced_token_sequence().parse($code).unwrap();
            $parser().parse(tokens.as_input()).unwrap()
        }};
        ($parser:path : $code:expr, $($name:ident => $value:expr),* $(,)?) => {{
            let mut tokens = balanced_token_sequence().parse($code).unwrap();
            tokens.interpolate(&interpolate! { $($name => $value),* }).unwrap();
            $parser().parse(tokens.as_input()).unwrap()
        }};
    }

    let items = ["foo", "bar", "baz"];

    let ast = quote! {
        translation_unit:
        r#"
            void foo(const char* s[], int n);

            int main() {
                foo(@args, @len);
            }
        "#,
        args => Expression::Postfix(PostfixExpression::CompoundLiteral(CompoundLiteral {
            storage_class_specifiers: vec![],
            type_name: quote!(type_name: "const char*[]"),
            initializer: BracedInitializer {
                initializers: items.into_iter().map(|s| DesignatedInitializer {
                    designation: None,
                    initializer: Initializer::Expression(Box::new(
                        quote!{expression: "@s", s => StringLiterals::from(s.to_string())}
                    ))
                }).collect()
            }
        })),
        len => Constant::Integer((items.len() as i128).into()),
    };

    let mut pp = elegance::Printer::new(String::new(), 80);
    pp.visit_translation_unit(&ast).unwrap();
    println!("{}", pp.finish().unwrap())
}

#[cfg(not(all(feature = "quasi-quote", feature = "printer")))]
fn main() {
    println!("Please run with --features dbg-pls --features quasi-quote");
}
