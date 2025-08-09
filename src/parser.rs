use crate::ast::*;
use crate::context::State;
use crate::utils::*;

use chumsky::prelude::*;
use macro_rules_attribute::apply;

pub type Token = BalancedToken;
pub type TokenStream = BalancedTokenSequence;
type Extra<'a> = chumsky::extra::Full<Rich<'a, Token>, State, ()>;

// =============================================================================
// Expressions
// =============================================================================

/// (6.5.1) primary expression
pub fn primary_expression<'a>() -> impl Parser<'a, &'a [Token], PrimaryExpression, Extra<'a>> + Clone {
    choice((
        generic_selection().map(PrimaryExpression::Generic),
        constant().map(PrimaryExpression::Constant),
        enumeration_constant().map(PrimaryExpression::EnumerationConstant),
        identifier().map(PrimaryExpression::Identifier),
        string_literal().map(PrimaryExpression::StringLiteral),
        expression()
            .parenthesized()
            .map(Box::new)
            .map(PrimaryExpression::Parenthesized),
    ))
    .labelled("primiary expression")
    .as_context()
}

pub fn enumeration_constant<'a>() -> impl Parser<'a, &'a [Token], Identifier, Extra<'a>> + Clone {
    identifier()
        .try_map_with(|name, extra| {
            if extra.state().ctx().is_enum_constant(&name) {
                Ok(name)
            } else {
                Err(expected_found(
                    ["enumeration constant"],
                    Some(Token::Identifier(name)),
                    extra.span(),
                ))
            }
        })
        .labelled("enumeration constant")
        .as_context()
}

/// (6.5.1.1) generic selection
pub fn generic_selection<'a>() -> impl Parser<'a, &'a [Token], GenericSelection, Extra<'a>> + Clone {
    keyword("_Generic")
        .ignore_then(
            assignment_expression()
                .map(Brand::into_inner)
                .map(Box::new)
                .then_ignore(punctuator(Punctuator::Comma))
                .then(generic_association_list())
                .parenthesized(),
        )
        .map(|(controlling_expression, associations)| GenericSelection { controlling_expression, associations })
        .labelled("generic selection")
        .as_context()
}

/// (6.5.1.1) generic association list
pub fn generic_association_list<'a>() -> impl Parser<'a, &'a [Token], Vec<GenericAssociation>, Extra<'a>> + Clone {
    generic_association()
        .separated_by(punctuator(Punctuator::Comma))
        .at_least(1)
        .collect::<Vec<GenericAssociation>>()
        .labelled("generic association list")
        .as_context()
}

/// (6.5.1.1) generic association
pub fn generic_association<'a>() -> impl Parser<'a, &'a [Token], GenericAssociation, Extra<'a>> + Clone {
    choice((
        keyword("default")
            .ignore_then(punctuator(Punctuator::Colon))
            .ignore_then(assignment_expression().map(Brand::into_inner).map(Box::new))
            .map(|expression| GenericAssociation::Default { expression }),
        type_name()
            .then_ignore(punctuator(Punctuator::Colon))
            .then(assignment_expression().map(Brand::into_inner).map(Box::new))
            .map(|(type_name, expression)| GenericAssociation::Type { type_name, expression }),
    ))
    .labelled("generic association")
    .as_context()
}

/// (6.5.2) postfix expression
#[apply(cached)]
pub fn postfix_expression<'a>() -> impl Parser<'a, &'a [Token], PostfixExpression, Extra<'a>> + Clone {
    let increment = punctuator(Punctuator::Increment);
    let decrement = punctuator(Punctuator::Decrement);
    let array = expression().bracketed();
    let function = assignment_expression()
        .map(Brand::into_inner)
        .separated_by(punctuator(Punctuator::Comma))
        .collect::<Vec<Expression>>()
        .parenthesized();
    let member_access = punctuator(Punctuator::Dot).ignore_then(identifier());
    let member_access_ptr = punctuator(Punctuator::Arrow).ignore_then(identifier());

    type PostfixFn = Box<dyn FnOnce(PostfixExpression) -> PostfixExpression>;
    let postfix = primary_expression().map(PostfixExpression::Primary).foldl(
        choice((
            increment.map(|_| -> PostfixFn { Box::new(|expr| PostfixExpression::PostIncrement(Box::new(expr))) }),
            decrement.map(|_| -> PostfixFn { Box::new(|expr| PostfixExpression::PostDecrement(Box::new(expr))) }),
            array.map(|idx| -> PostfixFn {
                Box::new(move |expr| PostfixExpression::ArrayAccess {
                    array: Box::new(expr),
                    index: Box::new(idx),
                })
            }),
            function.map(|arguments| -> PostfixFn {
                Box::new(move |expr| PostfixExpression::FunctionCall { function: Box::new(expr), arguments })
            }),
            member_access.map(|member| -> PostfixFn {
                Box::new(move |expr| PostfixExpression::MemberAccess { object: Box::new(expr), member })
            }),
            member_access_ptr.map(|member| -> PostfixFn {
                Box::new(move |expr| PostfixExpression::MemberAccessPtr { object: Box::new(expr), member })
            }),
        ))
        .repeated(),
        |acc, f| f(acc),
    );

    choice((compound_literal().map(PostfixExpression::CompoundLiteral), postfix))
        .labelled("postfix expression")
        .as_context()
}

/// (6.5.2.5) compound literal
pub fn compound_literal<'a>() -> impl Parser<'a, &'a [Token], CompoundLiteral, Extra<'a>> + Clone {
    storage_class_specifiers()
        .then(type_name())
        .parenthesized()
        .then(braced_initializer())
        .map(|((storage_class_specifiers, type_name), initializer)| CompoundLiteral {
            storage_class_specifiers,
            type_name,
            initializer,
        })
        .labelled("compound literal")
        .as_context()
}

/// (6.5.2.5) storage class specifiers
pub fn storage_class_specifiers<'a>() -> impl Parser<'a, &'a [Token], Vec<StorageClassSpecifier>, Extra<'a>> + Clone {
    storage_class_specifier()
        .repeated()
        .collect::<Vec<StorageClassSpecifier>>()
        .labelled("storage class specifiers")
        .as_context()
}

