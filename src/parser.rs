use crate::ast::*;
use chumsky::{
    pratt::{infix, left},
    prelude::*,
};

pub fn expression<'a>() -> impl Parser<'a, &'a [BalancedToken], Expression> + Clone {
    macro_rules! op {
        ($punct:expr) => {{
            use Punctuator::*;
            select! {
                BalancedToken::Punctuator(p) if p == $punct => ()
            }
        }};
    }

    macro_rules! binary {
        ($op:expr) => {
            |left, _, right, _| {
                use BinaryOperator::*;
                Expression::Binary(BinaryExpression {
                    operator: $op,
                    left: Box::new(left),
                    right: Box::new(right),
                })
            }
        };
    }

    // Suppose precedence is X, use 1000 - 10*X as the associativity level
    recursive(|expression| {
        primary_expression(expression)
            .map(Expression::Primary)
            .pratt((
                infix(left(1000 - 30), op!(Star), binary!(Multiply)),
                infix(left(1000 - 30), op!(Slash), binary!(Divide)),
                infix(left(1000 - 30), op!(Percent), binary!(Modulo)),
                infix(left(1000 - 40), op!(Plus), binary!(Add)),
                infix(left(1000 - 40), op!(Minus), binary!(Subtract)),
                infix(left(1000 - 50), op!(LeftShift), binary!(LeftShift)),
                infix(left(1000 - 50), op!(RightShift), binary!(RightShift)),
                infix(left(1000 - 60), op!(Less), binary!(Less)),
                infix(left(1000 - 60), op!(LessEqual), binary!(LessEqual)),
                infix(left(1000 - 60), op!(Greater), binary!(Greater)),
                infix(left(1000 - 60), op!(GreaterEqual), binary!(GreaterEqual)),
                infix(left(1000 - 70), op!(Equal), binary!(Equal)),
                infix(left(1000 - 70), op!(NotEqual), binary!(NotEqual)),
                infix(left(1000 - 80), op!(Ampersand), binary!(BitwiseAnd)),
                infix(left(1000 - 90), op!(Caret), binary!(BitwiseXor)),
                infix(left(1000 - 100), op!(Pipe), binary!(BitwiseOr)),
                infix(left(1000 - 110), op!(LogicalAnd), binary!(LogicalAnd)),
                infix(left(1000 - 120), op!(LogicalOr), binary!(LogicalOr)),
            ))
    })
}

pub fn primary_expression<'a>(
    expression: impl Parser<'a, &'a [BalancedToken], Expression> + Clone,
) -> impl Parser<'a, &'a [BalancedToken], PrimaryExpression> + Clone {
    let identifier = select! {
        BalancedToken::Identifier(value) => value
    };
    let constant = select! {
        BalancedToken::Constant(value) => value
    };
    let string_literal = select! {
        BalancedToken::StringLiteral(value) => value
    };
    let parens = select_ref! {
        BalancedToken::Parenthesized(BalancedTokenSequence { tokens } ) => tokens.as_slice()
    };

    choice((
        identifier.map(PrimaryExpression::Identifier),
        constant.map(PrimaryExpression::Constant),
        string_literal.map(PrimaryExpression::StringLiteral),
        expression
            .nested_in(parens)
            .map(Box::new)
            .map(PrimaryExpression::Parenthesized),
        // TODO: Generic selection
    ))
}
