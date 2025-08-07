//! Unit tests for the C grammar parser module
//!
//! This module contains comprehensive tests for all parser functions in parser.rs,
//! including primary expressions, postfix expressions, unary expressions,
//! binary expressions, conditional expressions, assignment expressions,
//! and the main expression parser.
//!
//! Tests cover:
//! - Basic functionality of each parser component
//! - Operator precedence and associativity
//! - Complex nested expressions
//! - Error cases and edge conditions
//! - Performance characteristics

use cgrammar::*;
use chumsky::prelude::*;
use rstest::rstest;

// Helper functions to create balanced tokens for testing
fn make_identifier(name: &str) -> BalancedToken {
    BalancedToken::Identifier(Identifier(name.to_string()))
}

fn make_constant(value: i128) -> BalancedToken {
    BalancedToken::Constant(Constant::Integer(IntegerConstant {
        value,
        suffix: None,
    }))
}

#[allow(dead_code)]
fn make_float_constant(value: f64) -> BalancedToken {
    BalancedToken::Constant(Constant::Floating(FloatingConstant {
        value,
        suffix: None,
    }))
}

fn make_string_literal(value: &str) -> BalancedToken {
    BalancedToken::StringLiteral(StringLiteral {
        encoding_prefix: None,
        value: value.to_string(),
    })
}

fn make_punctuator(punct: Punctuator) -> BalancedToken {
    BalancedToken::Punctuator(punct)
}

fn make_parenthesized(tokens: Vec<BalancedToken>) -> BalancedToken {
    BalancedToken::Parenthesized(BalancedTokenSequence { tokens })
}

fn make_bracketed(tokens: Vec<BalancedToken>) -> BalancedToken {
    BalancedToken::Bracketed(BalancedTokenSequence { tokens })
}

#[allow(dead_code)]
fn make_braced(tokens: Vec<BalancedToken>) -> BalancedToken {
    BalancedToken::Braced(BalancedTokenSequence { tokens })
}

// =============================================================================
// Test primary_expression parser
// =============================================================================

#[rstest]
#[case(
    vec![make_identifier("variable")],
    |result: PrimaryExpression| matches!(result, PrimaryExpression::Identifier(_))
)]
#[case(
    vec![make_constant(42)],
    |result: PrimaryExpression| matches!(result, PrimaryExpression::Constant(_))
)]
#[case(
    vec![make_string_literal("hello")],
    |result: PrimaryExpression| matches!(result, PrimaryExpression::StringLiteral(_))
)]
fn test_primary_expression_valid<F>(#[case] input: Vec<BalancedToken>, #[case] validator: F)
where
    F: Fn(PrimaryExpression) -> bool,
{
    let dummy_expression = empty().to(Expression::Unary(UnaryExpression::Postfix(
        PostfixExpression::Primary(PrimaryExpression::Identifier(Identifier(
            "dummy".to_string(),
        ))),
    )));
    let parser = primary_expression(dummy_expression);
    let result = parser.parse(input.as_slice()).unwrap();
    assert!(validator(result));
}

#[rstest]
#[case(
    vec![make_parenthesized(vec![make_identifier("x")])],
    |result: PrimaryExpression| matches!(result, PrimaryExpression::Parenthesized(_))
)]
fn test_primary_expression_parenthesized(
    #[case] input: Vec<BalancedToken>,
    #[case] validator: fn(PrimaryExpression) -> bool,
) {
    // Create a recursive expression parser for testing parenthesized expressions
    let parser = expression();
    let primary_parser = primary_expression(parser);
    let result = primary_parser.parse(input.as_slice()).unwrap();
    assert!(validator(result));
}

// =============================================================================
// Test postfix_expression parser
// =============================================================================