/// (6.5.3) unary expression
#[apply(cached)]
pub fn unary_expression<'a>() -> impl Parser<'a, &'a [Token], UnaryExpression, Extra<'a>> + Clone {
    let pre_increment = punctuator(Punctuator::Increment)
        .ignore_then(unary_expression())
        .map(Box::new);

    let pre_decrement = punctuator(Punctuator::Decrement)
        .ignore_then(unary_expression())
        .map(Box::new);

    let unary_operator = select! {
        Token::Punctuator(Punctuator::Ampersand) => UnaryOperator::Address,
        Token::Punctuator(Punctuator::Star) => UnaryOperator::Dereference,
        Token::Punctuator(Punctuator::Plus) => UnaryOperator::Plus,
        Token::Punctuator(Punctuator::Minus) => UnaryOperator::Minus,
        Token::Punctuator(Punctuator::Bang) => UnaryOperator::LogicalNot,
        Token::Punctuator(Punctuator::Tilde) => UnaryOperator::BitwiseNot,
    };
    let unary = unary_operator.then(cast_expression());

    let sizeof_expr = keyword("sizeof").ignore_then(unary_expression()).map(Box::new);
    let sizeof_type = keyword("sizeof").ignore_then(type_name().parenthesized());
    let alignof_type = keyword("alignof").ignore_then(type_name().parenthesized());

    let postfix = postfix_expression();

    choice((
        pre_increment.map(UnaryExpression::PreIncrement),
        pre_decrement.map(UnaryExpression::PreDecrement),
        unary.map(|(operator, operand)| UnaryExpression::Unary { operator, operand: Box::new(operand) }),
        sizeof_type.map(UnaryExpression::SizeofType), // sizeof type must be before sizeof expr
        sizeof_expr.map(UnaryExpression::Sizeof),
        alignof_type.map(UnaryExpression::Alignof),
        postfix.map(UnaryExpression::Postfix),
    ))
    .labelled("unary expression")
    .as_context()
}

/// (6.5.4) cast expression
#[apply(cached)]
pub fn cast_expression<'a>() -> impl Parser<'a, &'a [Token], CastExpression, Extra<'a>> + Clone {
    choice((
        type_name()
            .parenthesized()
            .then(cast_expression().map(Box::new))
            .map(|(type_name, expression)| CastExpression::Cast { type_name, expression }),
        unary_expression().map(CastExpression::Unary),
    ))
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
pub fn binary_expression<'a>() -> impl Parser<'a, &'a [Token], Brand<Expression, BinaryExpression>, Extra<'a>> + Clone {
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

    // Suppose precedence is X, use 1000 - 10*X as the associativity level
    choice((
        cast_expression().map(Expression::Cast),
        unary_expression().map(Expression::Unary),
        postfix_expression().map(Expression::Postfix),
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
    .map(Brand::new)
    .labelled("binary expression")
    .as_context()
}

/// (6.5.15) conditional expression
#[apply(cached)]
pub fn conditional_expression<'a>()
-> impl Parser<'a, &'a [Token], Brand<Expression, ConditionalExpression>, Extra<'a>> + Clone {
    choice((
        binary_expression()
            .then_ignore(punctuator(Punctuator::Question))
            .then(expression())
            .then_ignore(punctuator(Punctuator::Colon))
            .then(conditional_expression().map(Brand::into_inner))
            .map(|((condition, then_expr), else_expr)| {
                Expression::Conditional(ConditionalExpression {
                    condition: Box::new(condition.into_inner()),
                    then_expr: Box::new(then_expr),
                    else_expr: Box::new(else_expr),
                })
            }),
        binary_expression().map(Brand::into_inner),
    ))
    .map(Brand::new)
    .labelled("conditional expression")
    .as_context()
}

/// (6.5.16) assignment expression
#[apply(cached)]
pub fn assignment_expression<'a>()
-> impl Parser<'a, &'a [Token], Brand<Expression, AssignmentExpression>, Extra<'a>> + Clone {
    let assigment_opeartor = select! {
        Token::Punctuator(Punctuator::Assign) => AssignmentOperator::Assign,
        Token::Punctuator(Punctuator::AddAssign) => AssignmentOperator::AddAssign,
        Token::Punctuator(Punctuator::SubAssign) => AssignmentOperator::SubAssign,
        Token::Punctuator(Punctuator::MulAssign) => AssignmentOperator::MulAssign,
        Token::Punctuator(Punctuator::DivAssign) => AssignmentOperator::DivAssign,
        Token::Punctuator(Punctuator::ModAssign) => AssignmentOperator::ModAssign,
        Token::Punctuator(Punctuator::AndAssign) => AssignmentOperator::AndAssign,
        Token::Punctuator(Punctuator::OrAssign) => AssignmentOperator::OrAssign,
        Token::Punctuator(Punctuator::XorAssign) => AssignmentOperator::XorAssign,
        Token::Punctuator(Punctuator::LeftShiftAssign) => AssignmentOperator::LeftShiftAssign,
        Token::Punctuator(Punctuator::RightShiftAssign) => AssignmentOperator::RightShiftAssign,
    };
    choice((
        unary_expression()
            .map(Expression::Unary)
            .then(assigment_opeartor)
            .then(assignment_expression().map(Brand::into_inner))
            .map(|((left, operator), right)| {
                Expression::Assignment(AssignmentExpression {
                    operator,
                    left: Box::new(left),
                    right: Box::new(right),
                })
            }),
        conditional_expression().map(Brand::into_inner),
    ))
    .map(Brand::new)
    .labelled("assignment expression")
    .as_context()
}

/// (6.5.17) expression
#[apply(cached)]
pub fn expression<'a>() -> impl Parser<'a, &'a [Token], Expression, Extra<'a>> + Clone {
    assignment_expression()
        .map(Brand::into_inner)
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
}

/// (6.6) constant expression
#[apply(cached)]
pub fn constant_expression<'a>() -> impl Parser<'a, &'a [Token], ConstantExpression, Extra<'a>> + Clone {
    conditional_expression()
        .map(Brand::into_inner)
        .map(Box::new)
        .map(ConstantExpression)
        .labelled("constant expression")
        .as_context()
}

// =============================================================================
// Declarations
// =============================================================================

/// (6.7) declaration
#[apply(cached)]
pub fn declaration<'a>() -> impl Parser<'a, &'a [Token], Declaration, Extra<'a>> + Clone {
    let normal = attribute_specifier_sequence()
        .then(declaration_specifiers())
        .then(init_declarator_list().or_not().map(Option::unwrap_or_default))
        .then_ignore(punctuator(Punctuator::Semicolon))
        .map(|((mut attributes, (specifiers, attributes_after)), declarators)| {
            attributes.extend(attributes_after);
            Declaration::Normal { attributes, specifiers, declarators }
        });

    let typedef = attribute_specifier_sequence()
        .then(declaration_specifiers_with_typedef())
        .then(typedef_declarator_list().or_not().map(Option::unwrap_or_default))
        .then_ignore(punctuator(Punctuator::Semicolon))
        .map(|((mut attributes, (specifiers, attributes_after)), declarators)| {
            attributes.extend(attributes_after);
            Declaration::Typedef { attributes, specifiers, declarators }
        });

    let static_assert = static_assert_declaration();

    let attribute = choice((
        old_fashioned_attribute_specifier()
            .repeated()
            .at_least(1)
            .collect::<Vec<AttributeSpecifier>>(),
        attribute_specifier_sequence().then_ignore(punctuator(Punctuator::Semicolon)),
    ));

    choice((
        static_assert.map(Declaration::StaticAssert),
        normal,
        typedef,
        attribute.map(Declaration::Attribute),
    ))
    .labelled("declaration")
    .as_context()
}

/// (6.7) declaration specifiers (without typedef)
pub fn declaration_specifiers<'a>()
-> impl Parser<'a, &'a [Token], (DeclarationSpecifiers, Vec<AttributeSpecifier>), Extra<'a>> + Clone {
    declaration_specifier()
        .repeated()
        .at_least(1)
        .collect::<Vec<DeclarationSpecifier>>()
        .then(attribute_specifier_sequence())
        .map(|(specifiers, attrs)| (DeclarationSpecifiers { specifiers }, attrs))
        .labelled("declaration specifiers")
        .as_context()
}

