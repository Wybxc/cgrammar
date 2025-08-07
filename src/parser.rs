use crate::ast::*;
use chumsky::prelude::*;

type Extra<'a> = chumsky::extra::Err<Rich<'a, BalancedToken>>;

/// (6.5.1) primary expression
pub fn primary_expression<'a>(
    expression: impl Parser<'a, &'a [BalancedToken], Expression, Extra<'a>> + Clone,
) -> impl Parser<'a, &'a [BalancedToken], PrimaryExpression, Extra<'a>> + Clone {
    let identifier = select! {
        BalancedToken::Identifier(value) => value
    };
    let constant = select! {
        BalancedToken::Constant(value) => value
    };
    let string_literal = select! {
        BalancedToken::StringLiteral(value) => value
    };

    choice((
        identifier.map(PrimaryExpression::Identifier),
        constant.map(PrimaryExpression::Constant),
        string_literal.map(PrimaryExpression::StringLiteral),
        expression
            .parenthesized()
            .map(Box::new)
            .map(PrimaryExpression::Parenthesized),
        // TODO: Generic selection
    ))
    .labelled("primiary expression")
    .as_context()
}

/// (6.5.2) postfix expression
pub fn postfix_expression<'a>(
    expression: impl Parser<'a, &'a [BalancedToken], Expression, Extra<'a>> + Clone + 'a,
    assignment_expression: impl Parser<'a, &'a [BalancedToken], Expression, Extra<'a>> + Clone + 'a,
) -> impl Parser<'a, &'a [BalancedToken], PostfixExpression, Extra<'a>> + Clone {
    let primary = primary_expression(expression.clone());

    let increment = punctuator(Punctuator::Increment);
    let decrement = punctuator(Punctuator::Decrement);
    let array = expression.bracketed();
    let function = assignment_expression
        .separated_by(punctuator(Punctuator::Comma))
        .collect::<Vec<Expression>>()
        .parenthesized();
    let identifier = select! {
        BalancedToken::Identifier(value) => value
    };
    let member_access = punctuator(Punctuator::Dot).ignore_then(identifier);
    let member_access_ptr = punctuator(Punctuator::Arrow).ignore_then(identifier);

    // TODO: compound literal

    type PostfixFn = Box<dyn FnOnce(PostfixExpression) -> PostfixExpression>;
    primary
        .map(PostfixExpression::Primary)
        .foldl(
            choice((
                increment.map(|_| -> PostfixFn {
                    Box::new(|expr| PostfixExpression::PostIncrement(Box::new(expr)))
                }),
                decrement.map(|_| -> PostfixFn {
                    Box::new(|expr| PostfixExpression::PostDecrement(Box::new(expr)))
                }),
                array.map(|idx| -> PostfixFn {
                    Box::new(move |expr| PostfixExpression::ArrayAccess {
                        array: Box::new(expr),
                        index: Box::new(idx),
                    })
                }),
                function.map(|args| -> PostfixFn {
                    Box::new(move |expr| PostfixExpression::FunctionCall {
                        function: Box::new(expr),
                        arguments: args,
                    })
                }),
                member_access.map(|member| -> PostfixFn {
                    Box::new(move |expr| PostfixExpression::MemberAccess {
                        object: Box::new(expr),
                        member,
                    })
                }),
                member_access_ptr.map(|member| -> PostfixFn {
                    Box::new(move |expr| PostfixExpression::MemberAccessPtr {
                        object: Box::new(expr),
                        member,
                    })
                }),
            ))
            .repeated(),
            |acc, f| f(acc),
        )
        .labelled("postfix expression")
        .as_context()
}