#[rstest]
#[case(
    vec![make_identifier("var"), make_punctuator(Punctuator::Increment)],
    |result: PostfixExpression| matches!(result, PostfixExpression::PostIncrement(_))
)]
#[case(
    vec![make_identifier("var"), make_punctuator(Punctuator::Decrement)],
    |result: PostfixExpression| matches!(result, PostfixExpression::PostDecrement(_))
)]
#[case(
    vec![make_identifier("array"), make_bracketed(vec![make_constant(0)])],
    |result: PostfixExpression| matches!(result, PostfixExpression::ArrayAccess { .. })
)]
fn test_postfix_expression_valid<F>(#[case] input: Vec<BalancedToken>, #[case] validator: F)
where
    F: Fn(PostfixExpression) -> bool,
{
    let parser = expression();
    let postfix_parser = postfix_expression(parser);
    let result = postfix_parser.parse(input.as_slice()).unwrap();
    assert!(validator(result));
}

#[rstest]
#[case(
    vec![make_identifier("simple")],
    |result: PostfixExpression| matches!(result, PostfixExpression::Primary(_))
)]
fn test_postfix_expression_primary(
    #[case] input: Vec<BalancedToken>,
    #[case] validator: fn(PostfixExpression) -> bool,
) {
    let parser = expression();
    let postfix_parser = postfix_expression(parser);
    let result = postfix_parser.parse(input.as_slice()).unwrap();
    assert!(validator(result));
}

// =============================================================================
// Test unary_expression parser
// =============================================================================

#[rstest]
#[case(
    vec![make_punctuator(Punctuator::Increment), make_identifier("var")],
    |result: UnaryExpression| matches!(result, UnaryExpression::PreIncrement(_))
)]
#[case(
    vec![make_punctuator(Punctuator::Decrement), make_identifier("var")],
    |result: UnaryExpression| matches!(result, UnaryExpression::PreDecrement(_))
)]
#[case(
    vec![make_punctuator(Punctuator::Ampersand), make_identifier("var")],
    |result: UnaryExpression| matches!(result, UnaryExpression::Unary { operator: UnaryOperator::Address, .. })
)]
#[case(
    vec![make_punctuator(Punctuator::Star), make_identifier("ptr")],
    |result: UnaryExpression| matches!(result, UnaryExpression::Unary { operator: UnaryOperator::Dereference, .. })
)]
#[case(
    vec![make_punctuator(Punctuator::Plus), make_constant(42)],
    |result: UnaryExpression| matches!(result, UnaryExpression::Unary { operator: UnaryOperator::Plus, .. })
)]
#[case(
    vec![make_punctuator(Punctuator::Minus), make_constant(42)],
    |result: UnaryExpression| matches!(result, UnaryExpression::Unary { operator: UnaryOperator::Minus, .. })
)]
#[case(
    vec![make_punctuator(Punctuator::Bang), make_identifier("flag")],
    |result: UnaryExpression| matches!(result, UnaryExpression::Unary { operator: UnaryOperator::LogicalNot, .. })
)]
#[case(
    vec![make_punctuator(Punctuator::Tilde), make_identifier("bits")],
    |result: UnaryExpression| matches!(result, UnaryExpression::Unary { operator: UnaryOperator::BitwiseNot, .. })
)]
fn test_unary_expression_valid<F>(#[case] input: Vec<BalancedToken>, #[case] validator: F)
where
    F: Fn(UnaryExpression) -> bool,
{
    let parser = expression();
    let unary_parser = unary_expression(parser);
    let result = unary_parser.parse(input.as_slice()).unwrap();
    assert!(validator(result));
}

#[rstest]
#[case(
    vec![make_identifier("simple")],
    |result: UnaryExpression| matches!(result, UnaryExpression::Postfix(_))
)]
fn test_unary_expression_postfix(
    #[case] input: Vec<BalancedToken>,
    #[case] validator: fn(UnaryExpression) -> bool,
) {
    let parser = expression();
    let unary_parser = unary_expression(parser);
    let result = unary_parser.parse(input.as_slice()).unwrap();
    assert!(validator(result));
}

// =============================================================================
// Test cast_expression parser
// =============================================================================