/// (6.7) declaration specifiers (with typedef)
pub fn declaration_specifiers_with_typedef<'a>()
-> impl Parser<'a, &'a [Token], (DeclarationSpecifiers, Vec<AttributeSpecifier>), Extra<'a>> + Clone {
    declaration_specifier()
        .or(keyword("typedef").to(DeclarationSpecifier::StorageClass(StorageClassSpecifier::Typedef)))
        .repeated()
        .at_least(1)
        .collect::<Vec<DeclarationSpecifier>>()
        .then(attribute_specifier_sequence())
        .map(|(specifiers, attrs)| (DeclarationSpecifiers { specifiers }, attrs))
        .labelled("declaration specifiers")
        .as_context()
}

/// (6.7) declaration specifier (without typedef)
#[apply(cached)]
pub fn declaration_specifier<'a>() -> impl Parser<'a, &'a [Token], DeclarationSpecifier, Extra<'a>> + Clone {
    choice((
        storage_class_specifier().map(DeclarationSpecifier::StorageClass),
        type_specifier_qualifier().map(DeclarationSpecifier::TypeSpecifierQualifier),
        function_specifier()
            .then(attribute_specifier_sequence())
            .map(|(specifier, attributes)| DeclarationSpecifier::Function { specifier, attributes }),
    ))
    .labelled("declaration specifier")
    .as_context()
}

/// (6.7) init declarator list
pub fn init_declarator_list<'a>() -> impl Parser<'a, &'a [Token], Vec<InitDeclarator>, Extra<'a>> + Clone {
    init_declarator()
        .separated_by(punctuator(Punctuator::Comma))
        .at_least(1)
        .collect::<Vec<InitDeclarator>>()
        .labelled("init declarator list")
        .as_context()
}

/// (6.7) init declarator
pub fn init_declarator<'a>() -> impl Parser<'a, &'a [Token], InitDeclarator, Extra<'a>> + Clone {
    declarator()
        .then(punctuator(Punctuator::Assign).ignore_then(initializer()).or_not())
        .map(|(declarator, initializer)| InitDeclarator { declarator, initializer })
        .labelled("init declarator")
        .as_context()
}

pub fn typedef_declarator_list<'a>() -> impl Parser<'a, &'a [Token], Vec<Declarator>, Extra<'a>> + Clone {
    typedef_declarator()
        .separated_by(punctuator(Punctuator::Comma))
        .at_least(1)
        .collect::<Vec<Declarator>>()
        .labelled("init declarator list")
        .as_context()
}

pub fn typedef_declarator<'a>() -> impl Parser<'a, &'a [Token], Declarator, Extra<'a>> + Clone {
    declarator()
        .map_with(move |declarator, extra| {
            extra
                .state()
                .ctx_mut()
                .add_typedef_name(declarator.identifier().clone());
            declarator
        })
        .labelled("init declarator")
        .as_context()
}

/// (6.7.1) storage class specifier (without typedef)
#[apply(cached)]
pub fn storage_class_specifier<'a>() -> impl Parser<'a, &'a [Token], StorageClassSpecifier, Extra<'a>> + Clone {
    choice((
        keyword("auto").to(StorageClassSpecifier::Auto),
        keyword("constexpr").to(StorageClassSpecifier::Constexpr),
        keyword("extern").to(StorageClassSpecifier::Extern),
        keyword("register").to(StorageClassSpecifier::Register),
        keyword("static").to(StorageClassSpecifier::Static),
        keyword("thread_local").to(StorageClassSpecifier::ThreadLocal),
        // keyword("typedef").to(StorageClassSpecifier::Typedef),
    ))
    .labelled("storage class specifier")
    .as_context()
}

/// (6.7.2) type specifier
pub fn type_specifier<'a>() -> impl Parser<'a, &'a [Token], TypeSpecifier, Extra<'a>> + Clone {
    choice((
        keyword("void").to(TypeSpecifier::Void),
        keyword("char").to(TypeSpecifier::Char),
        keyword("short").to(TypeSpecifier::Short),
        keyword("int").to(TypeSpecifier::Int),
        keyword("long").to(TypeSpecifier::Long),
        keyword("float").to(TypeSpecifier::Float),
        keyword("double").to(TypeSpecifier::Double),
        keyword("signed").to(TypeSpecifier::Signed),
        keyword("unsigned").to(TypeSpecifier::Unsigned),
        keyword("bool").to(TypeSpecifier::Bool),
        keyword("_Complex").to(TypeSpecifier::Complex),
        keyword("_Decimal32").to(TypeSpecifier::Decimal32),
        keyword("_Decimal64").to(TypeSpecifier::Decimal64),
        keyword("_Decimal128").to(TypeSpecifier::Decimal128),
        keyword("_BitInt")
            .ignore_then(constant_expression().parenthesized())
            .map(TypeSpecifier::BitInt),
        struct_or_union_specifier().map(TypeSpecifier::Struct),
        enum_specifier().map(TypeSpecifier::Enum),
        typeof_specifier().map(TypeSpecifier::Typeof),
        typedef_name().map(TypeSpecifier::TypedefName), // Must be last to avoid conflicts
    ))
    .labelled("type specifier")
    .as_context()
}

/// (6.7.2.1) struct or union specifier
pub fn struct_or_union_specifier<'a>() -> impl Parser<'a, &'a [Token], StructOrUnionSpecifier, Extra<'a>> + Clone {
    let struct_or_union = choice((
        keyword("struct").to(StructOrUnion::Struct),
        keyword("union").to(StructOrUnion::Union),
    ));

    struct_or_union
        .then(attribute_specifier_sequence())
        .then(identifier().or_not())
        .then(member_declaration_list().braced().or_not())
        .map(|(((kind, attributes), identifier), members)| StructOrUnionSpecifier {
            kind,
            attributes,
            identifier,
            members,
        })
        .labelled("struct or union specifier")
        .as_context()
}

