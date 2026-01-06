//! Pretty printer for the AST.

use elegance::{Printer, Render};

use crate::{ast::*, visitor::Visitor};

// pub trait Parenthesize {
//     fn precedence(&self) -> i32;

//     fn parenthesize(&mut self) {
//         self.parenthesize_at(self.precedence())
//     }

//     fn parenthesize_at(&mut self, precedence: i32);
// }

// trait Wrappable: Parenthesize + Sized {
//     fn error() -> Self;
//     fn into_expression(self) -> Expression;
//     fn from_primary(primary: PrimaryExpression) -> Self;

//     fn wrapped(&mut self) {
//         let expr = std::mem::replace(self, Self::error());
//         *self =
// Self::from_primary(PrimaryExpression::Parenthesized(Box::new(expr.
// into_expression())));     }

//     fn wrapped_at(&mut self, precedence: i32) {
//         if precedence < self.precedence() {
//             self.wrapped();
//         }
//     }
// }

// impl Parenthesize for Expression {
//     fn precedence(&self) -> i32 {
//         match self {
//             Expression::Postfix(e) => e.precedence(),
//             Expression::Unary(e) => e.precedence(),
//             Expression::Cast(e) => e.precedence(),
//             Expression::Binary(e) => match e.operator {
//                 BinaryOperator::Multiply | BinaryOperator::Divide |
// BinaryOperator::Modulo => 30,                 BinaryOperator::Add |
// BinaryOperator::Subtract => 40,                 BinaryOperator::LeftShift |
// BinaryOperator::RightShift => 50,                 BinaryOperator::Less
//                 | BinaryOperator::Greater
//                 | BinaryOperator::LessEqual
//                 | BinaryOperator::GreaterEqual => 60,
//                 BinaryOperator::Equal | BinaryOperator::NotEqual => 70,
//                 BinaryOperator::BitwiseAnd => 80,
//                 BinaryOperator::BitwiseXor => 90,
//                 BinaryOperator::BitwiseOr => 100,
//                 BinaryOperator::LogicalAnd => 110,
//                 BinaryOperator::LogicalOr => 120,
//             },
//             Expression::Conditional(_) => 130,
//             Expression::Assignment(_) => 140,
//             Expression::Comma(_) => 150,
//             Expression::Error => todo!(),
//         }
//     }

//     fn parenthesize_at(&mut self, precedence: i32) {
//         let this_prec = self.precedence();
//         match self {
//             Expression::Postfix(e) => e.parenthesize_at(precedence),
//             Expression::Unary(e) => e.parenthesize_at(precedence),
//             Expression::Cast(e) => e.parenthesize_at(precedence),
//             Expression::Binary(e) => {
//                 e.left.parenthesize_at(this_prec);
//                 e.right.parenthesize_at(this_prec - 1);
//                 self.wrapped_at(precedence);
//             }
//             Expression::Conditional(e) => todo!(),
//             Expression::Assignment(e) => todo!(),
//             Expression::Comma(e) => todo!(),
//             Expression::Error => (),
//         }
//     }
// }

// impl Wrappable for Expression {
//     fn error() -> Self {
//         Expression::Postfix(PostfixExpression::Primary(PrimaryExpression::Error))
//     }

//     fn into_expression(self) -> Expression {
//         self
//     }

//     fn from_primary(primary: PrimaryExpression) -> Self {
//         Expression::Postfix(PostfixExpression::Primary(primary))
//     }
// }

// impl Parenthesize for PrimaryExpression {
//     fn precedence(&self) -> i32 {
//         0
//     }

//     fn parenthesize_at(&mut self, _: i32) {
//         match self {
//             // identifier | constant | string-literal
//             PrimaryExpression::Identifier(_)
//             | PrimaryExpression::Constant(_)
//             | PrimaryExpression::EnumerationConstant(_)
//             | PrimaryExpression::StringLiteral(_)
//             | PrimaryExpression::QuotedString(_) => {}
//             // ( expression )
//             PrimaryExpression::Parenthesized(e) => e.parenthesize(),
//             // generic-selection
//             PrimaryExpression::Generic(gs) => gs.parenthesize(),
//             PrimaryExpression::Error => {}
//         }
//     }
// }

// impl Parenthesize for GenericSelection {
//     fn precedence(&self) -> i32 {
//         0
//     }

//     fn parenthesize_at(&mut self, _: i32) {
//         // _Generic( assignment-expression , generic-assoc-list )
//         self.controlling_expression.parenthesize_at(140);
//         for a in &mut self.associations {
//             a.parenthesize();
//         }
//     }
// }

// impl Parenthesize for GenericAssociation {
//     fn precedence(&self) -> i32 {
//         0
//     }

//     fn parenthesize_at(&mut self, _: i32) {
//         match self {
//             // type-name : assignment-expression
//             GenericAssociation::Type { expression, .. } =>
// expression.parenthesize_at(140),             // default :
// assignment-expression             GenericAssociation::Default { expression }
// => expression.parenthesize_at(140),         }
//     }
// }

// impl Parenthesize for PostfixExpression {
//     fn precedence(&self) -> i32 {
//         10
//     }

//     fn parenthesize_at(&mut self, precedence: i32) {
//         match self {
//             // primary-expression
//             PostfixExpression::Primary(e) => e.parenthesize(),
//             // postfix-expression [ expression ]
//             PostfixExpression::ArrayAccess { array, index } => {
//                 array.parenthesize_at(10);
//                 index.parenthesize_at(1000);
//                 self.wrapped_at(precedence);
//             }
//             // postfix-expression ( argument-expression-list opt )
//             PostfixExpression::FunctionCall { function, arguments } => {
//                 function.parenthesize_at(10);
//                 for a in arguments {
//                     a.parenthesize_at(140);
//                 }
//                 self.wrapped_at(precedence);
//             }
//             // postfix-expression . identifier
//             PostfixExpression::MemberAccess { object, .. } => {
//                 object.parenthesize_at(10);
//                 self.wrapped_at(precedence);
//             }
//             // postfix-expression -> identifier
//             PostfixExpression::MemberAccessPtr { object, .. } => {
//                 object.parenthesize_at(10);
//                 self.wrapped_at(precedence);
//             }
//             // postfix-expression ++
//             PostfixExpression::PostIncrement(e) => {
//                 e.parenthesize_at(10);
//                 self.wrapped_at(precedence);
//             }
//             // postfix-expression --
//             PostfixExpression::PostDecrement(e) => {
//                 e.parenthesize_at(10);
//                 self.wrapped_at(precedence);
//             }
//             // compound-literal
//             PostfixExpression::CompoundLiteral(_) => {}
//         }
//     }
// }

// impl Wrappable for PostfixExpression {
//     fn error() -> Self {
//         PostfixExpression::Primary(PrimaryExpression::Error)
//     }

//     fn into_expression(self) -> Expression {
//         Expression::Postfix(self)
//     }

//     fn from_primary(primary: PrimaryExpression) -> Self {
//         PostfixExpression::Primary(primary)
//     }
// }

// impl Parenthesize for UnaryExpression {
//     fn precedence(&self) -> i32 {
//         20
//     }

//     fn parenthesize_at(&mut self, precedence: i32) {
//         match self {
//             // postfix-expression
//             UnaryExpression::Postfix(e) => e.parenthesize_at(precedence),
//             // ++ unary-expression
//             UnaryExpression::PreIncrement(e) => {
//                 e.parenthesize_at(20);
//                 self.wrapped_at(precedence);
//             }
//             // -- unary-expression
//             UnaryExpression::PreDecrement(e) => {
//                 e.parenthesize_at(20);
//                 self.wrapped_at(precedence);
//             }
//             // unary-operator cast-expression
//             UnaryExpression::Unary { operand, .. } => {
//                 operand.parenthesize_at(25);
//                 self.wrapped_at(precedence);
//             }
//             // sizeof unary-expression
//             UnaryExpression::Sizeof(e) => {
//                 e.parenthesize_at(20);
//                 self.wrapped_at(precedence);
//             }
//             // sizeof ( type-name ) | alignof ( type-name )
//             UnaryExpression::SizeofType(_) | UnaryExpression::Alignof(_) =>
// {}         }
//     }
// }

