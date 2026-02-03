#![cfg(feature = "quasi-quote")]

use std::collections::HashMap;

use cgrammar::{quasi_quote::Interpolate, visitor::{VisitorMut, walk_expression_mut, walk_statement_mut, walk_declaration_mut}, *};
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

    fn visit_expression_mut(&mut self, e: &'_ mut Expression) -> Self::Result {
        walk_expression_mut(self, e);
        e.span = Default::default();
    }

    fn visit_statement_mut(&mut self, s: &'_ mut Statement) -> Self::Result {
        walk_statement_mut(self, s);
        s.span = Default::default();
    }

    fn visit_declaration_mut(&mut self, d: &'_ mut Declaration) -> Self::Result {
        walk_declaration_mut(self, d);
        d.span = Default::default();
    }
}

#[rstest]
#[case("int x = @v;", interpolate! { v => Constant::Integer(1.into()) }, "int x = 1;")]
fn test_interpolation(
    #[case] code: &str,
    #[case] mapping: HashMap<&'static str, Box<dyn Interpolate>>,
    #[case] target: &str,
) {
    let (mut tokens, _) = lex(code, None);
    tokens.interpolate(&mapping).unwrap();
    let mut ast1 = translation_unit().parse(tokens.as_input()).unwrap();
    RemoveSpans.visit_translation_unit_mut(&mut ast1);

    let (tokens, _) = lex(target, None);
    let mut ast2 = translation_unit().parse(tokens.as_input()).unwrap();
    RemoveSpans.visit_translation_unit_mut(&mut ast2);

    assert_eq!(ast1, ast2);
}