/// (6.7.2.1) member declaration list
pub fn member_declaration_list<'a>() -> impl Parser<'a, &'a [Token], Vec<MemberDeclaration>, Extra<'a>> + Clone {
    member_declaration()
        .repeated()
        .collect::<Vec<MemberDeclaration>>()
        .labelled("member declaration list")
        .as_context()
}

/// (6.7.2.1) member declaration
pub fn member_declaration<'a>() -> impl Parser<'a, &'a [Token], MemberDeclaration, Extra<'a>> + Clone {
    let static_assert = static_assert_declaration().map(MemberDeclaration::StaticAssert);

    let normal = attribute_specifier_sequence()
        .then(specifier_qualifier_list())
        .then(member_declarator_list().or_not().map(Option::unwrap_or_default))
        .then_ignore(punctuator(Punctuator::Semicolon))
        .map(|((attributes, specifiers), declarators)| MemberDeclaration::Normal {
            attributes,
            specifiers,
            declarators,
        });

    choice((static_assert, normal))
        .labelled("member declaration")
        .as_context()
}

/// (6.7.2.1) specifier qualifier list
#[apply(cached)]
pub fn specifier_qualifier_list<'a>() -> impl Parser<'a, &'a [Token], SpecifierQualifierList, Extra<'a>> + Clone {
    type_specifier_qualifier()
        .repeated()
        .at_least(1)
        .collect::<Vec<TypeSpecifierQualifier>>()
        .then(attribute_specifier_sequence())
        .map(|(items, attributes)| SpecifierQualifierList { items, attributes })
        .labelled("specifier qualifier list")
        .as_context()
}

/// (6.7.2.1) type specifier qualifier
#[apply(cached)]
pub fn type_specifier_qualifier<'a>() -> impl Parser<'a, &'a [Token], TypeSpecifierQualifier, Extra<'a>> + Clone {
    choice((
        type_specifier().map(TypeSpecifierQualifier::TypeSpecifier),
        type_qualifier().map(TypeSpecifierQualifier::TypeQualifier),
        alignment_specifier().map(TypeSpecifierQualifier::AlignmentSpecifier),
    ))
    .labelled("type specifier qualifier")
    .as_context()
}

/// (6.7.2.1) member declarator list
pub fn member_declarator_list<'a>() -> impl Parser<'a, &'a [Token], Vec<MemberDeclarator>, Extra<'a>> + Clone {
    member_declarator()
        .separated_by(punctuator(Punctuator::Comma))
        .at_least(1)
        .collect::<Vec<MemberDeclarator>>()
        .labelled("member declarator list")
        .as_context()
}

/// (6.7.2.1) member declarator
pub fn member_declarator<'a>() -> impl Parser<'a, &'a [Token], MemberDeclarator, Extra<'a>> + Clone {
    choice((
        declarator()
            .or_not()
            .then_ignore(punctuator(Punctuator::Colon))
            .then(constant_expression())
            .map(|(declarator, width)| MemberDeclarator::BitField { declarator, width }),
        declarator().map(MemberDeclarator::Declarator),
    ))
    .labelled("member declarator")
    .as_context()
}

/// (6.7.2.2) enum specifier
pub fn enum_specifier<'a>() -> impl Parser<'a, &'a [Token], EnumSpecifier, Extra<'a>> + Clone {
    keyword("enum")
        .ignore_then(attribute_specifier_sequence())
        .then(identifier().or_not())
        .then(
            punctuator(Punctuator::Colon)
                .ignore_then(specifier_qualifier_list())
                .or_not(),
        )
        .then(enumerator_list().braced().or_not())
        .map(
            |(((attributes, identifier), type_specifier), enumerators)| EnumSpecifier {
                attributes,
                identifier,
                type_specifier,
                enumerators,
            },
        )
        .labelled("enum specifier")
        .as_context()
}

/// (6.7.2.2) enumerator list
pub fn enumerator_list<'a>() -> impl Parser<'a, &'a [Token], Vec<Enumerator>, Extra<'a>> + Clone {
    enumerator()
        .map_with(|enumerator, extra| {
            extra.state().ctx_mut().add_enum_constant(enumerator.name.clone());
            enumerator
        })
        .separated_by(punctuator(Punctuator::Comma))
        .allow_trailing()
        .collect::<Vec<Enumerator>>()
        .labelled("enumerator list")
        .as_context()
}

/// (6.7.2.2) enumerator
pub fn enumerator<'a>() -> impl Parser<'a, &'a [Token], Enumerator, Extra<'a>> + Clone {
    identifier()
        .then(attribute_specifier_sequence())
        .then(
            punctuator(Punctuator::Assign)
                .ignore_then(constant_expression())
                .or_not(),
        )
        .map(|((name, attributes), value)| Enumerator { name, attributes, value })
        .labelled("enumerator")
        .as_context()
}

/// (6.7.2.4) atomic type specifier
pub fn atomic_type_specifier<'a>() -> impl Parser<'a, &'a [Token], AtomicTypeSpecifier, Extra<'a>> + Clone {
    keyword("_Atomic")
        .ignore_then(type_name().parenthesized())
        .map(|type_name| AtomicTypeSpecifier { type_name })
        .labelled("atomic type specifier")
        .as_context()
}

/// (6.7.2.5) typeof specifier
pub fn typeof_specifier<'a>() -> impl Parser<'a, &'a [Token], TypeofSpecifier, Extra<'a>> + Clone {
    let typeof_arg = choice((
        expression().map(Box::new).map(TypeofSpecifierArgument::Expression),
        type_name().map(TypeofSpecifierArgument::TypeName),
    ));

    choice((
        keyword("typeof")
            .ignore_then(typeof_arg.clone().parenthesized())
            .map(TypeofSpecifier::Typeof),
        keyword("typeof_unqual")
            .ignore_then(typeof_arg.parenthesized())
            .map(TypeofSpecifier::TypeofUnqual),
    ))
    .labelled("typeof specifier")
    .as_context()
}

/// (6.7.3) type qualifier
#[apply(cached)]
pub fn type_qualifier<'a>() -> impl Parser<'a, &'a [Token], TypeQualifier, Extra<'a>> + Clone {
    choice((
        keyword("const").to(TypeQualifier::Const),
        keyword("restrict").to(TypeQualifier::Restrict),
        keyword("volatile").to(TypeQualifier::Volatile),
        keyword("_Atomic").to(TypeQualifier::Atomic),
        keyword("_Nonnull").to(TypeQualifier::Nonnull),
        keyword("_Nullable").to(TypeQualifier::Nullable),
        keyword("_Thread_local").to(TypeQualifier::ThreadLocal),
    ))
    .labelled("type qualifier")
    .as_context()
}