// impl Wrappable for UnaryExpression {
//     fn error() -> Self {
//         UnaryExpression::Postfix(PostfixExpression::Primary(PrimaryExpression::Error))
//     }

//     fn into_expression(self) -> Expression {
//         Expression::Unary(self)
//     }

//     fn from_primary(primary: PrimaryExpression) -> Self {
//         UnaryExpression::Postfix(PostfixExpression::Primary(primary))
//     }
// }

// impl Parenthesize for CastExpression {
//     fn precedence(&self) -> i32 {
//         25
//     }

//     fn parenthesize_at(&mut self, precedence: i32) {
//         match self {
//             // unary-expression
//             CastExpression::Unary(e) => e.parenthesize_at(precedence),
//             // ( type-name ) cast-expression
//             CastExpression::Cast { expression, .. } => {
//                 expression.parenthesize_at(25);
//                 self.wrapped_at(precedence);
//             }
//         }
//     }
// }

// impl Wrappable for CastExpression {
//     fn error() -> Self {
//         CastExpression::Unary(UnaryExpression::Postfix(PostfixExpression::Primary(
//             PrimaryExpression::Error,
//         )))
//     }

//     fn into_expression(self) -> Expression {
//         Expression::Cast(self)
//     }

//     fn from_primary(primary: PrimaryExpression) -> Self {
//         CastExpression::Unary(UnaryExpression::Postfix(PostfixExpression::Primary(primary)))
//     }
// }

impl<'a, R: Render> Visitor<'a> for Printer<'a, R> {
    type Result = Result<(), R::Error>;

    fn visit_variable_name(&mut self, id: &'a Identifier) -> Self::Result {
        print_identifier(self, id)
    }

    fn visit_type_name_identifier(&mut self, id: &'a Identifier) -> Self::Result {
        print_identifier(self, id)
    }

    fn visit_enum_constant(&mut self, id: &'a Identifier) -> Self::Result {
        print_identifier(self, id)
    }

    fn visit_label_name(&mut self, id: &'a Identifier) -> Self::Result {
        print_identifier(self, id)
    }

    fn visit_member_name(&mut self, id: &'a Identifier) -> Self::Result {
        print_identifier(self, id)
    }

    fn visit_struct_name(&mut self, id: &'a Identifier) -> Self::Result {
        print_identifier(self, id)
    }

    fn visit_enum_name(&mut self, id: &'a Identifier) -> Self::Result {
        print_identifier(self, id)
    }

    fn visit_enumerator_name(&mut self, id: &'a Identifier) -> Self::Result {
        print_identifier(self, id)
    }

    fn visit_translation_unit(&mut self, tu: &'a TranslationUnit) -> Self::Result {
        print_translation_unit(self, tu)
    }

    fn visit_function_definition(&mut self, f: &'a FunctionDefinition) -> Self::Result {
        print_function_definition(self, f)
    }

    fn visit_attribute_specifier(&mut self, a: &'a AttributeSpecifier) -> Self::Result {
        print_attribute_specfier(self, a)
    }

    fn visit_attribute(&mut self, a: &'a Attribute) -> Self::Result {
        print_attribute(self, a)
    }

    fn visit_statement(&mut self, s: &'a Statement) -> Self::Result {
        print_statement(self, s)
    }

    fn visit_unlabeled_statement(&mut self, s: &'a UnlabeledStatement) -> Self::Result {
        print_unlabeled_statement(self, s)
    }

    fn visit_expression(&mut self, e: &'a Expression) -> Self::Result {
        print_expression(self, e)
    }

    fn visit_declaration(&mut self, d: &'a Declaration) -> Self::Result {
        print_declaration(self, d)
    }

    fn visit_declarator(&mut self, d: &'a Declarator) -> Self::Result {
        print_declarator(self, d)
    }

    fn visit_direct_declarator(&mut self, d: &'a DirectDeclarator) -> Self::Result {
        print_direct_declarator(self, d)
    }

    fn visit_postfix_expression(&mut self, p: &'a PostfixExpression) -> Self::Result {
        print_postfix_expression(self, p)
    }

    fn visit_unary_expression(&mut self, u: &'a UnaryExpression) -> Self::Result {
        print_unary_expression(self, u)
    }

    fn visit_cast_expression(&mut self, c: &'a CastExpression) -> Self::Result {
        print_cast_expression(self, c)
    }

    fn visit_compound_statement(&mut self, c: &'a CompoundStatement) -> Self::Result {
        print_compound_statement(self, c)
    }

    fn visit_declaration_specifiers(&mut self, s: &'a DeclarationSpecifiers) -> Self::Result {
        print_declaration_specifiers(self, s)
    }

    fn visit_type_specifier_qualifier(&mut self, x: &'a TypeSpecifierQualifier) -> Self::Result {
        print_type_specifier_qualifier(self, x)
    }

    fn visit_type_specifier(&mut self, ts: &'a TypeSpecifier) -> Self::Result {
        print_type_specifier(self, ts)
    }

    fn visit_atomic_type_specifier(&mut self, a: &'a AtomicTypeSpecifier) -> Self::Result {
        print_atomic_type_specifier(self, a)
    }

    fn visit_typeof(&mut self, t: &'a TypeofSpecifier) -> Self::Result {
        print_typeof(self, t)
    }

    fn visit_specifier_qualifier_list(&mut self, s: &'a SpecifierQualifierList) -> Self::Result {
        print_specifier_qualifier_list(self, s)
    }

    fn visit_type_name(&mut self, tn: &'a TypeName) -> Self::Result {
        print_type_name(self, tn)
    }

    fn visit_abstract_declarator(&mut self, a: &'a AbstractDeclarator) -> Self::Result {
        print_abstract_declarator(self, a)
    }

    fn visit_direct_abstract_declarator(&mut self, d: &'a DirectAbstractDeclarator) -> Self::Result {
        print_direct_abstract_declarator(self, d)
    }

    fn visit_external_declaration(&mut self, d: &'a ExternalDeclaration) -> Self::Result {
        print_external_declaration(self, d)
    }
}

fn print_identifier<'a, R: Render>(pp: &mut Printer<'a, R>, id: &'a Identifier) -> Result<(), R::Error> {
    pp.text(&id.0)
}

fn print_translation_unit<'a, R: Render>(pp: &mut Printer<'a, R>, tu: &'a TranslationUnit) -> Result<(), R::Error> {
    for external in &tu.external_declarations {
        pp.visit_external_declaration(external)?;
        pp.hard_break()?;
    }
    Ok(())
}

fn print_function_definition<'a, R: Render>(
    pp: &mut Printer<'a, R>,
    f: &'a FunctionDefinition,
) -> Result<(), R::Error> {
    pp.igroup(0, |pp| {
        for a in &f.attributes {
            pp.visit_attribute_specifier(a)?;
            pp.space()?;
        }
        pp.visit_declaration_specifiers(&f.specifiers)?;
        pp.space()?;
        pp.visit_declarator(&f.declarator)?;
        pp.space()?;
        pp.visit_compound_statement(&f.body)?;
        Ok(())
    })
}

fn print_attribute_specfier<'a, R: Render>(pp: &mut Printer<'a, R>, a: &'a AttributeSpecifier) -> Result<(), R::Error> {
    match a {
        AttributeSpecifier::Attributes(attributes) => pp.igroup(2, |pp| {
            pp.text("[[")?;
            for (i, attr) in attributes.iter().enumerate() {
                if i > 0 {
                    pp.text(",")?;
                    pp.space()?;
                }
                pp.visit_attribute(attr)?;
            }
            pp.scan_break(0, -2)?;
            pp.text("]]")?;
            Ok(())
        }),
        AttributeSpecifier::Asm(string_literals) => pp.visit_asm_attribute_specifier(string_literals),
        AttributeSpecifier::Error => Ok(()),
    }
}