/// (6.5.3) unary expression
pub fn unary_expression<'a>(
    expression: impl Parser<'a, &'a [BalancedToken], Expression, Extra<'a>> + Clone + 'a,
    assignment_expression: impl Parser<'a, &'a [BalancedToken], Expression, Extra<'a>> + Clone + 'a,
) -> impl Parser<'a, &'a [BalancedToken], UnaryExpression, Extra<'a>> + Clone {
    recursive(|unary_expression| {
        let postfix = postfix_expression(expression, assignment_expression);

        let pre_increment = punctuator(Punctuator::Increment)
            .ignore_then(unary_expression.clone())
            .map(Box::new);

        let pre_decrement = punctuator(Punctuator::Decrement)
            .ignore_then(unary_expression.clone())
            .map(Box::new);

        let unary_operator = select! {
            BalancedToken::Punctuator(Punctuator::Ampersand) => UnaryOperator::Address,
            BalancedToken::Punctuator(Punctuator::Star) => UnaryOperator::Dereference,
            BalancedToken::Punctuator(Punctuator::Plus) => UnaryOperator::Plus,
            BalancedToken::Punctuator(Punctuator::Minus) => UnaryOperator::Minus,
            BalancedToken::Punctuator(Punctuator::Bang) => UnaryOperator::LogicalNot,
            BalancedToken::Punctuator(Punctuator::Tilde) => UnaryOperator::BitwiseNot,
        };
        let unary = unary_operator.then(cast_expression(unary_expression.clone()));

        let sizeof = keyword("sizeof")
            .ignore_then(unary_expression.clone())
            .map(Box::new);

        // TODO: sizeof type
        // TODO: _Alignof

        choice((
            pre_increment.map(UnaryExpression::PreIncrement),
            pre_decrement.map(UnaryExpression::PreDecrement),
            unary.map(|(operator, operand)| UnaryExpression::Unary {
                operator,
                operand: Box::new(operand),
            }),
            sizeof.map(UnaryExpression::Sizeof), // sizeof must be before postfix
            postfix.map(UnaryExpression::Postfix),
        ))
        .labelled("unary expression")
        .as_context()
    })
}

/// (6.5.4) cast expression
pub fn cast_expression<'a>(
    unary_expression: impl Parser<'a, &'a [BalancedToken], UnaryExpression, Extra<'a>> + Clone + 'a,
) -> impl Parser<'a, &'a [BalancedToken], CastExpression, Extra<'a>> + Clone {
    // TODO: cast
    recursive(|_cast_expression| unary_expression.map(CastExpression::Unary))
}

/// (6.5.5) multiplicative expression
///
/// (6.5.6) additive expression
///
/// (6.5.7) shift expression
///
/// (6.5.8) relational expression
///
/// (6.5.9) equality expression
///
/// (6.5.10) AND expression
///
/// (6.5.11) exclusive OR expression
///
/// (6.5.12) inclusive OR expression
///
/// (6.5.13) logical AND expression
///
/// (6.5.14) logical OR expression
pub fn binary_expression<'a>(
    expression: impl Parser<'a, &'a [BalancedToken], Expression, Extra<'a>> + Clone + 'a,
    assignment_expression: impl Parser<'a, &'a [BalancedToken], Expression, Extra<'a>> + Clone + 'a,
) -> impl Parser<'a, &'a [BalancedToken], Expression, Extra<'a>> + Clone {
    use chumsky::pratt::*;

    macro_rules! op {
        ($punct:expr) => {{
            use Punctuator::*;
            punctuator($punct)
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

    let unary = unary_expression(expression.clone(), assignment_expression.clone());
    let cast = cast_expression(unary.clone());
    let postfix = postfix_expression(expression.clone(), assignment_expression.clone());

    // Suppose precedence is X, use 1000 - 10*X as the associativity level
    choice((
        unary.map(Expression::Unary),
        cast.map(Expression::Cast),
        postfix.map(Expression::Postfix),
    ))
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
    .labelled("binary expression")
    .as_context()
}

/// (6.5.15) conditional expression
pub fn conditional_expression<'a>(
    binary_expression: impl Parser<'a, &'a [BalancedToken], Expression, Extra<'a>> + Clone + 'a,
    expression: impl Parser<'a, &'a [BalancedToken], Expression, Extra<'a>> + Clone + 'a,
) -> impl Parser<'a, &'a [BalancedToken], Expression, Extra<'a>> + Clone {
    recursive(|conditional_expression| {
        choice((
            binary_expression
                .clone()
                .then_ignore(punctuator(Punctuator::Question))
                .then(expression)
                .then_ignore(punctuator(Punctuator::Colon))
                .then(conditional_expression)
                .map(|((condition, then_expr), else_expr)| {
                    Expression::Conditional(ConditionalExpression {
                        condition: Box::new(condition),
                        then_expr: Box::new(then_expr),
                        else_expr: Box::new(else_expr),
                    })
                }),
            binary_expression,
        ))
        .labelled("conditional expression")
        .as_context()
    })
}