/// (6.7.4) function specifier
pub fn function_specifier<'a>() -> impl Parser<'a, &'a [Token], FunctionSpecifier, Extra<'a>> + Clone {
    choice((
        keyword("inline").to(FunctionSpecifier::Inline),
        keyword("_Noreturn").to(FunctionSpecifier::Noreturn),
    ))
    .labelled("function specifier")
    .as_context()
}

/// (6.7.5) alignment specifier
pub fn alignment_specifier<'a>() -> impl Parser<'a, &'a [Token], AlignmentSpecifier, Extra<'a>> + Clone {
    keyword("alignas")
        .ignore_then(
            choice((
                constant_expression().map(AlignmentSpecifier::Expression),
                type_name().map(AlignmentSpecifier::Type),
            ))
            .parenthesized(),
        )
        .labelled("alignment specifier")
        .as_context()
}

/// (6.7.6) declarator
#[apply(cached)]
pub fn declarator<'a>() -> impl Parser<'a, &'a [Token], Declarator, Extra<'a>> + Clone {
    let pointer = pointer()
        .then(declarator().map(Box::new))
        .map(|(pointer, declarator)| Declarator::Pointer { pointer, declarator });
    let direct = direct_declarator().map(Declarator::Direct);
    choice((pointer, direct)).labelled("declarator").as_context()
}

/// (6.7.6) direct declarator
#[apply(cached)]
pub fn direct_declarator<'a>() -> impl Parser<'a, &'a [Token], DirectDeclarator, Extra<'a>> + Clone {
    let identifier_decl = identifier()
        .then(attribute_specifier_sequence())
        .map(|(identifier, attributes)| DirectDeclarator::Identifier { identifier, attributes });

    let parenthesized = declarator()
        .parenthesized()
        .map(Box::new)
        .map(DirectDeclarator::Parenthesized);

    type DirectDeclaratorFn = Box<dyn FnOnce(DirectDeclarator) -> DirectDeclarator>;
    let base = choice((identifier_decl, parenthesized));
    base.foldl(
        choice((
            array_declarator().then(attribute_specifier_sequence()).map(
                |(array_declarator, attributes)| -> DirectDeclaratorFn {
                    Box::new(move |declarator| DirectDeclarator::Array {
                        declarator: Box::new(declarator),
                        attributes,
                        array_declarator,
                    })
                },
            ),
            parameter_type_list()
                .parenthesized()
                .then(attribute_specifier_sequence())
                .map(|(parameters, attributes)| -> DirectDeclaratorFn {
                    Box::new(move |declarator| DirectDeclarator::Function {
                        declarator: Box::new(declarator),
                        attributes,
                        parameters,
                    })
                }),
        ))
        .repeated(),
        |acc, f| f(acc),
    )
    .labelled("direct declarator")
    .as_context()
}

/// (6.7.6) array declarator
pub fn array_declarator<'a>() -> impl Parser<'a, &'a [Token], ArrayDeclarator, Extra<'a>> + Clone {
    choice((
        keyword("static")
            .ignore_then(type_qualifier_list().or_not().map(Option::unwrap_or_default))
            .then(assignment_expression().map(Brand::into_inner).map(Box::new))
            .map(|(type_qualifiers, size)| ArrayDeclarator::Static { type_qualifiers, size }),
        type_qualifier_list()
            .then_ignore(keyword("static"))
            .then(assignment_expression().map(Brand::into_inner).map(Box::new))
            .map(|(type_qualifiers, size)| ArrayDeclarator::Static { type_qualifiers, size }),
        type_qualifier_list()
            .or_not()
            .map(Option::unwrap_or_default)
            .then_ignore(punctuator(Punctuator::Star))
            .map(|type_qualifiers| ArrayDeclarator::VLA { type_qualifiers }),
        type_qualifier_list()
            .or_not()
            .map(Option::unwrap_or_default)
            .then(assignment_expression().map(Brand::into_inner).map(Box::new).or_not())
            .map(|(type_qualifiers, size)| ArrayDeclarator::Normal { type_qualifiers, size }),
    ))
    .bracketed()
    .labelled("array declarator")
    .as_context()
}

/// (6.7.6) pointer
#[apply(cached)]
pub fn pointer<'a>() -> impl Parser<'a, &'a [Token], Pointer, Extra<'a>> + Clone {
    choice((
        punctuator(Punctuator::Star).to(PointerOrBlock::Pointer),
        punctuator(Punctuator::Caret).to(PointerOrBlock::Block),
    ))
    .then(attribute_specifier_sequence())
    .then(type_qualifier_list().or_not().map(Option::unwrap_or_default))
    .map(|((pointer_or_block, attributes), type_qualifiers)| Pointer {
        pointer_or_block,
        attributes,
        type_qualifiers,
    })
    .labelled("pointer")
    .as_context()
}

/// (6.7.6) type qualifier list
#[apply(cached)]
pub fn type_qualifier_list<'a>() -> impl Parser<'a, &'a [Token], Vec<TypeQualifier>, Extra<'a>> + Clone {
    type_qualifier()
        .repeated()
        .at_least(1)
        .collect::<Vec<TypeQualifier>>()
        .labelled("type qualifier list")
        .as_context()
}

/// (6.7.6) parameter type list
#[apply(cached)]
pub fn parameter_type_list<'a>() -> impl Parser<'a, &'a [Token], ParameterTypeList, Extra<'a>> + Clone {
    choice((
        punctuator(Punctuator::Ellipsis).to(ParameterTypeList::OnlyVariadic),
        parameter_declaration()
            .separated_by(punctuator(Punctuator::Comma))
            .collect::<Vec<ParameterDeclaration>>()
            .then(
                punctuator(Punctuator::Comma)
                    .ignore_then(punctuator(Punctuator::Ellipsis))
                    .or_not(),
            )
            .map(|(params, variadic)| {
                if variadic.is_some() {
                    ParameterTypeList::Variadic(params)
                } else {
                    ParameterTypeList::Parameters(params)
                }
            }),
    ))
    .labelled("parameter type list")
    .as_context()
}

/// (6.7.6) parameter declaration
pub fn parameter_declaration<'a>() -> impl Parser<'a, &'a [Token], ParameterDeclaration, Extra<'a>> + Clone {
    attribute_specifier_sequence()
        .then(declaration_specifiers())
        .then(choice((
            declarator().map(ParameterDeclarationKind::Declarator).map(Some),
            abstract_declarator().map(ParameterDeclarationKind::Abstract).or_not(),
        )))
        .map(|((mut attributes, (specifiers, attributes_after)), declarator)| {
            attributes.extend(attributes_after);
            ParameterDeclaration { attributes, specifiers, declarator }
        })
        .labelled("parameter declaration")
        .as_context()
}