fn print_attribute<'a, R: Render>(pp: &mut Printer<'a, R>, a: &'a Attribute) -> Result<(), R::Error> {
    pp.igroup(2, |pp| {
        print_attribute_token(pp, &a.token)?;
        if let Some(args) = &a.arguments {
            pp.igroup(2, |pp| {
                pp.text("(")?;
                print_balanced_token_sequence(pp, args)?;
                pp.scan_break(0, -2)?;
                pp.text(")")
            })?;
        }
        Ok(())
    })
}

fn print_attribute_token<'a, R: Render>(pp: &mut Printer<'a, R>, a: &'a AttributeToken) -> Result<(), R::Error> {
    match a {
        AttributeToken::Standard(identifier) => print_identifier(pp, identifier),
        AttributeToken::Prefixed { prefix, identifier } => {
            print_identifier(pp, prefix)?;
            pp.text("::")?;
            print_identifier(pp, identifier)
        }
    }
}

fn print_balanced_token_sequence<'a, R: Render>(
    pp: &mut Printer<'a, R>,
    seq: &'a BalancedTokenSequence,
) -> Result<(), R::Error> {
    for (i, token) in seq.tokens.iter().enumerate() {
        if i > 0 {
            pp.space()?;
        }
        match &token.value {
            BalancedToken::Parenthesized(seq) => pp.igroup(2, |pp| {
                pp.text("(")?;
                print_balanced_token_sequence(pp, seq)?;
                pp.scan_break(0, -2)?;
                pp.text(")")
            })?,
            BalancedToken::Bracketed(seq) => pp.igroup(2, |pp| {
                pp.text("[")?;
                print_balanced_token_sequence(pp, seq)?;
                pp.scan_break(0, -2)?;
                pp.text("]")
            })?,
            BalancedToken::Braced(seq) => pp.igroup(2, |pp| {
                pp.text("{")?;
                print_balanced_token_sequence(pp, seq)?;
                pp.scan_break(0, -2)?;
                pp.text("}")
            })?,
            BalancedToken::Identifier(identifier) => print_identifier(pp, identifier)?,
            BalancedToken::StringLiteral(string_literals) => {
                for (i, lit) in string_literals.0.iter().enumerate() {
                    if i > 0 {
                        pp.space()?;
                    }
                    print_string_literal(pp, lit)?;
                }
            }
            BalancedToken::QuotedString(lit) => print_quoted_string(pp, lit)?,
            BalancedToken::Constant(constant) => print_constant(pp, constant)?,
            BalancedToken::Punctuator(punctuator) => print_punctuator(pp, punctuator)?,
            BalancedToken::Unknown => {}
        }
    }
    Ok(())
}

fn print_string_literal<'a, R: Render>(pp: &mut Printer<'a, R>, lit: &'a StringLiteral) -> Result<(), R::Error> {
    match &lit.encoding_prefix {
        Some(EncodingPrefix::U8) => pp.text("u8")?,
        Some(EncodingPrefix::U) => pp.text("u")?,
        Some(EncodingPrefix::CapitalU) => pp.text("U")?,
        Some(EncodingPrefix::L) => pp.text("L")?,
        None => {}
    }
    pp.text("\"")?;
    for ch in lit.value.chars() {
        match ch {
            '\\' => pp.text("\\\\")?,
            '\'' => pp.text("\\'")?,
            '"' => pp.text("\\\"")?,
            '\n' => pp.text("\\n")?,
            '\r' => pp.text("\\r")?,
            '\t' => pp.text("\\t")?,
            '\0' => pp.text("\\0")?,
            '\x07' => pp.text("\\a")?,
            '\x08' => pp.text("\\b")?,
            '\x0c' => pp.text("\\f")?,
            '\x0b' => pp.text("\\v")?,
            '\x1b' => pp.text("\\e")?,
            _ => pp.text_owned(ch.to_string())?,
        }
    }
    pp.text("\"")
}

fn print_quoted_string<'a, R: Render>(pp: &mut Printer<'a, R>, lit: &'a String) -> Result<(), R::Error> {
    pp.text("`")?;
    pp.text(lit)?;
    pp.text("`")
}

fn print_constant<'a, R: Render>(pp: &mut Printer<'a, R>, c: &'a Constant) -> Result<(), R::Error> {
    match c {
        Constant::Integer(integer_constant) => {
            pp.text_owned(integer_constant.value.to_string())?;
            if let Some(suffix) = integer_constant.suffix {
                let suffix_str = match suffix {
                    crate::ast::IntegerSuffix::Unsigned => "U",
                    crate::ast::IntegerSuffix::Long => "L",
                    crate::ast::IntegerSuffix::LongLong => "LL",
                    crate::ast::IntegerSuffix::UnsignedLong => "UL",
                    crate::ast::IntegerSuffix::UnsignedLongLong => "ULL",
                    crate::ast::IntegerSuffix::BitPrecise => "_BitInt()",
                    crate::ast::IntegerSuffix::UnsignedBitPrecise => "unsigned _BitInt()",
                };
                pp.text(suffix_str)?;
            }
            Ok(())
        }
        Constant::Floating(floating_constant) => {
            pp.text_owned(floating_constant.value.to_string())?;
            if let Some(suffix) = floating_constant.suffix {
                let suffix_str = match suffix {
                    crate::ast::FloatingSuffix::F => "f",
                    crate::ast::FloatingSuffix::L => "l",
                    crate::ast::FloatingSuffix::DF => "df",
                    crate::ast::FloatingSuffix::DD => "dd",
                    crate::ast::FloatingSuffix::DL => "dl",
                };
                pp.text(suffix_str)?;
            }
            Ok(())
        }
        Constant::Character(character_constant) => {
            match &character_constant.encoding_prefix {
                Some(EncodingPrefix::U8) => pp.text("u8")?,
                Some(EncodingPrefix::U) => pp.text("u")?,
                Some(EncodingPrefix::CapitalU) => pp.text("U")?,
                Some(EncodingPrefix::L) => pp.text("L")?,
                None => {}
            }
            pp.text("'")?;
            for ch in character_constant.value.chars() {
                match ch {
                    '\\' => pp.text("\\\\")?,
                    '\'' => pp.text("\\'")?,
                    '"' => pp.text("\\\"")?,
                    '\n' => pp.text("\\n")?,
                    '\r' => pp.text("\\r")?,
                    '\t' => pp.text("\\t")?,
                    '\0' => pp.text("\\0")?,
                    '\x07' => pp.text("\\a")?,
                    '\x08' => pp.text("\\b")?,
                    '\x0c' => pp.text("\\f")?,
                    '\x0b' => pp.text("\\v")?,
                    '\x1b' => pp.text("\\e")?,
                    _ => pp.text_owned(ch.to_string())?,
                }
            }
            pp.text("'")
        }
        Constant::Predefined(predefined_constant) => {
            let const_str = match predefined_constant {
                crate::ast::PredefinedConstant::False => "false",
                crate::ast::PredefinedConstant::True => "true",
                crate::ast::PredefinedConstant::Nullptr => "nullptr",
            };
            pp.text(const_str)
        }
    }
}