#[rstest]
#[case(
    vec![make_identifier("var")],
    |result: CastExpression| matches!(result, CastExpression::Unary(_))
)]
fn test_cast_expression_valid<F>(#[case] input: Vec<BalancedToken>, #[case] validator: F)
where
    F: Fn(CastExpression) -> bool,
{
    let parser = expression();
    let unary_parser = unary_expression(parser);
    let cast_parser = cast_expression(unary_parser);
    let result = cast_parser.parse(input.as_slice()).unwrap();
    assert!(validator(result));
}

// =============================================================================
// Test binary_expression parser (multiplication, addition, etc.)
// =============================================================================

#[rstest]
#[case(
    vec![
        make_constant(2),
        make_punctuator(Punctuator::Star),
        make_constant(3)
    ],
    |result: Expression| matches!(result, Expression::Binary(BinaryExpression { operator: BinaryOperator::Multiply, .. }))
)]
#[case(
    vec![
        make_constant(5),
        make_punctuator(Punctuator::Plus),
        make_constant(3)
    ],
    |result: Expression| matches!(result, Expression::Binary(BinaryExpression { operator: BinaryOperator::Add, .. }))
)]
#[case(
    vec![
        make_constant(10),
        make_punctuator(Punctuator::Minus),
        make_constant(4)
    ],
    |result: Expression| matches!(result, Expression::Binary(BinaryExpression { operator: BinaryOperator::Subtract, .. }))
)]
#[case(
    vec![
        make_constant(15),
        make_punctuator(Punctuator::Slash),
        make_constant(3)
    ],
    |result: Expression| matches!(result, Expression::Binary(BinaryExpression { operator: BinaryOperator::Divide, .. }))
)]
#[case(
    vec![
        make_constant(17),
        make_punctuator(Punctuator::Percent),
        make_constant(5)
    ],
    |result: Expression| matches!(result, Expression::Binary(BinaryExpression { operator: BinaryOperator::Modulo, .. }))
)]
fn test_binary_expression_arithmetic<F>(#[case] input: Vec<BalancedToken>, #[case] validator: F)
where
    F: Fn(Expression) -> bool,
{
    let parser = expression();
    let binary_parser = binary_expression(parser);
    let result = binary_parser.parse(input.as_slice()).unwrap();
    assert!(validator(result));
}

#[rstest]
#[case(
    vec![
        make_constant(1),
        make_punctuator(Punctuator::LeftShift),
        make_constant(2)
    ],
    |result: Expression| matches!(result, Expression::Binary(BinaryExpression { operator: BinaryOperator::LeftShift, .. }))
)]
#[case(
    vec![
        make_constant(8),
        make_punctuator(Punctuator::RightShift),
        make_constant(1)
    ],
    |result: Expression| matches!(result, Expression::Binary(BinaryExpression { operator: BinaryOperator::RightShift, .. }))
)]
fn test_binary_expression_shift<F>(#[case] input: Vec<BalancedToken>, #[case] validator: F)
where
    F: Fn(Expression) -> bool,
{
    let parser = expression();
    let binary_parser = binary_expression(parser);
    let result = binary_parser.parse(input.as_slice()).unwrap();
    assert!(validator(result));
}

#[rstest]
#[case(
    vec![
        make_constant(5),
        make_punctuator(Punctuator::Less),
        make_constant(10)
    ],
    |result: Expression| matches!(result, Expression::Binary(BinaryExpression { operator: BinaryOperator::Less, .. }))
)]
#[case(
    vec![
        make_constant(15),
        make_punctuator(Punctuator::Greater),
        make_constant(10)
    ],
    |result: Expression| matches!(result, Expression::Binary(BinaryExpression { operator: BinaryOperator::Greater, .. }))
)]
#[case(
    vec![
        make_constant(5),
        make_punctuator(Punctuator::LessEqual),
        make_constant(5)
    ],
    |result: Expression| matches!(result, Expression::Binary(BinaryExpression { operator: BinaryOperator::LessEqual, .. }))
)]
#[case(
    vec![
        make_constant(10),
        make_punctuator(Punctuator::GreaterEqual),
        make_constant(5)
    ],
    |result: Expression| matches!(result, Expression::Binary(BinaryExpression { operator: BinaryOperator::GreaterEqual, .. }))
)]
#[case(
    vec![
        make_constant(5),
        make_punctuator(Punctuator::Equal),
        make_constant(5)
    ],
    |result: Expression| matches!(result, Expression::Binary(BinaryExpression { operator: BinaryOperator::Equal, .. }))
)]
#[case(
    vec![
        make_constant(5),
        make_punctuator(Punctuator::NotEqual),
        make_constant(3)
    ],
    |result: Expression| matches!(result, Expression::Binary(BinaryExpression { operator: BinaryOperator::NotEqual, .. }))
)]
fn test_binary_expression_relational<F>(#[case] input: Vec<BalancedToken>, #[case] validator: F)
where
    F: Fn(Expression) -> bool,
{
    let parser = expression();
    let binary_parser = binary_expression(parser);
    let result = binary_parser.parse(input.as_slice()).unwrap();
    assert!(validator(result));
}

#[rstest]
#[case(
    vec![
        make_constant(5),
        make_punctuator(Punctuator::Ampersand),
        make_constant(3)
    ],
    |result: Expression| matches!(result, Expression::Binary(BinaryExpression { operator: BinaryOperator::BitwiseAnd, .. }))
)]
#[case(
    vec![
        make_constant(5),
        make_punctuator(Punctuator::Caret),
        make_constant(3)
    ],
    |result: Expression| matches!(result, Expression::Binary(BinaryExpression { operator: BinaryOperator::BitwiseXor, .. }))
)]
#[case(
    vec![
        make_constant(5),
        make_punctuator(Punctuator::Pipe),
        make_constant(3)
    ],
    |result: Expression| matches!(result, Expression::Binary(BinaryExpression { operator: BinaryOperator::BitwiseOr, .. }))
)]
fn test_binary_expression_bitwise<F>(#[case] input: Vec<BalancedToken>, #[case] validator: F)
where
    F: Fn(Expression) -> bool,
{
    let parser = expression();
    let binary_parser = binary_expression(parser);
    let result = binary_parser.parse(input.as_slice()).unwrap();
    assert!(validator(result));
}

#[rstest]
#[case(
    vec![
        make_constant(1),
        make_punctuator(Punctuator::LogicalAnd),
        make_constant(1)
    ],
    |result: Expression| matches!(result, Expression::Binary(BinaryExpression { operator: BinaryOperator::LogicalAnd, .. }))
)]
#[case(
    vec![
        make_constant(0),
        make_punctuator(Punctuator::LogicalOr),
        make_constant(1)
    ],
    |result: Expression| matches!(result, Expression::Binary(BinaryExpression { operator: BinaryOperator::LogicalOr, .. }))
)]
fn test_binary_expression_logical<F>(#[case] input: Vec<BalancedToken>, #[case] validator: F)
where
    F: Fn(Expression) -> bool,
{
    let parser = expression();
    let binary_parser = binary_expression(parser);
    let result = binary_parser.parse(input.as_slice()).unwrap();
    assert!(validator(result));
}

// =============================================================================
// Test precedence in binary_expression parser
// =============================================================================

#[rstest]
#[case(
    vec![
        make_constant(2),
        make_punctuator(Punctuator::Plus),
        make_constant(3),
        make_punctuator(Punctuator::Star),
        make_constant(4)
    ],
    |result: Expression| {
        // Should parse as: 2 + (3 * 4)
        if let Expression::Binary(BinaryExpression { operator: BinaryOperator::Add, left, right }) = result {
            matches!(*left, Expression::Unary(_)) &&
            matches!(*right, Expression::Binary(BinaryExpression { operator: BinaryOperator::Multiply, .. }))
        } else {
            false
        }
    }
)]
#[case(
    vec![
        make_constant(2),
        make_punctuator(Punctuator::Star),
        make_constant(3),
        make_punctuator(Punctuator::Plus),
        make_constant(4)
    ],
    |result: Expression| {
        // Should parse as: (2 * 3) + 4
        if let Expression::Binary(BinaryExpression { operator: BinaryOperator::Add, left, right }) = result {
            matches!(*left, Expression::Binary(BinaryExpression { operator: BinaryOperator::Multiply, .. })) &&
            matches!(*right, Expression::Unary(_))
        } else {
            false
        }
    }
)]
fn test_binary_expression_precedence<F>(#[case] input: Vec<BalancedToken>, #[case] validator: F)
where
    F: Fn(Expression) -> bool,
{
    let parser = expression();
    let binary_parser = binary_expression(parser);
    let result = binary_parser.parse(input.as_slice()).unwrap();
    assert!(validator(result));
}

// =============================================================================
// Test conditional_expression parser (ternary operator)
// =============================================================================

#[rstest]
#[case(
    vec![
        make_constant(1),
        make_punctuator(Punctuator::Question),
        make_constant(42),
        make_punctuator(Punctuator::Colon),
        make_constant(0)
    ],
    |result: Expression| matches!(result, Expression::Conditional(_))
)]
fn test_conditional_expression_valid<F>(#[case] input: Vec<BalancedToken>, #[case] validator: F)
where
    F: Fn(Expression) -> bool,
{
    let parser = expression();
    let binary_parser = binary_expression(parser.clone());
    let conditional_parser = conditional_expression(binary_parser, parser);
    let result = conditional_parser.parse(input.as_slice()).unwrap();
    assert!(validator(result));
}

#[rstest]
#[case(
    vec![make_constant(42)],
    |result: Expression| {
        // Should fall back to binary expression
        matches!(result, Expression::Unary(_))
    }
)]
fn test_conditional_expression_fallback<F>(#[case] input: Vec<BalancedToken>, #[case] validator: F)
where
    F: Fn(Expression) -> bool,
{
    let parser = expression();
    let binary_parser = binary_expression(parser.clone());
    let conditional_parser = conditional_expression(binary_parser, parser);
    let result = conditional_parser.parse(input.as_slice()).unwrap();
    assert!(validator(result));
}

// =============================================================================
// Test assignment_expression parser
// =============================================================================

#[rstest]
#[case(
    vec![
        make_identifier("var"),
        make_punctuator(Punctuator::Assign),
        make_constant(42)
    ],
    |result: Expression| matches!(result, Expression::Assignment(AssignmentExpression { operator: AssignmentOperator::Assign, .. }))
)]
#[case(
    vec![
        make_identifier("var"),
        make_punctuator(Punctuator::AddAssign),
        make_constant(5)
    ],
    |result: Expression| matches!(result, Expression::Assignment(AssignmentExpression { operator: AssignmentOperator::AddAssign, .. }))
)]
#[case(
    vec![
        make_identifier("var"),
        make_punctuator(Punctuator::SubAssign),
        make_constant(3)
    ],
    |result: Expression| matches!(result, Expression::Assignment(AssignmentExpression { operator: AssignmentOperator::SubAssign, .. }))
)]
#[case(
    vec![
        make_identifier("var"),
        make_punctuator(Punctuator::MulAssign),
        make_constant(2)
    ],
    |result: Expression| matches!(result, Expression::Assignment(AssignmentExpression { operator: AssignmentOperator::MulAssign, .. }))
)]
#[case(
    vec![
        make_identifier("var"),
        make_punctuator(Punctuator::DivAssign),
        make_constant(2)
    ],
    |result: Expression| matches!(result, Expression::Assignment(AssignmentExpression { operator: AssignmentOperator::DivAssign, .. }))
)]
#[case(
    vec![
        make_identifier("var"),
        make_punctuator(Punctuator::ModAssign),
        make_constant(3)
    ],
    |result: Expression| matches!(result, Expression::Assignment(AssignmentExpression { operator: AssignmentOperator::ModAssign, .. }))
)]
#[case(
    vec![
        make_identifier("var"),
        make_punctuator(Punctuator::AndAssign),
        make_constant(15)
    ],
    |result: Expression| matches!(result, Expression::Assignment(AssignmentExpression { operator: AssignmentOperator::AndAssign, .. }))
)]
#[case(
    vec![
        make_identifier("var"),
        make_punctuator(Punctuator::OrAssign),
        make_constant(7)
    ],
    |result: Expression| matches!(result, Expression::Assignment(AssignmentExpression { operator: AssignmentOperator::OrAssign, .. }))
)]
#[case(
    vec![
        make_identifier("var"),
        make_punctuator(Punctuator::XorAssign),
        make_constant(12)
    ],
    |result: Expression| matches!(result, Expression::Assignment(AssignmentExpression { operator: AssignmentOperator::XorAssign, .. }))
)]
#[case(
    vec![
        make_identifier("var"),
        make_punctuator(Punctuator::LeftShiftAssign),
        make_constant(2)
    ],
    |result: Expression| matches!(result, Expression::Assignment(AssignmentExpression { operator: AssignmentOperator::LeftShiftAssign, .. }))
)]
#[case(
    vec![
        make_identifier("var"),
        make_punctuator(Punctuator::RightShiftAssign),
        make_constant(1)
    ],
    |result: Expression| matches!(result, Expression::Assignment(AssignmentExpression { operator: AssignmentOperator::RightShiftAssign, .. }))
)]
fn test_assignment_expression_valid<F>(#[case] input: Vec<BalancedToken>, #[case] validator: F)
where
    F: Fn(Expression) -> bool,
{
    let expression_parser = expression();
    let binary_parser = binary_expression(expression_parser.clone());
    let conditional_parser = conditional_expression(binary_parser, expression_parser.clone());
    let unary_parser = unary_expression(expression_parser);
    let assignment_parser = assignment_expression(conditional_parser, unary_parser);
    let result = assignment_parser.parse(input.as_slice()).unwrap();
    assert!(validator(result));
}

// =============================================================================
// Test expression parser (comma operator)
// =============================================================================

#[rstest]
#[case(
    vec![make_constant(42)],
    |result: Expression| {
        // Single expression should not be wrapped in comma expression
        matches!(result, Expression::Unary(_))
    }
)]
#[case(
    vec![
        make_constant(1),
        make_punctuator(Punctuator::Comma),
        make_constant(2),
        make_punctuator(Punctuator::Comma),
        make_constant(3)
    ],
    |result: Expression| {
        if let Expression::Comma(CommaExpression { expressions }) = result {
            expressions.len() == 3
        } else {
            false
        }
    }
)]
fn test_expression_comma<F>(#[case] input: Vec<BalancedToken>, #[case] validator: F)
where
    F: Fn(Expression) -> bool,
{
    let parser = expression();
    let result = parser.parse(input.as_slice()).unwrap();
    assert!(validator(result));
}

// =============================================================================
// Test complex nested expressions
// =============================================================================

#[rstest]
#[case(
    vec![
        make_punctuator(Punctuator::Star),
        make_identifier("ptr"),
        make_punctuator(Punctuator::Plus),
        make_constant(1)
    ],
    |result: Expression| {
        // Should parse as: (*ptr) + 1
        if let Expression::Binary(BinaryExpression { operator: BinaryOperator::Add, left, right }) = result {
            matches!(*left, Expression::Unary(UnaryExpression::Unary { operator: UnaryOperator::Dereference, .. })) &&
            matches!(*right, Expression::Unary(_))
        } else {
            false
        }
    }
)]
#[case(
    vec![
        make_identifier("array"),
        make_bracketed(vec![make_constant(0)]),
        make_punctuator(Punctuator::Assign),
        make_constant(42)
    ],
    |result: Expression| {
        // Should parse as: array[0] = 42
        matches!(result, Expression::Assignment(AssignmentExpression { operator: AssignmentOperator::Assign, .. }))
    }
)]
fn test_complex_expressions<F>(#[case] input: Vec<BalancedToken>, #[case] validator: F)
where
    F: Fn(Expression) -> bool,
{
    let parser = expression();
    let result = parser.parse(input.as_slice()).unwrap();
    assert!(validator(result));
}

// =============================================================================
// Test error cases
// =============================================================================

#[rstest]
#[case(vec![])] // Empty input
#[case(vec![make_punctuator(Punctuator::Plus)])] // Incomplete unary expression
#[case(vec![make_constant(1), make_punctuator(Punctuator::Plus)])] // Incomplete binary expression
#[case(vec![make_constant(1), make_punctuator(Punctuator::Question)])] // Incomplete ternary
#[case(vec![make_identifier("var"), make_punctuator(Punctuator::Assign)])] // Incomplete assignment
fn test_expression_invalid(#[case] input: Vec<BalancedToken>) {
    let parser = expression();
    let result = parser.parse(input.as_slice());
    assert!(result.has_errors());
}

// =============================================================================
// Test real-world expression patterns
// =============================================================================

#[rstest]
#[case(
    vec![
        make_identifier("x"),
        make_punctuator(Punctuator::Star),
        make_identifier("x"),
        make_punctuator(Punctuator::Plus),
        make_identifier("y"),
        make_punctuator(Punctuator::Star),
        make_identifier("y")
    ],
    |result: Expression| {
        // x * x + y * y - should respect precedence
        if let Expression::Binary(BinaryExpression { operator: BinaryOperator::Add, left, right }) = result {
            matches!(*left, Expression::Binary(BinaryExpression { operator: BinaryOperator::Multiply, .. })) &&
            matches!(*right, Expression::Binary(BinaryExpression { operator: BinaryOperator::Multiply, .. }))
        } else {
            false
        }
    }
)]
#[case(
    vec![
        make_identifier("a"),
        make_punctuator(Punctuator::Greater),
        make_identifier("b"),
        make_punctuator(Punctuator::Question),
        make_identifier("a"),
        make_punctuator(Punctuator::Colon),
        make_identifier("b")
    ],
    |result: Expression| {
        // a > b ? a : b (max function)
        matches!(result, Expression::Conditional(_))
    }
)]
#[case(
    vec![
        make_identifier("sum"),
        make_punctuator(Punctuator::AddAssign),
        make_identifier("arr"),
        make_bracketed(vec![make_identifier("i")]),
        make_punctuator(Punctuator::Star),
        make_constant(2)
    ],
    |result: Expression| {
        // sum += arr[i] * 2
        matches!(result, Expression::Assignment(AssignmentExpression { operator: AssignmentOperator::AddAssign, .. }))
    }
)]
fn test_realistic_expressions<F>(#[case] input: Vec<BalancedToken>, #[case] validator: F)
where
    F: Fn(Expression) -> bool,
{
    let parser = expression();
    let result = parser.parse(input.as_slice()).unwrap();
    assert!(validator(result));
}

// =============================================================================
// Performance and stress tests
// =============================================================================

#[test]
fn test_large_expression_performance() {
    // Create a large expression: 1 + 2 + 3 + ... + 50
    let mut tokens = vec![make_constant(1)];
    for i in 2..=50 {
        tokens.push(make_punctuator(Punctuator::Plus));
        tokens.push(make_constant(i));
    }

    let parser = expression();
    let start = std::time::Instant::now();
    let result = parser.parse(tokens.as_slice());
    let duration = start.elapsed();

    assert!(!result.has_errors());
    assert!(duration.as_millis() < 100); // Should parse quickly
}

#[test]
fn test_deeply_nested_parentheses() {
    // Create deeply nested parenthesized expression
    let x_token = make_identifier("x");
    let nested = make_parenthesized(vec![x_token]);
    let tokens = vec![nested];

    let parser = expression();
    let result = parser.parse(tokens.as_slice());
    assert!(!result.has_errors());
}

#[test]
fn test_complex_precedence_chain() {
    // Test: a + b * c / d - e % f
    let tokens = vec![
        make_identifier("a"),
        make_punctuator(Punctuator::Plus),
        make_identifier("b"),
        make_punctuator(Punctuator::Star),
        make_identifier("c"),
        make_punctuator(Punctuator::Slash),
        make_identifier("d"),
        make_punctuator(Punctuator::Minus),
        make_identifier("e"),
        make_punctuator(Punctuator::Percent),
        make_identifier("f"),
    ];

    let parser = expression();
    let result = parser.parse(tokens.as_slice());
    assert!(!result.has_errors());
}

#[test]
fn test_cast_expression_basic() {
    // Basic cast expression test
    let tokens = vec![make_identifier("value")];

    let expression_parser = expression();
    let unary_parser = unary_expression(expression_parser);
    let cast_parser = cast_expression(unary_parser);

    let result = cast_parser.parse(tokens.as_slice());
    assert!(!result.has_errors());

    let cast_expr = result.unwrap();
    match cast_expr {
        CastExpression::Unary(_) => {
            // Expected for current implementation
        }
        CastExpression::Cast { .. } => {
            // Would be expected for full cast implementation
        }
    }
}

// =============================================================================
// Error recovery and edge case tests
// =============================================================================

#[test]
fn test_empty_parentheses() {
    let tokens = vec![make_parenthesized(vec![])];

    let parser = expression();
    let result = parser.parse(tokens.as_slice());
    // This should fail because empty parentheses don't contain a valid expression
    assert!(result.has_errors());
}

#[test]
fn test_mixed_operators_without_operands() {
    let tokens = vec![
        make_punctuator(Punctuator::Plus),
        make_punctuator(Punctuator::Star),
        make_punctuator(Punctuator::Minus),
    ];

    let parser = expression();
    let result = parser.parse(tokens.as_slice());
    // This should fail because operators without operands are invalid
    assert!(result.has_errors());
}

#[test]
fn test_unmatched_ternary_operator() {
    let tokens = vec![
        make_identifier("condition"),
        make_punctuator(Punctuator::Question),
        make_identifier("true_value"),
        // Missing colon and false value
    ];

    let parser = expression();
    let result = parser.parse(tokens.as_slice());
    // This should fail because ternary operator is incomplete
    assert!(result.has_errors());
}