/// (6.7.7) type name
#[apply(cached)]
pub fn type_name<'a>() -> impl Parser<'a, &'a [Token], TypeName, Extra<'a>> + Clone {
    specifier_qualifier_list()
        .then(abstract_declarator().or_not())
        .map(|(specifiers, abstract_declarator)| TypeName { specifiers, abstract_declarator })
        .labelled("type name")
        .as_context()
}

/// (6.7.7) abstract declarator
#[apply(cached)]
pub fn abstract_declarator<'a>() -> impl Parser<'a, &'a [Token], AbstractDeclarator, Extra<'a>> + Clone {
    let pointer = pointer()
        .then(abstract_declarator().map(Box::new).or_not())
        .map(|(pointer, abstract_declarator)| AbstractDeclarator::Pointer { pointer, abstract_declarator });
    let direct = direct_abstract_declarator().map(AbstractDeclarator::Direct);
    choice((pointer, direct)).labelled("abstract declarator").as_context()
}

/// (6.7.7) direct abstract declarator
#[apply(cached)]
pub fn direct_abstract_declarator<'a>() -> impl Parser<'a, &'a [Token], DirectAbstractDeclarator, Extra<'a>> + Clone {
    let parenthesized = abstract_declarator()
        .parenthesized()
        .map(Box::new)
        .map(DirectAbstractDeclarator::Parenthesized);

    type DirectAbstractDeclaratorFn = Box<dyn FnOnce(Option<DirectAbstractDeclarator>) -> DirectAbstractDeclarator>;
    let postfix = choice((
        array_declarator().then(attribute_specifier_sequence()).map(
            |(array_declarator, attributes)| -> DirectAbstractDeclaratorFn {
                Box::new(move |declarator| {
                    let declarator = declarator.map(Box::new);
                    DirectAbstractDeclarator::Array { declarator, attributes, array_declarator }
                })
            },
        ),
        parameter_type_list()
            .parenthesized()
            .then(attribute_specifier_sequence())
            .map(|(parameters, attributes)| -> DirectAbstractDeclaratorFn {
                Box::new(move |declarator| {
                    let declarator = declarator.map(Box::new);
                    DirectAbstractDeclarator::Function { declarator, attributes, parameters }
                })
            }),
    ))
    .repeated();
    choice((
        parenthesized.map(Some).foldl(postfix.clone(), |acc, f| Some(f(acc))),
        empty().to(None).foldl(postfix.at_least(1), |acc, f| Some(f(acc))),
    ))
    .unwrapped()
    .labelled("direct abstract declarator")
    .as_context()
}

/// (6.7.8) typedef name
pub fn typedef_name<'a>() -> impl Parser<'a, &'a [Token], Identifier, Extra<'a>> + Clone {
    identifier()
        .try_map_with(|name, extra| {
            if extra.state().ctx().is_typedef_name(&name) {
                Ok(name)
            } else {
                Err(expected_found(
                    ["typedef name"],
                    Some(BalancedToken::Identifier(name)),
                    extra.span(),
                ))
            }
        })
        .labelled("typedef name")
        .as_context()
}

/// (6.7.10) braced initializer
#[apply(cached)]
pub fn braced_initializer<'a>() -> impl Parser<'a, &'a [Token], BracedInitializer, Extra<'a>> + Clone {
    designated_initializer()
        .separated_by(punctuator(Punctuator::Comma))
        .allow_trailing()
        .collect::<Vec<DesignatedInitializer>>()
        .braced()
        .map(|initializers| BracedInitializer { initializers })
        .labelled("braced initializer")
        .as_context()
}

/// (6.7.10) initializer
#[apply(cached)]
pub fn initializer<'a>() -> impl Parser<'a, &'a [Token], Initializer, Extra<'a>> + Clone {
    choice((
        braced_initializer().map(Initializer::Braced),
        assignment_expression()
            .map(Brand::into_inner)
            .map(Box::new)
            .map(Initializer::Expression),
    ))
    .labelled("initializer")
    .as_context()
}

/// (6.7.10) designated initializer
pub fn designated_initializer<'a>() -> impl Parser<'a, &'a [Token], DesignatedInitializer, Extra<'a>> + Clone {
    designation()
        .or_not()
        .then(initializer())
        .map(|(designation, initializer)| DesignatedInitializer { designation, initializer })
        .labelled("designated initializer")
        .as_context()
}

/// (6.7.10) designation
pub fn designation<'a>() -> impl Parser<'a, &'a [Token], Designation, Extra<'a>> + Clone {
    empty()
        .to(None)
        .foldl(designator().repeated().at_least(1), |designation, designator| {
            Some(Designation {
                designator,
                designation: designation.map(Box::new),
            })
        })
        .unwrapped()
        .then_ignore(punctuator(Punctuator::Assign))
        .labelled("designation")
        .as_context()
}

/// (6.7.10) designator
pub fn designator<'a>() -> impl Parser<'a, &'a [Token], Designator, Extra<'a>> + Clone {
    choice((
        expression().bracketed().map(Box::new).map(Designator::Array),
        punctuator(Punctuator::Dot)
            .ignore_then(identifier())
            .map(Designator::Member),
    ))
    .labelled("designator")
    .as_context()
}

/// (6.7.11) static assert declaration
pub fn static_assert_declaration<'a>() -> impl Parser<'a, &'a [Token], StaticAssertDeclaration, Extra<'a>> + Clone {
    keyword("static_assert")
        .or(keyword("_Static_assert"))
        .ignore_then(
            constant_expression()
                .then(punctuator(Punctuator::Comma).ignore_then(string_literal()).or_not())
                .parenthesized(),
        )
        .then_ignore(punctuator(Punctuator::Semicolon))
        .map(|(condition, message)| StaticAssertDeclaration { condition, message })
        .labelled("static assert declaration")
        .as_context()
}

// =============================================================================
// Statements
// =============================================================================

/// (6.8) statement
#[apply(cached)]
pub fn statement<'a>() -> impl Parser<'a, &'a [Token], Statement, Extra<'a>> + Clone {
    choice((
        labelled_statement().map(Statement::Labeled),
        unlabeled_statement().map(Statement::Unlabeled),
    ))
    .labelled("statement")
    .as_context()
}

/// (6.8) unlabeled statement
#[apply(cached)]
pub fn unlabeled_statement<'a>() -> impl Parser<'a, &'a [Token], UnlabeledStatement, Extra<'a>> + Clone {
    let primary_block = attribute_specifier_sequence()
        .then(choice((
            compound_statement().map(PrimaryBlock::Compound),
            selection_statement().map(PrimaryBlock::Selection),
            iteration_statement().map(PrimaryBlock::Iteration),
        )))
        .map(|(attributes, block)| UnlabeledStatement::Primary { attributes, block });

    let jump = attribute_specifier_sequence()
        .then(jump_statement())
        .map(|(attributes, statement)| UnlabeledStatement::Jump { attributes, statement });

    let expr = expression_statement().map(UnlabeledStatement::Expression);

    choice((primary_block, jump, expr))
        .labelled("unlabeled statement")
        .as_context()
}