fn print_punctuator<'a, R: Render>(pp: &mut Printer<'a, R>, p: &'a Punctuator) -> Result<(), R::Error> {
    let text = match p {
        // Brackets
        Punctuator::LeftBracket => "[",
        Punctuator::RightBracket => "]",
        Punctuator::LeftParen => "(",
        Punctuator::RightParen => ")",
        Punctuator::LeftBrace => "{",
        Punctuator::RightBrace => "}",

        // Operators
        Punctuator::Dot => ".",
        Punctuator::Arrow => "->",
        Punctuator::Increment => "++",
        Punctuator::Decrement => "--",
        Punctuator::Ampersand => "&",
        Punctuator::Star => "*",
        Punctuator::Plus => "+",
        Punctuator::Minus => "-",
        Punctuator::Tilde => "~",
        Punctuator::Bang => "!",
        Punctuator::Slash => "/",
        Punctuator::Percent => "%",
        Punctuator::LeftShift => "<<",
        Punctuator::RightShift => ">>",
        Punctuator::Less => "<",
        Punctuator::Greater => ">",
        Punctuator::LessEqual => "<=",
        Punctuator::GreaterEqual => ">=",
        Punctuator::Equal => "==",
        Punctuator::NotEqual => "!=",
        Punctuator::Caret => "^",
        Punctuator::Pipe => "|",
        Punctuator::LogicalAnd => "&&",
        Punctuator::LogicalOr => "||",
        Punctuator::Question => "?",
        Punctuator::Colon => ":",
        Punctuator::Scope => "::",
        Punctuator::Semicolon => ";",
        Punctuator::Ellipsis => "...",

        // Assignment
        Punctuator::Assign => "=",
        Punctuator::MulAssign => "*=",
        Punctuator::DivAssign => "/=",
        Punctuator::ModAssign => "%=",
        Punctuator::AddAssign => "+=",
        Punctuator::SubAssign => "-=",
        Punctuator::LeftShiftAssign => "<<=",
        Punctuator::RightShiftAssign => ">>=",
        Punctuator::AndAssign => "&=",
        Punctuator::XorAssign => "^=",
        Punctuator::OrAssign => "|=",

        // Other
        Punctuator::Comma => ",",
        Punctuator::Hash => "#",
        Punctuator::HashHash => "##",
    };
    pp.text(text)
}

fn print_external_declaration<'a, R: Render>(
    pp: &mut Printer<'a, R>,
    d: &'a ExternalDeclaration,
) -> Result<(), R::Error> {
    match d {
        ExternalDeclaration::Function(f) => pp.visit_function_definition(f),
        ExternalDeclaration::Declaration(d) => pp.visit_declaration(d),
    }
}

fn print_statement<'a, R: Render>(pp: &mut Printer<'a, R>, s: &'a Statement) -> Result<(), R::Error> {
    match s {
        Statement::Labeled(ls) => {
            // Print label
            match &ls.label {
                Label::Identifier { attributes, identifier } => {
                    for a in attributes {
                        pp.visit_attribute_specifier(a)?;
                        pp.space()?;
                    }
                    pp.visit_label_name(identifier)?;
                    pp.text(":")?;
                }
                Label::Case { attributes, expression } => {
                    for a in attributes {
                        pp.visit_attribute_specifier(a)?;
                        pp.space()?;
                    }
                    pp.text("case")?;
                    pp.space()?;
                    print_constant_expression(pp, expression)?;
                    pp.text(":")?;
                }
                Label::Default { attributes } => {
                    for a in attributes {
                        pp.visit_attribute_specifier(a)?;
                        pp.space()?;
                    }
                    pp.text("default:")?;
                }
            }
            pp.space()?;
            pp.visit_statement(&ls.statement)
        }
        Statement::Unlabeled(u) => pp.visit_unlabeled_statement(u),
    }
}

fn print_unlabeled_statement<'a, R: Render>(
    pp: &mut Printer<'a, R>,
    s: &'a UnlabeledStatement,
) -> Result<(), R::Error> {
    match s {
        UnlabeledStatement::Expression(es) => {
            for a in &es.attributes {
                pp.visit_attribute_specifier(a)?;
                pp.space()?;
            }
            if let Some(expr) = &es.expression {
                pp.visit_expression(expr)?;
            }
            pp.text(";")
        }
        UnlabeledStatement::Primary { attributes, block } => {
            for a in attributes {
                pp.visit_attribute_specifier(a)?;
                pp.space()?;
            }
            match block {
                PrimaryBlock::Compound(c) => pp.visit_compound_statement(c),
                PrimaryBlock::Selection(sel) => print_selection_statement(pp, sel),
                PrimaryBlock::Iteration(iter) => print_iteration_statement(pp, iter),
            }
        }
        UnlabeledStatement::Jump { attributes, statement } => {
            for a in attributes {
                pp.visit_attribute_specifier(a)?;
                pp.space()?;
            }
            match statement {
                JumpStatement::Goto(id) => {
                    pp.text("goto")?;
                    pp.space()?;
                    pp.visit_label_name(id)?;
                    pp.text(";")
                }
                JumpStatement::Continue => pp.text("continue;"),
                JumpStatement::Break => pp.text("break;"),
                JumpStatement::Return(expr) => {
                    pp.text("return")?;
                    if let Some(e) = expr {
                        pp.space()?;
                        pp.visit_expression(e)?;
                    }
                    pp.text(";")
                }
            }
        }
    }
}

fn print_selection_statement<'a, R: Render>(
    pp: &mut Printer<'a, R>,
    sel: &'a SelectionStatement,
) -> Result<(), R::Error> {
    match sel {
        SelectionStatement::If { condition, then_stmt, else_stmt } => {
            pp.text("if")?;
            pp.space()?;
            pp.text("(")?;
            pp.visit_expression(condition)?;
            pp.text(")")?;
            pp.space()?;
            pp.visit_statement(then_stmt)?;
            if let Some(e) = else_stmt {
                pp.space()?;
                pp.text("else")?;
                pp.space()?;
                pp.visit_statement(e)?;
            }
            Ok(())
        }
        SelectionStatement::Switch { expression, statement } => {
            pp.text("switch")?;
            pp.space()?;
            pp.text("(")?;
            pp.visit_expression(expression)?;
            pp.text(")")?;
            pp.space()?;
            pp.visit_statement(statement)
        }
    }
}

fn print_iteration_statement<'a, R: Render>(
    pp: &mut Printer<'a, R>,
    iter: &'a IterationStatement,
) -> Result<(), R::Error> {
    match iter {
        IterationStatement::While { condition, body } => {
            pp.text("while")?;
            pp.space()?;
            pp.text("(")?;
            pp.visit_expression(condition)?;
            pp.text(")")?;
            pp.space()?;
            pp.visit_statement(body)
        }
        IterationStatement::DoWhile { body, condition } => {
            pp.text("do")?;
            pp.space()?;
            pp.visit_statement(body)?;
            pp.space()?;
            pp.text("while")?;
            pp.space()?;
            pp.text("(")?;
            pp.visit_expression(condition)?;
            pp.text(");")
        }
        IterationStatement::For { init, condition, update, body } => {
            pp.text("for")?;
            pp.space()?;
            pp.text("(")?;
            if let Some(i) = init {
                match i {
                    ForInit::Expression(e) => pp.visit_expression(e)?,
                    ForInit::Declaration(d) => pp.visit_declaration(d)?,
                }
            }
            pp.text(";")?;
            if let Some(c) = condition {
                pp.space()?;
                pp.visit_expression(c)?;
            }
            pp.text(";")?;
            if let Some(u) = update {
                pp.space()?;
                pp.visit_expression(u)?;
            }
            pp.text(")")?;
            pp.space()?;
            pp.visit_statement(body)
        }
        IterationStatement::Error => Ok(()),
    }
}

fn print_compound_statement<'a, R: Render>(pp: &mut Printer<'a, R>, c: &'a CompoundStatement) -> Result<(), R::Error> {
    pp.igroup(2, |pp| {
        pp.text("{")?;
        pp.hard_break()?;
        for item in &c.items {
            match item {
                BlockItem::Declaration(d) => {
                    pp.visit_declaration(d)?;
                    pp.hard_break()?;
                }
                BlockItem::Statement(s) => {
                    pp.visit_unlabeled_statement(s)?;
                    pp.hard_break()?;
                }
                BlockItem::Label(l) => {
                    match l {
                        Label::Identifier { attributes, identifier } => {
                            for a in attributes {
                                pp.visit_attribute_specifier(a)?;
                                pp.space()?;
                            }
                            pp.visit_label_name(identifier)?;
                            pp.text(":")?;
                        }
                        Label::Case { attributes, expression } => {
                            for a in attributes {
                                pp.visit_attribute_specifier(a)?;
                                pp.space()?;
                            }
                            pp.text("case")?;
                            pp.space()?;
                            print_constant_expression(pp, expression)?;
                            pp.text(":")?;
                        }
                        Label::Default { attributes } => {
                            for a in attributes {
                                pp.visit_attribute_specifier(a)?;
                                pp.space()?;
                            }
                            pp.text("default:")?;
                        }
                    }
                    pp.hard_break()?;
                }
            }
        }
        pp.scan_break(0, -2)?;
        pp.text("}")
    })
}

