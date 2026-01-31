#![cfg(feature = "quasi-quote")]

use std::collections::HashMap;

use cgrammar::{quasi_quote::Interpolate, visitor::VisitorMut, *};
use rstest::rstest;

fn remove_spans(tokens: &mut BalancedTokenSequence) {
    tokens.eoi = Default::default();
    for token in &mut tokens.tokens {
        token.span = Default::default();
        match &mut token.value {
            BalancedToken::Parenthesized(tokens) | BalancedToken::Bracketed(tokens) | BalancedToken::Braced(tokens) => {
                remove_spans(tokens);
            }
            _ => {}
        }
    }
}

struct RemoveSpans;

impl VisitorMut<'_> for RemoveSpans {
    type Result = ();

    fn visit_attribute_mut(&mut self, attr: &'_ mut Attribute) -> Self::Result {
        if let Some(tokens) = attr.arguments.as_mut() {
            remove_spans(tokens);
        }
    }
}

#[rstest]
#[case("int x = @v;", interpolate! { v => Constant::Integer(1.into()) }, "int x = 1;")]
fn test_interpolation(
    #[case] code: &str,
    #[case] mapping: HashMap<&'static str, Box<dyn Interpolate>>,
    #[case] target: &str,
) {
    let (mut tokens, _) = lex(code, None).unwrap();
    tokens.interpolate(&mapping).unwrap();
    let mut ast1 = translation_unit().parse(tokens.as_input()).unwrap();
    RemoveSpans.visit_translation_unit_mut(&mut ast1);

    let (tokens, _) = lex(target, None).unwrap();
    let mut ast2 = translation_unit().parse(tokens.as_input()).unwrap();
    RemoveSpans.visit_translation_unit_mut(&mut ast2);

    assert_eq!(ast1, ast2);
}