/// (6.8.1) label
#[apply(cached)]
pub fn label<'a>() -> impl Parser<'a, &'a [Token], Label, Extra<'a>> + Clone {
    let case_label = attribute_specifier_sequence()
        .then(keyword("case").ignore_then(constant_expression()))
        .then_ignore(punctuator(Punctuator::Colon))
        .map(|(attributes, expression)| Label::Case { attributes, expression });

    let default_label = attribute_specifier_sequence()
        .then(keyword("default"))
        .then_ignore(punctuator(Punctuator::Colon))
        .map(|(attributes, _)| Label::Default { attributes });

    let ident_label = attribute_specifier_sequence()
        .then(identifier())
        .then_ignore(punctuator(Punctuator::Colon))
        .map(|(attributes, identifier)| Label::Identifier { attributes, identifier });

    choice((case_label, default_label, ident_label))
        .labelled("label")
        .as_context()
}

/// (6.8.1) labeled statement
pub fn labelled_statement<'a>() -> impl Parser<'a, &'a [Token], LabeledStatement, Extra<'a>> + Clone {
    label()
        .then(statement().map(Box::new))
        .map(|(label, statement)| LabeledStatement { label, statement })
        .labelled("labeled statement")
        .as_context()
}

/// (6.8.2) compound statement
#[apply(cached)]
pub fn compound_statement<'a>() -> impl Parser<'a, &'a [Token], CompoundStatement, Extra<'a>> + Clone {
    block_item()
        .repeated()
        .collect::<Vec<BlockItem>>()
        .braced()
        .map(|items| CompoundStatement { items })
        .labelled("compound statement")
        .as_context()
}

/// (6.8.2) block item
pub fn block_item<'a>() -> impl Parser<'a, &'a [Token], BlockItem, Extra<'a>> + Clone {
    choice((
        declaration().map(BlockItem::Declaration),
        label().map(BlockItem::Label),
        unlabeled_statement().map(BlockItem::Statement),
    ))
    .labelled("block item")
    .as_context()
}

/// (6.8.3) expression statement
pub fn expression_statement<'a>() -> impl Parser<'a, &'a [Token], ExpressionStatement, Extra<'a>> + Clone {
    attribute_specifier_sequence()
        .then(expression().map(Box::new).or_not())
        .then_ignore(punctuator(Punctuator::Semicolon))
        .map(|(attributes, expression)| ExpressionStatement { attributes, expression })
        .labelled("expression statement")
        .as_context()
}

/// (6.8.4) selection statement
pub fn selection_statement<'a>() -> impl Parser<'a, &'a [Token], SelectionStatement, Extra<'a>> + Clone {
    let if_stmt = keyword("if")
        .ignore_then(expression().map(Box::new).parenthesized())
        .then(statement().map(Box::new))
        .then(keyword("else").ignore_then(statement().map(Box::new)).or_not())
        .map(|((condition, then_stmt), else_stmt)| SelectionStatement::If { condition, then_stmt, else_stmt });

    let switch_stmt = keyword("switch")
        .ignore_then(expression().map(Box::new).parenthesized())
        .then(statement().map(Box::new))
        .map(|(expression, statement)| SelectionStatement::Switch { expression, statement });

    choice((if_stmt, switch_stmt))
        .labelled("selection statement")
        .as_context()
}

/// (6.8.5) iteration statement
pub fn iteration_statement<'a>() -> impl Parser<'a, &'a [Token], IterationStatement, Extra<'a>> + Clone {
    let while_stmt = keyword("while")
        .ignore_then(expression().map(Box::new).parenthesized())
        .then(statement().map(Box::new))
        .map(|(condition, body)| IterationStatement::While { condition, body });

    let do_while_stmt = keyword("do")
        .ignore_then(statement().map(Box::new))
        .then_ignore(keyword("while"))
        .then(expression().map(Box::new).parenthesized())
        .then_ignore(punctuator(Punctuator::Semicolon))
        .map(|(body, condition)| IterationStatement::DoWhile { body, condition });

    let for_stmt = keyword("for")
        .ignore_then(
            choice((
                expression()
                    .map(Box::new)
                    .map(ForInit::Expression)
                    .or_not()
                    .then_ignore(punctuator(Punctuator::Semicolon)),
                declaration().map(ForInit::Declaration).map(Some),
            ))
            .then(expression().map(Box::new).or_not())
            .then_ignore(punctuator(Punctuator::Semicolon))
            .then(expression().map(Box::new).or_not())
            .parenthesized(),
        )
        .then(statement().map(Box::new))
        .map(|(((init, condition), update), body)| IterationStatement::For { init, condition, update, body });

    choice((while_stmt, do_while_stmt, for_stmt))
        .labelled("iteration statement")
        .as_context()
}

/// (6.8.6) jump statement
pub fn jump_statement<'a>() -> impl Parser<'a, &'a [Token], JumpStatement, Extra<'a>> + Clone {
    let goto_stmt = keyword("goto")
        .ignore_then(identifier())
        .then_ignore(punctuator(Punctuator::Semicolon))
        .map(JumpStatement::Goto);

    let continue_stmt = keyword("continue")
        .then_ignore(punctuator(Punctuator::Semicolon))
        .to(JumpStatement::Continue);

    let break_stmt = keyword("break")
        .then_ignore(punctuator(Punctuator::Semicolon))
        .to(JumpStatement::Break);

    let return_stmt = keyword("return")
        .ignore_then(expression().or_not())
        .then_ignore(punctuator(Punctuator::Semicolon))
        .map(|expr| JumpStatement::Return(expr.map(Box::new)));

    choice((goto_stmt, continue_stmt, break_stmt, return_stmt))
        .labelled("jump statement")
        .as_context()
}

// =============================================================================
// Attributes
// =============================================================================

/// (6.7.12.1) attribute specifier sequence
#[apply(cached)]
pub fn attribute_specifier_sequence<'a>() -> impl Parser<'a, &'a [Token], Vec<AttributeSpecifier>, Extra<'a>> + Clone {
    attribute_specifier()
        .repeated()
        .collect::<Vec<AttributeSpecifier>>()
        .labelled("attribute specifier sequence")
        .as_context()
}

/// (6.7.12.1) attribute specifier
pub fn attribute_specifier<'a>() -> impl Parser<'a, &'a [Token], AttributeSpecifier, Extra<'a>> + Clone {
    choice((
        old_fashioned_attribute_specifier(),
        asm_attribute_specifier(),
        attribute_list()
            .bracketed()
            .bracketed()
            .map(AttributeSpecifier::Attributes),
    ))
    .labelled("attribute specifier")
    .as_context()
}