fn print_expression<'a, R: Render>(pp: &mut Printer<'a, R>, e: &'a Expression) -> Result<(), R::Error> {
    match e {
        Expression::Postfix(p) => pp.visit_postfix_expression(p),
        Expression::Unary(u) => pp.visit_unary_expression(u),
        Expression::Cast(c) => pp.visit_cast_expression(c),
        Expression::Binary(b) => {
            pp.visit_expression(&b.left)?;
            pp.space()?;
            print_binary_operator(pp, &b.operator)?;
            pp.space()?;
            pp.visit_expression(&b.right)
        }
        Expression::Conditional(cond) => {
            pp.visit_expression(&cond.condition)?;
            pp.space()?;
            pp.text("?")?;
            pp.space()?;
            pp.visit_expression(&cond.then_expr)?;
            pp.space()?;
            pp.text(":")?;
            pp.space()?;
            pp.visit_expression(&cond.else_expr)
        }
        Expression::Assignment(a) => {
            pp.visit_expression(&a.left)?;
            pp.space()?;
            print_assignment_operator(pp, &a.operator)?;
            pp.space()?;
            pp.visit_expression(&a.right)
        }
        Expression::Comma(c) => {
            for (i, expr) in c.expressions.iter().enumerate() {
                if i > 0 {
                    pp.text(",")?;
                    pp.space()?;
                }
                pp.visit_expression(expr)?;
            }
            Ok(())
        }
        Expression::Error => Ok(()),
    }
}

fn print_binary_operator<'a, R: Render>(pp: &mut Printer<'a, R>, op: &BinaryOperator) -> Result<(), R::Error> {
    let text = match op {
        BinaryOperator::Multiply => "*",
        BinaryOperator::Divide => "/",
        BinaryOperator::Modulo => "%",
        BinaryOperator::Add => "+",
        BinaryOperator::Subtract => "-",
        BinaryOperator::LeftShift => "<<",
        BinaryOperator::RightShift => ">>",
        BinaryOperator::BitwiseAnd => "&",
        BinaryOperator::BitwiseXor => "^",
        BinaryOperator::BitwiseOr => "|",
        BinaryOperator::Less => "<",
        BinaryOperator::Greater => ">",
        BinaryOperator::LessEqual => "<=",
        BinaryOperator::GreaterEqual => ">=",
        BinaryOperator::Equal => "==",
        BinaryOperator::NotEqual => "!=",
        BinaryOperator::LogicalAnd => "&&",
        BinaryOperator::LogicalOr => "||",
    };
    pp.text(text)
}

fn print_assignment_operator<'a, R: Render>(pp: &mut Printer<'a, R>, op: &AssignmentOperator) -> Result<(), R::Error> {
    let text = match op {
        AssignmentOperator::Assign => "=",
        AssignmentOperator::MulAssign => "*=",
        AssignmentOperator::DivAssign => "/=",
        AssignmentOperator::ModAssign => "%=",
        AssignmentOperator::AddAssign => "+=",
        AssignmentOperator::SubAssign => "-=",
        AssignmentOperator::LeftShiftAssign => "<<=",
        AssignmentOperator::RightShiftAssign => ">>=",
        AssignmentOperator::AndAssign => "&=",
        AssignmentOperator::XorAssign => "^=",
        AssignmentOperator::OrAssign => "|=",
    };
    pp.text(text)
}

fn print_postfix_expression<'a, R: Render>(pp: &mut Printer<'a, R>, p: &'a PostfixExpression) -> Result<(), R::Error> {
    match p {
        PostfixExpression::Primary(pr) => print_primary_expression(pp, pr),
        PostfixExpression::ArrayAccess { array, index } => {
            pp.visit_postfix_expression(array)?;
            pp.text("[")?;
            pp.visit_expression(index)?;
            pp.text("]")
        }
        PostfixExpression::FunctionCall { function, arguments } => {
            pp.visit_postfix_expression(function)?;
            pp.igroup(2, |pp| {
                pp.text("(")?;
                for (i, arg) in arguments.iter().enumerate() {
                    if i > 0 {
                        pp.text(",")?;
                        pp.space()?;
                    }
                    pp.visit_expression(arg)?;
                }
                pp.scan_break(0, -2)?;
                pp.text(")")
            })
        }
        PostfixExpression::MemberAccess { object, member } => {
            pp.visit_postfix_expression(object)?;
            pp.text(".")?;
            pp.visit_member_name(member)
        }
        PostfixExpression::MemberAccessPtr { object, member } => {
            pp.visit_postfix_expression(object)?;
            pp.text("->")?;
            pp.visit_member_name(member)
        }
        PostfixExpression::PostIncrement(inner) => {
            pp.visit_postfix_expression(inner)?;
            pp.text("++")
        }
        PostfixExpression::PostDecrement(inner) => {
            pp.visit_postfix_expression(inner)?;
            pp.text("--")
        }
        PostfixExpression::CompoundLiteral(cl) => {
            pp.text("(")?;
            for scs in &cl.storage_class_specifiers {
                print_storage_class_specifier(pp, scs)?;
                pp.space()?;
            }
            pp.visit_type_name(&cl.type_name)?;
            pp.text(")")?;
            print_braced_initializer(pp, &cl.initializer)
        }
    }
}

fn print_primary_expression<'a, R: Render>(pp: &mut Printer<'a, R>, pr: &'a PrimaryExpression) -> Result<(), R::Error> {
    match pr {
        PrimaryExpression::Identifier(id) => pp.visit_variable_name(id),
        PrimaryExpression::Constant(c) => print_constant(pp, c),
        PrimaryExpression::EnumerationConstant(id) => pp.visit_enum_constant(id),
        PrimaryExpression::StringLiteral(lits) => {
            for (i, lit) in lits.0.iter().enumerate() {
                if i > 0 {
                    pp.space()?;
                }
                print_string_literal(pp, lit)?;
            }
            Ok(())
        }
        PrimaryExpression::QuotedString(s) => print_quoted_string(pp, s),
        PrimaryExpression::Parenthesized(e) => {
            pp.text("(")?;
            pp.visit_expression(e)?;
            pp.text(")")
        }
        PrimaryExpression::Generic(g) => {
            pp.text("_Generic")?;
            pp.igroup(2, |pp| {
                pp.text("(")?;
                pp.visit_expression(&g.controlling_expression)?;
                pp.text(",")?;
                pp.space()?;
                for (i, assoc) in g.associations.iter().enumerate() {
                    if i > 0 {
                        pp.text(",")?;
                        pp.space()?;
                    }
                    match assoc {
                        GenericAssociation::Type { type_name, expression } => {
                            pp.visit_type_name(type_name)?;
                            pp.text(":")?;
                            pp.space()?;
                            pp.visit_expression(expression)?;
                        }
                        GenericAssociation::Default { expression } => {
                            pp.text("default:")?;
                            pp.space()?;
                            pp.visit_expression(expression)?;
                        }
                    }
                }
                pp.scan_break(0, -2)?;
                pp.text(")")
            })
        }
        PrimaryExpression::Error => Ok(()),
    }
}