/// (6.5.16) assignment expression
pub fn assignment_expression<'a>(
    conditional_expression: impl Parser<'a, &'a [BalancedToken], Expression, Extra<'a>> + Clone + 'a,
    unary_expression: impl Parser<'a, &'a [BalancedToken], UnaryExpression, Extra<'a>> + Clone + 'a,
) -> impl Parser<'a, &'a [BalancedToken], Expression, Extra<'a>> + Clone {
    recursive(|assignment_expression| {
        let assigment_opeartor = select! {
            BalancedToken::Punctuator(Punctuator::Assign) => AssignmentOperator::Assign,
            BalancedToken::Punctuator(Punctuator::AddAssign) => AssignmentOperator::AddAssign,
            BalancedToken::Punctuator(Punctuator::SubAssign) => AssignmentOperator::SubAssign,
            BalancedToken::Punctuator(Punctuator::MulAssign) => AssignmentOperator::MulAssign,
            BalancedToken::Punctuator(Punctuator::DivAssign) => AssignmentOperator::DivAssign,
            BalancedToken::Punctuator(Punctuator::ModAssign) => AssignmentOperator::ModAssign,
            BalancedToken::Punctuator(Punctuator::AndAssign) => AssignmentOperator::AndAssign,
            BalancedToken::Punctuator(Punctuator::OrAssign) => AssignmentOperator::OrAssign,
            BalancedToken::Punctuator(Punctuator::XorAssign) => AssignmentOperator::XorAssign,
            BalancedToken::Punctuator(Punctuator::LeftShiftAssign) => AssignmentOperator::LeftShiftAssign,
            BalancedToken::Punctuator(Punctuator::RightShiftAssign) => AssignmentOperator::RightShiftAssign,
        };
        choice((
            unary_expression
                .then(assigment_opeartor)
                .then(assignment_expression)
                .map(|((left, operator), right)| {
                    Expression::Assignment(AssignmentExpression {
                        operator,
                        left: Box::new(Expression::Unary(left)),
                        right: Box::new(right),
                    })
                }),
            conditional_expression,
        ))
        .labelled("assignment expression")
        .as_context()
    })
}

/// (6.5.17) expression
pub fn expression<'a>() -> impl Parser<'a, &'a [BalancedToken], Expression, Extra<'a>> + Clone {
    recursive(|expression| {
        let mut assignment = Recursive::declare();

        let binary = binary_expression(expression.clone(), assignment.clone());
        let conditional = conditional_expression(binary, expression.clone());

        assignment.define(assignment_expression(
            conditional.clone(),
            unary_expression(expression.clone(), assignment.clone()),
        ));

        assignment
            .separated_by(punctuator(Punctuator::Comma))
            .at_least(1)
            .collect::<Vec<Expression>>()
            .map(|expressions| {
                if expressions.len() == 1 {
                    expressions.into_iter().next().unwrap()
                } else {
                    Expression::Comma(CommaExpression { expressions })
                }
            })
            .labelled("expression")
            .as_context()
    })
}

fn keyword<'a>(kwd: &str) -> impl Parser<'a, &'a [BalancedToken], (), Extra<'a>> + Clone {
    select! {
        BalancedToken::Identifier(Identifier(name)) if name == kwd => ()
    }
}

fn punctuator<'a>(punc: Punctuator) -> impl Parser<'a, &'a [BalancedToken], (), Extra<'a>> + Clone {
    select! {
        BalancedToken::Punctuator(p) if p == punc => ()
    }
}

trait ParserExt<O, E> {
    fn parenthesized<'a>(self) -> impl Parser<'a, &'a [BalancedToken], O, E> + Clone
    where
        Self: Sized,
        Self: Parser<'a, &'a [BalancedToken], O, E> + Clone,
        E: chumsky::extra::ParserExtra<'a, &'a [BalancedToken]>,
    {
        self.nested_in(select_ref! {
            BalancedToken::Parenthesized(BalancedTokenSequence { tokens }) => tokens.as_slice()
        })
    }

    fn bracketed<'a>(self) -> impl Parser<'a, &'a [BalancedToken], O, E> + Clone
    where
        Self: Sized,
        Self: Parser<'a, &'a [BalancedToken], O, E> + Clone,
        E: chumsky::extra::ParserExtra<'a, &'a [BalancedToken]>,
    {
        self.nested_in(select_ref! {
            BalancedToken::Bracketed(BalancedTokenSequence { tokens }) => tokens.as_slice()
        })
    }

    fn braced<'a>(self) -> impl Parser<'a, &'a [BalancedToken], O, E> + Clone
    where
        Self: Sized,
        Self: Parser<'a, &'a [BalancedToken], O, E> + Clone,
        E: chumsky::extra::ParserExtra<'a, &'a [BalancedToken]>,
    {
        self.nested_in(select_ref! {
            BalancedToken::Braced(BalancedTokenSequence { tokens }) => tokens.as_slice()
        })
    }
}

impl<'a, T, O, E> ParserExt<O, E> for T
where
    T: Parser<'a, &'a [BalancedToken], O, E>,
    E: chumsky::extra::ParserExtra<'a, &'a [BalancedToken]>,
{
}