#[apply(cached)]
pub fn old_fashioned_attribute_specifier<'a>() -> impl Parser<'a, &'a [Token], AttributeSpecifier, Extra<'a>> + Clone {
    keyword("__attribute__")
        .ignore_then(attribute_list().parenthesized().parenthesized())
        .map(AttributeSpecifier::Attributes)
        .labelled("old fashioned attribute specifier")
        .as_context()
}

pub fn asm_attribute_specifier<'a>() -> impl Parser<'a, &'a [Token], AttributeSpecifier, Extra<'a>> + Clone {
    keyword("__asm")
        .ignore_then(string_literal().parenthesized())
        .map(AttributeSpecifier::Asm)
        .labelled("asm attribute specifier")
        .as_context()
}

/// (6.7.12.1) attribute list
pub fn attribute_list<'a>() -> impl Parser<'a, &'a [Token], Vec<Attribute>, Extra<'a>> + Clone {
    attribute()
        .separated_by(punctuator(Punctuator::Comma))
        .allow_trailing()
        .collect::<Vec<Attribute>>()
        .labelled("attribute list")
        .as_context()
}

/// (6.7.12.1) attribute
#[apply(cached)]
pub fn attribute<'a>() -> impl Parser<'a, &'a [Token], Attribute, Extra<'a>> + Clone {
    attribute_token()
        .then(attribute_argument_clause().or_not())
        .map(|(token, arguments)| Attribute {
            token,
            arguments: arguments.unwrap_or_default(),
        })
        .labelled("attribute")
        .as_context()
}

/// (6.7.12.1) attribute token
pub fn attribute_token<'a>() -> impl Parser<'a, &'a [Token], AttributeToken, Extra<'a>> + Clone {
    let standard = identifier();
    let prefixed = identifier()
        .then_ignore(punctuator(Punctuator::Scope))
        .then(identifier());

    choice((
        prefixed.map(|(prefix, identifier)| AttributeToken::Prefixed { prefix, identifier }),
        standard.map(AttributeToken::Standard),
    ))
    .labelled("attribute token")
    .as_context()
}

/// (6.7.12.1) attribute argument clause
pub fn attribute_argument_clause<'a>() -> impl Parser<'a, &'a [Token], TokenStream, Extra<'a>> + Clone {
    select! {
        Token::Parenthesized(tokens) => tokens
    }
}

// =============================================================================
// Translation Units
// =============================================================================

/// (6.9) translation unit
pub fn translation_unit<'a>() -> impl Parser<'a, &'a [Token], TranslationUnit, Extra<'a>> + Clone {
    external_declaration()
        .repeated()
        .collect::<Vec<ExternalDeclaration>>()
        .map(|external_declarations| TranslationUnit { external_declarations })
        .labelled("translation unit")
        .as_context()
}

/// (6.9) external declaration
pub fn external_declaration<'a>() -> impl Parser<'a, &'a [Token], ExternalDeclaration, Extra<'a>> + Clone {
    choice((
        function_definition().map(ExternalDeclaration::Function),
        declaration().map(ExternalDeclaration::Declaration),
    ))
    .labelled("external declaration")
    .as_context()
}

/// (6.9.1) function definition
pub fn function_definition<'a>() -> impl Parser<'a, &'a [Token], FunctionDefinition, Extra<'a>> + Clone {
    attribute_specifier_sequence()
        .then(declaration_specifiers())
        .then(declarator())
        .then(compound_statement())
        .map(
            |(((mut attributes, (specifiers, attributes_after)), declarator), body)| {
                attributes.extend(attributes_after);
                FunctionDefinition { attributes, specifiers, declarator, body }
            },
        )
        .labelled("function definition")
        .as_context()
}

// =============================================================================
// Parser utilities
// =============================================================================

fn identifier<'a>() -> impl Parser<'a, &'a [Token], Identifier, Extra<'a>> + Clone {
    select! {
        Token::Identifier(value) => value
    }
}

fn constant<'a>() -> impl Parser<'a, &'a [Token], Constant, Extra<'a>> + Clone {
    select! {
        Token::Constant(value) => value
    }
}

fn string_literal<'a>() -> impl Parser<'a, &'a [Token], StringLiterals, Extra<'a>> + Clone {
    select! {
        Token::StringLiteral(value) => value
    }
}

fn keyword<'a>(kwd: &str) -> impl Parser<'a, &'a [Token], (), Extra<'a>> + Clone {
    select! {
        Token::Identifier(Identifier(name)) if name == kwd => ()
    }
}

fn punctuator<'a>(punc: Punctuator) -> impl Parser<'a, &'a [Token], (), Extra<'a>> + Clone {
    select! {
        Token::Punctuator(p) if p == punc => ()
    }
}

fn expected_found<'a, L>(
    expected: impl IntoIterator<Item = L>,
    found: Option<BalancedToken>,
    span: SimpleSpan,
) -> Rich<'a, Token, SimpleSpan>
where
    L: Into<chumsky::error::RichPattern<'a, Token>>,
{
    use chumsky::label::LabelError;
    use chumsky::util::MaybeRef;
    LabelError::<&'a [Token], L>::expected_found(expected, found.map(MaybeRef::Val), span)
}

trait ParserExt<O, E> {
    fn parenthesized<'a>(self) -> impl Parser<'a, &'a [Token], O, E> + Clone
    where
        Self: Sized,
        Self: Parser<'a, &'a [Token], O, E> + Clone,
        E: chumsky::extra::ParserExtra<'a, &'a [Token]>,
    {
        self.nested_in(select_ref! {
            Token::Parenthesized(tokens) => tokens.as_ref()
        })
    }

    fn bracketed<'a>(self) -> impl Parser<'a, &'a [Token], O, E> + Clone
    where
        Self: Sized,
        Self: Parser<'a, &'a [Token], O, E> + Clone,
        E: chumsky::extra::ParserExtra<'a, &'a [Token]>,
    {
        self.nested_in(select_ref! {
            Token::Bracketed(tokens) => tokens.as_ref()
        })
    }

    fn braced<'a>(self) -> impl Parser<'a, &'a [Token], O, E> + Clone
    where
        Self: Sized,
        Self: Parser<'a, &'a [Token], O, E> + Clone,
        E: chumsky::extra::ParserExtra<'a, &'a [Token]>,
    {
        self.nested_in(select_ref! {
            Token::Braced(tokens) => tokens.as_ref()
        })
    }
}

impl<'a, T, O, E> ParserExt<O, E> for T
where
    T: Parser<'a, &'a [Token], O, E>,
    E: chumsky::extra::ParserExtra<'a, &'a [Token]>,
{
}