fn print_unary_expression<'a, R: Render>(pp: &mut Printer<'a, R>, u: &'a UnaryExpression) -> Result<(), R::Error> {
    match u {
        UnaryExpression::Postfix(p) => pp.visit_postfix_expression(p),
        UnaryExpression::PreIncrement(inner) => {
            pp.text("++")?;
            pp.visit_unary_expression(inner)
        }
        UnaryExpression::PreDecrement(inner) => {
            pp.text("--")?;
            pp.visit_unary_expression(inner)
        }
        UnaryExpression::Unary { operator, operand } => {
            print_unary_operator(pp, operator)?;
            pp.visit_cast_expression(operand)
        }
        UnaryExpression::Sizeof(inner) => {
            pp.text("sizeof")?;
            pp.space()?;
            pp.visit_unary_expression(inner)
        }
        UnaryExpression::SizeofType(tn) => {
            pp.text("sizeof")?;
            pp.text("(")?;
            pp.visit_type_name(tn)?;
            pp.text(")")
        }
        UnaryExpression::Alignof(tn) => {
            pp.text("_Alignof")?;
            pp.text("(")?;
            pp.visit_type_name(tn)?;
            pp.text(")")
        }
    }
}

fn print_unary_operator<'a, R: Render>(pp: &mut Printer<'a, R>, op: &UnaryOperator) -> Result<(), R::Error> {
    let text = match op {
        UnaryOperator::Address => "&",
        UnaryOperator::Dereference => "*",
        UnaryOperator::Plus => "+",
        UnaryOperator::Minus => "-",
        UnaryOperator::BitwiseNot => "~",
        UnaryOperator::LogicalNot => "!",
    };
    pp.text(text)
}

fn print_cast_expression<'a, R: Render>(pp: &mut Printer<'a, R>, c: &'a CastExpression) -> Result<(), R::Error> {
    match c {
        CastExpression::Unary(u) => pp.visit_unary_expression(u),
        CastExpression::Cast { type_name, expression } => {
            pp.text("(")?;
            pp.visit_type_name(type_name)?;
            pp.text(")")?;
            pp.visit_cast_expression(expression)
        }
    }
}

fn print_declaration<'a, R: Render>(pp: &mut Printer<'a, R>, d: &'a Declaration) -> Result<(), R::Error> {
    match d {
        Declaration::Normal { attributes, specifiers, declarators } => {
            for a in attributes {
                pp.visit_attribute_specifier(a)?;
                pp.space()?;
            }
            pp.visit_declaration_specifiers(specifiers)?;
            if !declarators.is_empty() {
                pp.space()?;
                for (i, init_decl) in declarators.iter().enumerate() {
                    if i > 0 {
                        pp.text(",")?;
                        pp.space()?;
                    }
                    pp.visit_declarator(&init_decl.declarator)?;
                    if let Some(initializer) = &init_decl.initializer {
                        pp.space()?;
                        pp.text("=")?;
                        pp.space()?;
                        print_initializer(pp, initializer)?;
                    }
                }
            }
            pp.text(";")
        }
        Declaration::Typedef { attributes, specifiers, declarators } => {
            for a in attributes {
                pp.visit_attribute_specifier(a)?;
                pp.space()?;
            }
            pp.text("typedef")?;
            pp.space()?;
            pp.visit_declaration_specifiers(specifiers)?;
            if !declarators.is_empty() {
                pp.space()?;
                for (i, decl) in declarators.iter().enumerate() {
                    if i > 0 {
                        pp.text(",")?;
                        pp.space()?;
                    }
                    pp.visit_declarator(decl)?;
                }
            }
            pp.text(";")
        }
        Declaration::StaticAssert(sa) => {
            pp.text("_Static_assert")?;
            pp.text("(")?;
            print_constant_expression(pp, &sa.condition)?;
            if let Some(msg) = &sa.message {
                pp.text(",")?;
                pp.space()?;
                for (i, lit) in msg.0.iter().enumerate() {
                    if i > 0 {
                        pp.space()?;
                    }
                    print_string_literal(pp, lit)?;
                }
            }
            pp.text(");")?;
            Ok(())
        }
        Declaration::Attribute(attrs) => {
            for (i, a) in attrs.iter().enumerate() {
                if i > 0 {
                    pp.space()?;
                }
                pp.visit_attribute_specifier(a)?;
            }
            pp.text(";")
        }
        Declaration::Error => Ok(()),
    }
}

fn print_initializer<'a, R: Render>(pp: &mut Printer<'a, R>, init: &'a Initializer) -> Result<(), R::Error> {
    match init {
        Initializer::Expression(e) => pp.visit_expression(e),
        Initializer::Braced(b) => print_braced_initializer(pp, b),
    }
}

fn print_braced_initializer<'a, R: Render>(pp: &mut Printer<'a, R>, b: &'a BracedInitializer) -> Result<(), R::Error> {
    pp.igroup(2, |pp| {
        pp.text("{")?;
        for (i, init) in b.initializers.iter().enumerate() {
            if i > 0 {
                pp.text(",")?;
            }
            pp.space()?;
            if let Some(designation) = &init.designation {
                print_designation(pp, designation)?;
                pp.space()?;
                pp.text("=")?;
                pp.space()?;
            }
            print_initializer(pp, &init.initializer)?;
        }
        pp.scan_break(0, -2)?;
        pp.text("}")
    })
}

fn print_designation<'a, R: Render>(pp: &mut Printer<'a, R>, d: &'a Designation) -> Result<(), R::Error> {
    if let Some(next) = &d.designation {
        print_designation(pp, next)?;
    }
    match &d.designator {
        Designator::Array(e) => {
            pp.text("[")?;
            print_constant_expression(pp, e)?;
            pp.text("]")
        }
        Designator::Member(id) => {
            pp.text(".")?;
            pp.visit_member_name(id)
        }
    }
}

fn print_constant_expression<'a, R: Render>(
    pp: &mut Printer<'a, R>,
    e: &'a ConstantExpression,
) -> Result<(), R::Error> {
    match e {
        ConstantExpression::Expression(expr) => pp.visit_expression(expr),
        ConstantExpression::Error => Ok(()),
    }
}

fn print_declaration_specifiers<'a, R: Render>(
    pp: &mut Printer<'a, R>,
    s: &'a DeclarationSpecifiers,
) -> Result<(), R::Error> {
    for (i, spec) in s.specifiers.iter().enumerate() {
        if i > 0 {
            pp.space()?;
        }
        match spec {
            DeclarationSpecifier::StorageClass(scs) => print_storage_class_specifier(pp, scs)?,
            DeclarationSpecifier::TypeSpecifierQualifier(tsq) => pp.visit_type_specifier_qualifier(tsq)?,
            DeclarationSpecifier::Function { specifier, attributes } => {
                for a in attributes {
                    pp.visit_attribute_specifier(a)?;
                    pp.space()?;
                }
                print_function_specifier(pp, specifier)?;
            }
        }
    }
    Ok(())
}

fn print_storage_class_specifier<'a, R: Render>(
    pp: &mut Printer<'a, R>,
    scs: &StorageClassSpecifier,
) -> Result<(), R::Error> {
    let text = match scs {
        StorageClassSpecifier::Auto => "auto",
        StorageClassSpecifier::Constexpr => "constexpr",
        StorageClassSpecifier::Extern => "extern",
        StorageClassSpecifier::Register => "register",
        StorageClassSpecifier::Static => "static",
        StorageClassSpecifier::ThreadLocal => "_Thread_local",
        StorageClassSpecifier::Typedef => "typedef",
    };
    pp.text(text)
}

fn print_function_specifier<'a, R: Render>(pp: &mut Printer<'a, R>, fs: &FunctionSpecifier) -> Result<(), R::Error> {
    let text = match fs {
        FunctionSpecifier::Inline => "inline",
        FunctionSpecifier::Noreturn => "_Noreturn",
    };
    pp.text(text)
}

fn print_type_specifier_qualifier<'a, R: Render>(
    pp: &mut Printer<'a, R>,
    x: &'a TypeSpecifierQualifier,
) -> Result<(), R::Error> {
    match x {
        TypeSpecifierQualifier::TypeSpecifier(ts) => pp.visit_type_specifier(ts),
        TypeSpecifierQualifier::TypeQualifier(tq) => print_type_qualifier(pp, tq),
        TypeSpecifierQualifier::AlignmentSpecifier(a) => print_alignment_specifier(pp, a),
    }
}

fn print_type_qualifier<'a, R: Render>(pp: &mut Printer<'a, R>, tq: &TypeQualifier) -> Result<(), R::Error> {
    let text = match tq {
        TypeQualifier::Const => "const",
        TypeQualifier::Restrict => "restrict",
        TypeQualifier::Volatile => "volatile",
        TypeQualifier::Atomic => "_Atomic",
        TypeQualifier::Nonnull => "_Nonnull",
        TypeQualifier::Nullable => "_Nullable",
        TypeQualifier::ThreadLocal => "_Thread_local",
    };
    pp.text(text)
}

fn print_alignment_specifier<'a, R: Render>(
    pp: &mut Printer<'a, R>,
    a: &'a AlignmentSpecifier,
) -> Result<(), R::Error> {
    pp.text("_Alignas")?;
    pp.text("(")?;
    match a {
        AlignmentSpecifier::Type(tn) => pp.visit_type_name(tn)?,
        AlignmentSpecifier::Expression(e) => print_constant_expression(pp, e)?,
    }
    pp.text(")")
}

fn print_type_specifier<'a, R: Render>(pp: &mut Printer<'a, R>, ts: &'a TypeSpecifier) -> Result<(), R::Error> {
    match ts {
        TypeSpecifier::Void => pp.text("void"),
        TypeSpecifier::Char => pp.text("char"),
        TypeSpecifier::Short => pp.text("short"),
        TypeSpecifier::Int => pp.text("int"),
        TypeSpecifier::Long => pp.text("long"),
        TypeSpecifier::Float => pp.text("float"),
        TypeSpecifier::Double => pp.text("double"),
        TypeSpecifier::Signed => pp.text("signed"),
        TypeSpecifier::Unsigned => pp.text("unsigned"),
        TypeSpecifier::BitInt(e) => {
            pp.text("_BitInt")?;
            pp.text("(")?;
            print_constant_expression(pp, e)?;
            pp.text(")")
        }
        TypeSpecifier::Bool => pp.text("_Bool"),
        TypeSpecifier::Complex => pp.text("_Complex"),
        TypeSpecifier::Decimal32 => pp.text("_Decimal32"),
        TypeSpecifier::Decimal64 => pp.text("_Decimal64"),
        TypeSpecifier::Decimal128 => pp.text("_Decimal128"),
        TypeSpecifier::Atomic(a) => pp.visit_atomic_type_specifier(a),
        TypeSpecifier::Struct(s) => print_struct_or_union_specifier(pp, s),
        TypeSpecifier::Enum(e) => print_enum_specifier(pp, e),
        TypeSpecifier::TypedefName(id) => pp.visit_type_name_identifier(id),
        TypeSpecifier::Typeof(t) => pp.visit_typeof(t),
    }
}

fn print_struct_or_union_specifier<'a, R: Render>(
    pp: &mut Printer<'a, R>,
    s: &'a StructOrUnionSpecifier,
) -> Result<(), R::Error> {
    let keyword = match s.kind {
        StructOrUnion::Struct => "struct",
        StructOrUnion::Union => "union",
    };
    pp.text(keyword)?;

    for a in &s.attributes {
        pp.space()?;
        pp.visit_attribute_specifier(a)?;
    }

    if let Some(id) = &s.identifier {
        pp.space()?;
        pp.visit_struct_name(id)?;
    }

    if let Some(members) = &s.members {
        pp.space()?;
        pp.igroup(2, |pp| {
            pp.text("{")?;
            pp.hard_break()?;
            for member in members {
                print_member_declaration(pp, member)?;
                pp.hard_break()?;
            }
            pp.scan_break(0, -2)?;
            pp.text("}")
        })?;
    }

    Ok(())
}

fn print_member_declaration<'a, R: Render>(pp: &mut Printer<'a, R>, m: &'a MemberDeclaration) -> Result<(), R::Error> {
    match m {
        MemberDeclaration::Normal { attributes, specifiers, declarators } => {
            for a in attributes {
                pp.visit_attribute_specifier(a)?;
                pp.space()?;
            }
            pp.visit_specifier_qualifier_list(specifiers)?;
            if !declarators.is_empty() {
                pp.space()?;
                for (i, decl) in declarators.iter().enumerate() {
                    if i > 0 {
                        pp.text(",")?;
                        pp.space()?;
                    }
                    match decl {
                        MemberDeclarator::Declarator(d) => pp.visit_declarator(d)?,
                        MemberDeclarator::BitField { declarator, width } => {
                            if let Some(d) = declarator {
                                pp.visit_declarator(d)?;
                            }
                            pp.text(":")?;
                            pp.space()?;
                            print_constant_expression(pp, width)?;
                        }
                    }
                }
            }
            pp.text(";")
        }
        MemberDeclaration::StaticAssert(sa) => {
            pp.text("_Static_assert")?;
            pp.text("(")?;
            print_constant_expression(pp, &sa.condition)?;
            if let Some(msg) = &sa.message {
                pp.text(",")?;
                pp.space()?;
                for (i, lit) in msg.0.iter().enumerate() {
                    if i > 0 {
                        pp.space()?;
                    }
                    print_string_literal(pp, lit)?;
                }
            }
            pp.text(");")
        }
        MemberDeclaration::Error => Ok(()),
    }
}

fn print_enum_specifier<'a, R: Render>(pp: &mut Printer<'a, R>, e: &'a EnumSpecifier) -> Result<(), R::Error> {
    pp.text("enum")?;

    for a in &e.attributes {
        pp.space()?;
        pp.visit_attribute_specifier(a)?;
    }

    if let Some(id) = &e.identifier {
        pp.space()?;
        pp.visit_enum_name(id)?;
    }

    if let Some(type_spec) = &e.type_specifier {
        pp.space()?;
        pp.text(":")?;
        pp.space()?;
        pp.visit_specifier_qualifier_list(type_spec)?;
    }

    if let Some(enumerators) = &e.enumerators {
        pp.space()?;
        pp.igroup(2, |pp| {
            pp.text("{")?;
            pp.space()?;
            for (i, enumerator) in enumerators.iter().enumerate() {
                if i > 0 {
                    pp.text(",")?;
                    pp.space()?;
                }
                for a in &enumerator.attributes {
                    pp.visit_attribute_specifier(a)?;
                    pp.space()?;
                }
                pp.visit_enumerator_name(&enumerator.name)?;
                if let Some(value) = &enumerator.value {
                    pp.space()?;
                    pp.text("=")?;
                    pp.space()?;
                    print_constant_expression(pp, value)?;
                }
            }
            pp.scan_break(0, -2)?;
            pp.text("}")
        })?;
    }

    Ok(())
}

fn print_atomic_type_specifier<'a, R: Render>(
    pp: &mut Printer<'a, R>,
    a: &'a AtomicTypeSpecifier,
) -> Result<(), R::Error> {
    pp.text("_Atomic")?;
    pp.text("(")?;
    pp.visit_type_name(&a.type_name)?;
    pp.text(")")
}

fn print_typeof<'a, R: Render>(pp: &mut Printer<'a, R>, t: &'a TypeofSpecifier) -> Result<(), R::Error> {
    match t {
        TypeofSpecifier::Typeof(arg) => {
            pp.text("typeof")?;
            pp.text("(")?;
            match arg {
                TypeofSpecifierArgument::Expression(e) => pp.visit_expression(e)?,
                TypeofSpecifierArgument::TypeName(tn) => pp.visit_type_name(tn)?,
                TypeofSpecifierArgument::Error => {}
            }
            pp.text(")")
        }
        TypeofSpecifier::TypeofUnqual(arg) => {
            pp.text("typeof_unqual")?;
            pp.text("(")?;
            match arg {
                TypeofSpecifierArgument::Expression(e) => pp.visit_expression(e)?,
                TypeofSpecifierArgument::TypeName(tn) => pp.visit_type_name(tn)?,
                TypeofSpecifierArgument::Error => {}
            }
            pp.text(")")
        }
    }
}

fn print_specifier_qualifier_list<'a, R: Render>(
    pp: &mut Printer<'a, R>,
    s: &'a SpecifierQualifierList,
) -> Result<(), R::Error> {
    for (i, item) in s.items.iter().enumerate() {
        if i > 0 {
            pp.space()?;
        }
        pp.visit_type_specifier_qualifier(item)?;
    }
    for a in &s.attributes {
        pp.space()?;
        pp.visit_attribute_specifier(a)?;
    }
    Ok(())
}

fn print_type_name<'a, R: Render>(pp: &mut Printer<'a, R>, tn: &'a TypeName) -> Result<(), R::Error> {
    match tn {
        TypeName::TypeName { specifiers, abstract_declarator } => {
            pp.visit_specifier_qualifier_list(specifiers)?;
            if let Some(ad) = abstract_declarator {
                pp.space()?;
                pp.visit_abstract_declarator(ad)?;
            }
            Ok(())
        }
        TypeName::Error => Ok(()),
    }
}

fn print_declarator<'a, R: Render>(pp: &mut Printer<'a, R>, d: &'a Declarator) -> Result<(), R::Error> {
    match d {
        Declarator::Direct(dd) => pp.visit_direct_declarator(dd),
        Declarator::Pointer { pointer, declarator } => {
            print_pointer(pp, pointer)?;
            pp.visit_declarator(declarator)
        }
        Declarator::Error => Ok(()),
    }
}

fn print_pointer<'a, R: Render>(pp: &mut Printer<'a, R>, p: &'a Pointer) -> Result<(), R::Error> {
    match p.pointer_or_block {
        PointerOrBlock::Pointer => pp.text("*")?,
        PointerOrBlock::Block => pp.text("^")?,
    }

    for a in &p.attributes {
        pp.space()?;
        pp.visit_attribute_specifier(a)?;
    }

    for tq in &p.type_qualifiers {
        pp.space()?;
        print_type_qualifier(pp, tq)?;
    }

    Ok(())
}

fn print_direct_declarator<'a, R: Render>(pp: &mut Printer<'a, R>, d: &'a DirectDeclarator) -> Result<(), R::Error> {
    match d {
        DirectDeclarator::Identifier { identifier, attributes } => {
            pp.visit_variable_name(identifier)?;
            for a in attributes {
                pp.space()?;
                pp.visit_attribute_specifier(a)?;
            }
            Ok(())
        }
        DirectDeclarator::Parenthesized(inner) => {
            pp.text("(")?;
            pp.visit_declarator(inner)?;
            pp.text(")")
        }
        DirectDeclarator::Array { declarator, attributes, array_declarator } => {
            pp.visit_direct_declarator(declarator)?;
            for a in attributes {
                pp.space()?;
                pp.visit_attribute_specifier(a)?;
            }
            pp.text("[")?;
            print_array_declarator(pp, array_declarator)?;
            pp.text("]")
        }
        DirectDeclarator::Function { declarator, attributes, parameters } => {
            pp.visit_direct_declarator(declarator)?;
            for a in attributes {
                pp.space()?;
                pp.visit_attribute_specifier(a)?;
            }
            pp.igroup(2, |pp| {
                pp.text("(")?;
                print_parameter_type_list(pp, parameters)?;
                pp.scan_break(0, -2)?;
                pp.text(")")
            })
        }
    }
}

fn print_array_declarator<'a, R: Render>(pp: &mut Printer<'a, R>, a: &'a ArrayDeclarator) -> Result<(), R::Error> {
    match a {
        ArrayDeclarator::Normal { type_qualifiers, size } => {
            for (i, tq) in type_qualifiers.iter().enumerate() {
                if i > 0 {
                    pp.space()?;
                }
                print_type_qualifier(pp, tq)?;
            }
            if let Some(s) = size {
                if !type_qualifiers.is_empty() {
                    pp.space()?;
                }
                pp.visit_expression(s)?;
            }
            Ok(())
        }
        ArrayDeclarator::Static { type_qualifiers, size } => {
            pp.text("static")?;
            for tq in type_qualifiers {
                pp.space()?;
                print_type_qualifier(pp, tq)?;
            }
            pp.space()?;
            pp.visit_expression(size)
        }
        ArrayDeclarator::VLA { type_qualifiers } => {
            for (i, tq) in type_qualifiers.iter().enumerate() {
                if i > 0 {
                    pp.space()?;
                }
                print_type_qualifier(pp, tq)?;
            }
            if !type_qualifiers.is_empty() {
                pp.space()?;
            }
            pp.text("*")
        }
        ArrayDeclarator::Error => Ok(()),
    }
}

fn print_parameter_type_list<'a, R: Render>(pp: &mut Printer<'a, R>, p: &'a ParameterTypeList) -> Result<(), R::Error> {
    match p {
        ParameterTypeList::Parameters(params) => {
            for (i, param) in params.iter().enumerate() {
                if i > 0 {
                    pp.text(",")?;
                    pp.space()?;
                }
                print_parameter_declaration(pp, param)?;
            }
            Ok(())
        }
        ParameterTypeList::Variadic(params) => {
            for (i, param) in params.iter().enumerate() {
                if i > 0 {
                    pp.text(",")?;
                    pp.space()?;
                }
                print_parameter_declaration(pp, param)?;
            }
            pp.text(",")?;
            pp.space()?;
            pp.text("...")
        }
        ParameterTypeList::OnlyVariadic => pp.text("..."),
    }
}

fn print_parameter_declaration<'a, R: Render>(
    pp: &mut Printer<'a, R>,
    p: &'a ParameterDeclaration,
) -> Result<(), R::Error> {
    for a in &p.attributes {
        pp.visit_attribute_specifier(a)?;
        pp.space()?;
    }
    pp.visit_declaration_specifiers(&p.specifiers)?;
    if let Some(kind) = &p.declarator {
        pp.space()?;
        match kind {
            ParameterDeclarationKind::Declarator(d) => pp.visit_declarator(d)?,
            ParameterDeclarationKind::Abstract(a) => pp.visit_abstract_declarator(a)?,
        }
    }
    Ok(())
}

fn print_abstract_declarator<'a, R: Render>(
    pp: &mut Printer<'a, R>,
    a: &'a AbstractDeclarator,
) -> Result<(), R::Error> {
    match a {
        AbstractDeclarator::Direct(d) => pp.visit_direct_abstract_declarator(d),
        AbstractDeclarator::Pointer { pointer, abstract_declarator } => {
            print_pointer(pp, pointer)?;
            if let Some(ad) = abstract_declarator {
                pp.visit_abstract_declarator(ad)?;
            }
            Ok(())
        }
        AbstractDeclarator::Error => Ok(()),
    }
}

fn print_direct_abstract_declarator<'a, R: Render>(
    pp: &mut Printer<'a, R>,
    d: &'a DirectAbstractDeclarator,
) -> Result<(), R::Error> {
    match d {
        DirectAbstractDeclarator::Parenthesized(ad) => {
            pp.text("(")?;
            pp.visit_abstract_declarator(ad)?;
            pp.text(")")
        }
        DirectAbstractDeclarator::Array { declarator, attributes, array_declarator } => {
            if let Some(dd) = declarator {
                pp.visit_direct_abstract_declarator(dd)?;
            }
            for a in attributes {
                pp.space()?;
                pp.visit_attribute_specifier(a)?;
            }
            pp.text("[")?;
            print_array_declarator(pp, array_declarator)?;
            pp.text("]")
        }
        DirectAbstractDeclarator::Function { declarator, attributes, parameters } => {
            if let Some(dd) = declarator {
                pp.visit_direct_abstract_declarator(dd)?;
            }
            for a in attributes {
                pp.space()?;
                pp.visit_attribute_specifier(a)?;
            }
            pp.igroup(2, |pp| {
                pp.text("(")?;
                print_parameter_type_list(pp, parameters)?;
                pp.scan_break(0, -2)?;
                pp.text(")")
            })
        }
    }
}
