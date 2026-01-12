//! Pretty printer for the AST.

use elegance::Render;

use crate::{ast::*, visitor::Visitor};

/// Precedence levels for C expressions (lower number = lower precedence = binds less tightly)
mod precedence {
    pub const COMMA: usize = 1;
    pub const ASSIGNMENT: usize = 2;
    pub const CONDITIONAL: usize = 3;
    pub const LOGICAL_OR: usize = 4;
    pub const LOGICAL_AND: usize = 5;
    pub const BITWISE_OR: usize = 6;
    pub const BITWISE_XOR: usize = 7;
    pub const BITWISE_AND: usize = 8;
    pub const EQUALITY: usize = 9;
    pub const RELATIONAL: usize = 10;
    pub const SHIFT: usize = 11;
    pub const ADDITIVE: usize = 12;
    pub const MULTIPLICATIVE: usize = 13;
    pub const CAST: usize = 14;
    pub const UNARY: usize = 15;
    pub const POSTFIX: usize = 16;
}

/// Context for expression printing, used to determine when parentheses are needed.
///
/// This context tracks the precedence and associativity of the surrounding expression
/// to enable minimal parenthesization when printing expressions.
#[derive(Debug, Clone, Copy)]
pub struct Context {
    /// the precedence of the surrounding context (0 = no context/top level)
    precedence: usize,
    /// whether we're in a position that requires parens at equal precedence
    /// (e.g., right side of left-associative operator)
    assoc: bool,
}

impl Default for Context {
    fn default() -> Self {
        Context { precedence: 0, assoc: false }
    }
}

impl Context {
    /// Check if an expression with the given precedence needs parentheses in this context
    fn needs_parens(&self, expr_prec: usize) -> bool {
        if expr_prec < self.precedence {
            true
        } else if expr_prec == self.precedence && self.assoc {
            true
        } else {
            false
        }
    }
}

/// Get the precedence of a binary operator
fn binary_op_precedence(op: &BinaryOperator) -> usize {
    match op {
        BinaryOperator::Multiply | BinaryOperator::Divide | BinaryOperator::Modulo => precedence::MULTIPLICATIVE,
        BinaryOperator::Add | BinaryOperator::Subtract => precedence::ADDITIVE,
        BinaryOperator::LeftShift | BinaryOperator::RightShift => precedence::SHIFT,
        BinaryOperator::Less | BinaryOperator::Greater | BinaryOperator::LessEqual | BinaryOperator::GreaterEqual => {
            precedence::RELATIONAL
        }
        BinaryOperator::Equal | BinaryOperator::NotEqual => precedence::EQUALITY,
        BinaryOperator::BitwiseAnd => precedence::BITWISE_AND,
        BinaryOperator::BitwiseXor => precedence::BITWISE_XOR,
        BinaryOperator::BitwiseOr => precedence::BITWISE_OR,
        BinaryOperator::LogicalAnd => precedence::LOGICAL_AND,
        BinaryOperator::LogicalOr => precedence::LOGICAL_OR,
    }
}

/// Get the precedence of an expression
fn expr_precedence(e: &Expression) -> usize {
    match e {
        Expression::Postfix(_) => precedence::POSTFIX,
        Expression::Unary(_) => precedence::UNARY,
        Expression::Cast(_) => precedence::CAST,
        Expression::Binary(b) => binary_op_precedence(&b.operator),
        Expression::Conditional(_) => precedence::CONDITIONAL,
        Expression::Assignment(_) => precedence::ASSIGNMENT,
        Expression::Comma(_) => precedence::COMMA,
        Expression::Error => precedence::POSTFIX,
    }
}

/// A pretty printer for C AST nodes.
///
/// This type alias configures the elegance printer with the [`Context`] type
/// for tracking expression precedence during printing.
pub type Printer<'a, R> = elegance::Printer<'a, R, String, Context>;

impl<'a, R: Render> Visitor<'a> for Printer<'a, R> {
    type Result = Result<(), R::Error>;

    fn visit_variable_name(&mut self, id: &'a Identifier) -> Self::Result {
        self.text(&id.0)
    }

    fn visit_type_name_identifier(&mut self, id: &'a Identifier) -> Self::Result {
        self.text(&id.0)
    }

    fn visit_enum_constant(&mut self, id: &'a Identifier) -> Self::Result {
        self.text(&id.0)
    }

    fn visit_label_name(&mut self, id: &'a Identifier) -> Self::Result {
        self.text(&id.0)
    }

    fn visit_member_name(&mut self, id: &'a Identifier) -> Self::Result {
        self.text(&id.0)
    }

    fn visit_struct_name(&mut self, id: &'a Identifier) -> Self::Result {
        self.text(&id.0)
    }

    fn visit_enum_name(&mut self, id: &'a Identifier) -> Self::Result {
        self.text(&id.0)
    }

    fn visit_enumerator_name(&mut self, id: &'a Identifier) -> Self::Result {
        self.text(&id.0)
    }

    fn visit_translation_unit(&mut self, tu: &'a TranslationUnit) -> Self::Result {
        for external in &tu.external_declarations {
            self.visit_external_declaration(external)?;
            self.hard_break()?;
        }
        Ok(())
    }

    fn visit_function_definition(&mut self, f: &'a FunctionDefinition) -> Self::Result {
        self.igroup(0, |pp| {
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

    fn visit_attribute_specifier(&mut self, a: &'a AttributeSpecifier) -> Self::Result {
        match a {
            AttributeSpecifier::Attributes(attributes) => self.igroup(2, |pp| {
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
            AttributeSpecifier::Asm(string_literals) => self.visit_asm_attribute_specifier(string_literals),
            AttributeSpecifier::Error => Ok(()),
        }
    }

    fn visit_attribute(&mut self, a: &'a Attribute) -> Self::Result {
        self.igroup(2, |pp| {
            match &a.token {
                AttributeToken::Standard(identifier) => pp.text(&identifier.0)?,
                AttributeToken::Prefixed { prefix, identifier } => {
                    pp.text(&prefix.0)?;
                    pp.text("::")?;
                    pp.text(&identifier.0)?;
                }
            }
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

    fn visit_label(&mut self, l: &'a Label) -> Self::Result {
        match l {
            Label::Identifier { attributes, identifier } => {
                for a in attributes {
                    self.visit_attribute_specifier(a)?;
                    self.space()?;
                }
                self.visit_label_name(identifier)?;
                self.text(":")?;
            }
            Label::Case { attributes, expression } => {
                for a in attributes {
                    self.visit_attribute_specifier(a)?;
                    self.space()?;
                }
                self.text("case")?;
                self.space()?;
                self.visit_constant_expression(expression)?;
                self.text(":")?;
            }
            Label::Default { attributes } => {
                for a in attributes {
                    self.visit_attribute_specifier(a)?;
                    self.space()?;
                }
                self.text("default:")?;
            }
        }
        self.space()
    }

    fn visit_unlabeled_statement(&mut self, s: &'a UnlabeledStatement) -> Self::Result {
        match s {
            UnlabeledStatement::Expression(es) => self.visit_expression_statement(es),
            UnlabeledStatement::Primary { attributes, block } => {
                for a in attributes {
                    self.visit_attribute_specifier(a)?;
                    self.space()?;
                }
                self.visit_primary_block(block)
            }
            UnlabeledStatement::Jump { attributes, statement } => {
                for a in attributes {
                    self.visit_attribute_specifier(a)?;
                    self.space()?;
                }
                self.visit_jump_statement(statement)
            }
        }
    }

    fn visit_expression_statement(&mut self, e: &'a ExpressionStatement) -> Self::Result {
        for a in &e.attributes {
            self.visit_attribute_specifier(a)?;
            self.space()?;
        }
        if let Some(expr) = &e.expression {
            self.visit_expression(expr)?;
        }
        self.text(";")
    }

    fn visit_jump_statement(&mut self, j: &'a JumpStatement) -> Self::Result {
        match j {
            JumpStatement::Goto(id) => {
                self.text("goto")?;
                self.space()?;
                self.visit_label_name(id)?;
                self.text(";")
            }
            JumpStatement::Continue => self.text("continue;"),
            JumpStatement::Break => self.text("break;"),
            JumpStatement::Return(expr) => {
                self.text("return")?;
                if let Some(e) = expr {
                    self.space()?;
                    self.visit_expression(e)?;
                }
                self.text(";")
            }
        }
    }

    fn visit_selection_statement(&mut self, sel: &'a SelectionStatement) -> Self::Result {
        match sel {
            SelectionStatement::If { condition, then_stmt, else_stmt } => {
                self.text("if")?;
                self.space()?;
                self.text("(")?;
                self.visit_expression(condition)?;
                self.text(")")?;
                self.space()?;
                self.visit_statement(then_stmt)?;
                if let Some(e) = else_stmt {
                    self.space()?;
                    self.text("else")?;
                    self.space()?;
                    self.visit_statement(e)?;
                }
                Ok(())
            }
            SelectionStatement::Switch { expression, statement } => {
                self.text("switch")?;
                self.space()?;
                self.text("(")?;
                self.visit_expression(expression)?;
                self.text(")")?;
                self.space()?;
                self.visit_statement(statement)
            }
        }
    }

    fn visit_iteration_statement(&mut self, iter: &'a IterationStatement) -> Self::Result {
        match iter {
            IterationStatement::While { condition, body } => {
                self.text("while")?;
                self.space()?;
                self.text("(")?;
                self.visit_expression(condition)?;
                self.text(")")?;
                self.space()?;
                self.visit_statement(body)
            }
            IterationStatement::DoWhile { body, condition } => {
                self.text("do")?;
                self.space()?;
                self.visit_statement(body)?;
                self.space()?;
                self.text("while")?;
                self.space()?;
                self.text("(")?;
                self.visit_expression(condition)?;
                self.text(");")
            }
            IterationStatement::For { init, condition, update, body } => {
                self.text("for")?;
                self.space()?;
                self.text("(")?;
                if let Some(i) = init {
                    self.visit_for_init(i)?;
                } else {
                    self.text(";")?;
                }
                if let Some(c) = condition {
                    self.space()?;
                    self.visit_expression(c)?;
                }
                self.text(";")?;
                if let Some(u) = update {
                    self.space()?;
                    self.visit_expression(u)?;
                }
                self.text(")")?;
                self.space()?;
                self.visit_statement(body)
            }
            IterationStatement::Error => Ok(()),
        }
    }

    fn visit_for_init(&mut self, fi: &'a ForInit) -> Self::Result {
        match fi {
            ForInit::Expression(e) => {
                self.visit_expression(e)?;
                self.text(";")
            }
            ForInit::Declaration(d) => {
                self.visit_declaration(d) // Declaration already includes the semicolon
            }
        }
    }

    fn visit_expression(&mut self, e: &'a Expression) -> Self::Result {
        let ctx = self.extra;
        let expr_prec = expr_precedence(e);
        let needs_parens = ctx.needs_parens(expr_prec);

        if needs_parens {
            self.text("(")?;
        }

        // Reset context for inner expression processing
        self.extra = Context::default();

        match e {
            Expression::Postfix(p) => self.visit_postfix_expression(p)?,
            Expression::Unary(u) => self.visit_unary_expression(u)?,
            Expression::Cast(c) => self.visit_cast_expression(c)?,
            Expression::Binary(b) => {
                let op_prec = binary_op_precedence(&b.operator);
                // All binary operators are left-associative in C
                // Left operand: same precedence, no assoc flag (left side of left-assoc is fine)
                self.extra = Context { precedence: op_prec, assoc: false };
                self.visit_expression(&b.left)?;
                self.space()?;
                self.visit_binary_operator(&b.operator)?;
                self.space()?;
                // Right operand: same precedence, assoc = true (right side of left-assoc needs parens at equal prec)
                self.extra = Context { precedence: op_prec, assoc: true };
                self.visit_expression(&b.right)?;
            }
            Expression::Conditional(cond) => {
                // Conditional is right-associative
                // Condition needs parens if it's a conditional or lower precedence
                self.extra = Context {
                    precedence: precedence::CONDITIONAL,
                    assoc: true,
                };
                self.visit_expression(&cond.condition)?;
                self.space()?;
                self.text("?")?;
                self.space()?;
                // then_expr can be any expression (comma is allowed inside ?:)
                self.extra = Context::default();
                self.visit_expression(&cond.then_expr)?;
                self.space()?;
                self.text(":")?;
                self.space()?;
                // else_expr: right-associative, so same precedence is OK
                self.extra = Context {
                    precedence: precedence::CONDITIONAL,
                    assoc: false,
                };
                self.visit_expression(&cond.else_expr)?;
            }
            Expression::Assignment(a) => {
                // Assignment is right-associative
                // Left operand needs higher precedence (unary or above)
                self.extra = Context {
                    precedence: precedence::ASSIGNMENT,
                    assoc: true,
                };
                self.visit_expression(&a.left)?;
                self.space()?;
                self.visit_assignment_operator(&a.operator)?;
                self.space()?;
                // Right operand: right-associative, so same precedence is OK
                self.extra = Context {
                    precedence: precedence::ASSIGNMENT,
                    assoc: false,
                };
                self.visit_expression(&a.right)?;
            }
            Expression::Comma(c) => {
                for (i, expr) in c.expressions.iter().enumerate() {
                    if i > 0 {
                        self.text(",")?;
                        self.space()?;
                    }
                    // Comma is left-associative
                    // Each element needs to be at least assignment level
                    self.extra = Context {
                        precedence: precedence::COMMA,
                        assoc: i > 0, // right side needs parens at equal precedence
                    };
                    self.visit_expression(expr)?;
                }
            }
            Expression::Error => {}
        }

        // Restore context
        self.extra = ctx;

        if needs_parens {
            self.text(")")?;
        }

        Ok(())
    }

    fn visit_binary_operator(&mut self, op: &'a BinaryOperator) -> Self::Result {
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
        self.text(text)
    }

    fn visit_assignment_operator(&mut self, op: &'a AssignmentOperator) -> Self::Result {
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
        self.text(text)
    }

    fn visit_declaration(&mut self, d: &'a Declaration) -> Self::Result {
        self.igroup(2, |pp| {
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
                                pp.visit_initializer(initializer)?;
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
                    // Print declaration specifiers, but skip Typedef storage class
                    // since we already printed "typedef"
                    let mut first = true;
                    for spec in specifiers.specifiers.iter() {
                        match spec {
                            DeclarationSpecifier::StorageClass(StorageClassSpecifier::Typedef) => {
                                // Skip the Typedef storage class, we already printed it
                                continue;
                            }
                            _ => {
                                if !first {
                                    pp.space()?;
                                }
                                first = false;
                                pp.visit_declaration_specifier(spec)?;
                            }
                        }
                    }
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
                    pp.visit_constant_expression(&sa.condition)?;
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
        })
    }

    fn visit_declarator(&mut self, d: &'a Declarator) -> Self::Result {
        match d {
            Declarator::Direct(dd) => self.visit_direct_declarator(dd),
            Declarator::Pointer { pointer, declarator } => {
                self.visit_pointer(pointer)?;
                self.space()?;
                self.visit_declarator(declarator)
            }
            Declarator::Error => Ok(()),
        }
    }

    fn visit_direct_declarator(&mut self, d: &'a DirectDeclarator) -> Self::Result {
        match d {
            DirectDeclarator::Identifier { identifier, attributes } => {
                self.visit_variable_name(identifier)?;
                for a in attributes {
                    self.space()?;
                    self.visit_attribute_specifier(a)?;
                }
                Ok(())
            }
            DirectDeclarator::Parenthesized(inner) => {
                self.text("(")?;
                self.visit_declarator(inner)?;
                self.text(")")
            }
            DirectDeclarator::Array { declarator, attributes, array_declarator } => {
                self.visit_direct_declarator(declarator)?;
                for a in attributes {
                    self.space()?;
                    self.visit_attribute_specifier(a)?;
                }
                self.text("[")?;
                self.visit_array_declarator(array_declarator)?;
                self.text("]")
            }
            DirectDeclarator::Function { declarator, attributes, parameters } => {
                self.visit_direct_declarator(declarator)?;
                self.igroup(2, |pp| {
                    pp.text("(")?;
                    pp.visit_parameter_type_list(parameters)?;
                    pp.scan_break(0, -2)?;
                    pp.text(")")
                })?;
                for a in attributes {
                    self.space()?;
                    self.visit_attribute_specifier(a)?;
                }
                Ok(())
            }
        }
    }

    fn visit_postfix_expression(&mut self, p: &'a PostfixExpression) -> Self::Result {
        match p {
            PostfixExpression::Primary(pr) => self.visit_primary_expression(pr),
            PostfixExpression::ArrayAccess { array, index } => {
                self.visit_postfix_expression(array)?;
                self.text("[")?;
                // Reset context since brackets protect the index expression
                let old_ctx = self.extra;
                self.extra = Context::default();
                self.visit_expression(index)?;
                self.extra = old_ctx;
                self.text("]")
            }
            PostfixExpression::FunctionCall { function, arguments } => {
                self.visit_postfix_expression(function)?;
                self.igroup(2, |pp| {
                    pp.text("(")?;
                    for (i, arg) in arguments.iter().enumerate() {
                        if i > 0 {
                            pp.text(",")?;
                            pp.space()?;
                        }
                        // Reset context since function call parens protect arguments
                        let old_ctx = pp.extra;
                        pp.extra = Context::default();
                        pp.visit_expression(arg)?;
                        pp.extra = old_ctx;
                    }
                    pp.scan_break(0, -2)?;
                    pp.text(")")
                })
            }
            PostfixExpression::MemberAccess { object, member } => {
                self.visit_postfix_expression(object)?;
                self.text(".")?;
                self.visit_member_name(member)
            }
            PostfixExpression::MemberAccessPtr { object, member } => {
                self.visit_postfix_expression(object)?;
                self.text("->")?;
                self.visit_member_name(member)
            }
            PostfixExpression::PostIncrement(inner) => {
                self.visit_postfix_expression(inner)?;
                self.text("++")
            }
            PostfixExpression::PostDecrement(inner) => {
                self.visit_postfix_expression(inner)?;
                self.text("--")
            }
            PostfixExpression::CompoundLiteral(cl) => {
                self.text("(")?;
                for scs in &cl.storage_class_specifiers {
                    self.visit_storage_class_specifier(scs)?;
                    self.space()?;
                }
                self.visit_type_name(&cl.type_name)?;
                self.text(")")?;
                self.visit_braced_initializer(&cl.initializer)
            }
        }
    }

    fn visit_primary_expression(&mut self, pr: &'a PrimaryExpression) -> Self::Result {
        match pr {
            PrimaryExpression::Identifier(id) => self.visit_variable_name(id),
            PrimaryExpression::Constant(c) => print_constant(self, c),
            PrimaryExpression::EnumerationConstant(id) => self.visit_enum_constant(id),
            PrimaryExpression::StringLiteral(lits) => {
                for (i, lit) in lits.0.iter().enumerate() {
                    if i > 0 {
                        self.space()?;
                    }
                    print_string_literal(self, lit)?;
                }
                Ok(())
            }
            PrimaryExpression::QuotedString(s) => {
                self.text("`")?;
                self.text(s)?;
                self.text("`")
            }
            PrimaryExpression::Parenthesized(e) => {
                self.text("(")?;
                // Reset context since the parens we're printing protect the inner expression
                let old_ctx = self.extra;
                self.extra = Context::default();
                self.visit_expression(e)?;
                self.extra = old_ctx;
                self.text(")")
            }
            PrimaryExpression::Generic(g) => {
                self.text("_Generic")?;
                self.igroup(2, |pp| {
                    pp.text("(")?;
                    // Reset context since _Generic parens protect the expressions
                    let old_ctx = pp.extra;
                    pp.extra = Context::default();
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
                    pp.extra = old_ctx;
                    pp.scan_break(0, -2)?;
                    pp.text(")")
                })
            }
            PrimaryExpression::Error => Ok(()),
        }
    }

    fn visit_unary_expression(&mut self, u: &'a UnaryExpression) -> Self::Result {
        match u {
            UnaryExpression::Postfix(p) => self.visit_postfix_expression(p),
            UnaryExpression::PreIncrement(inner) => {
                self.text("++")?;
                // Unary operators are right-associative, operand needs unary precedence
                let old_ctx = self.extra;
                self.extra = Context {
                    precedence: precedence::UNARY,
                    assoc: false,
                };
                self.visit_unary_expression(inner)?;
                self.extra = old_ctx;
                Ok(())
            }
            UnaryExpression::PreDecrement(inner) => {
                self.text("--")?;
                let old_ctx = self.extra;
                self.extra = Context {
                    precedence: precedence::UNARY,
                    assoc: false,
                };
                self.visit_unary_expression(inner)?;
                self.extra = old_ctx;
                Ok(())
            }
            UnaryExpression::Unary { operator, operand } => {
                self.visit_unary_operator(operator)?;
                let old_ctx = self.extra;
                self.extra = Context {
                    precedence: precedence::CAST,
                    assoc: false,
                };
                self.visit_cast_expression(operand)?;
                self.extra = old_ctx;
                Ok(())
            }
            UnaryExpression::Sizeof(inner) => {
                self.text("sizeof")?;
                self.space()?;
                let old_ctx = self.extra;
                self.extra = Context {
                    precedence: precedence::UNARY,
                    assoc: false,
                };
                self.visit_unary_expression(inner)?;
                self.extra = old_ctx;
                Ok(())
            }
            UnaryExpression::SizeofType(tn) => {
                self.text("sizeof")?;
                self.text("(")?;
                self.visit_type_name(tn)?;
                self.text(")")
            }
            UnaryExpression::Alignof(tn) => {
                self.text("_Alignof")?;
                self.text("(")?;
                self.visit_type_name(tn)?;
                self.text(")")
            }
        }
    }

    fn visit_unary_operator(&mut self, op: &'a UnaryOperator) -> Self::Result {
        let text = match op {
            UnaryOperator::Address => "&",
            UnaryOperator::Dereference => "*",
            UnaryOperator::Plus => "+",
            UnaryOperator::Minus => "-",
            UnaryOperator::BitwiseNot => "~",
            UnaryOperator::LogicalNot => "!",
        };
        self.text(text)
    }

    fn visit_cast_expression(&mut self, c: &'a CastExpression) -> Self::Result {
        match c {
            CastExpression::Unary(u) => self.visit_unary_expression(u),
            CastExpression::Cast { type_name, expression } => {
                self.text("(")?;
                self.visit_type_name(type_name)?;
                self.text(")")?;
                // Cast is right-associative
                let old_ctx = self.extra;
                self.extra = Context {
                    precedence: precedence::CAST,
                    assoc: false,
                };
                self.visit_cast_expression(expression)?;
                self.extra = old_ctx;
                Ok(())
            }
        }
    }

    fn visit_compound_statement(&mut self, c: &'a CompoundStatement) -> Self::Result {
        self.cgroup(2, |pp| {
            pp.text("{")?;

            for item in &c.items {
                pp.igroup(2, |pp| pp.visit_block_item(item))?;
                pp.space()?;
            }
            pp.scan_break(1, -2)?;
            pp.text("}")
        })
    }

    fn visit_declaration_specifiers(&mut self, s: &'a DeclarationSpecifiers) -> Self::Result {
        for (i, spec) in s.specifiers.iter().enumerate() {
            if i > 0 {
                self.space()?;
            }
            self.visit_declaration_specifier(spec)?;
        }
        Ok(())
    }

    fn visit_declaration_specifier(&mut self, spec: &'a DeclarationSpecifier) -> Self::Result {
        match spec {
            DeclarationSpecifier::StorageClass(scs) => self.visit_storage_class_specifier(scs),
            DeclarationSpecifier::TypeSpecifierQualifier(tsq) => self.visit_type_specifier_qualifier(tsq),
            DeclarationSpecifier::Function(fs) => self.visit_function_specifier(fs),
        }
    }

    fn visit_storage_class_specifier(&mut self, scs: &'a StorageClassSpecifier) -> Self::Result {
        let text = match scs {
            StorageClassSpecifier::Auto => "auto",
            StorageClassSpecifier::Constexpr => "constexpr",
            StorageClassSpecifier::Extern => "extern",
            StorageClassSpecifier::Register => "register",
            StorageClassSpecifier::Static => "static",
            StorageClassSpecifier::ThreadLocal => "_Thread_local",
            StorageClassSpecifier::Typedef => "typedef",
        };
        self.text(text)
    }

    fn visit_function_specifier(&mut self, fs: &'a FunctionSpecifier) -> Self::Result {
        let text = match fs {
            FunctionSpecifier::Inline => "inline",
            FunctionSpecifier::Noreturn => "_Noreturn",
        };
        self.text(text)
    }

    fn visit_type_qualifier(&mut self, tq: &'a TypeQualifier) -> Self::Result {
        let text = match tq {
            TypeQualifier::Const => "const",
            TypeQualifier::Restrict => "restrict",
            TypeQualifier::Volatile => "volatile",
            TypeQualifier::Atomic => "_Atomic",
            TypeQualifier::Nonnull => "_Nonnull",
            TypeQualifier::Nullable => "_Nullable",
            TypeQualifier::ThreadLocal => "_Thread_local",
        };
        self.text(text)
    }

    fn visit_alignment_specifier(&mut self, a: &'a AlignmentSpecifier) -> Self::Result {
        self.text("_Alignas")?;
        self.text("(")?;
        match a {
            AlignmentSpecifier::Type(tn) => self.visit_type_name(tn)?,
            AlignmentSpecifier::Expression(e) => self.visit_constant_expression(e)?,
        }
        self.text(")")
    }

    fn visit_type_specifier(&mut self, ts: &'a TypeSpecifier) -> Self::Result {
        match ts {
            TypeSpecifier::Void => self.text("void"),
            TypeSpecifier::Char => self.text("char"),
            TypeSpecifier::Short => self.text("short"),
            TypeSpecifier::Int => self.text("int"),
            TypeSpecifier::Long => self.text("long"),
            TypeSpecifier::Float => self.text("float"),
            TypeSpecifier::Double => self.text("double"),
            TypeSpecifier::Signed => self.text("signed"),
            TypeSpecifier::Unsigned => self.text("unsigned"),
            TypeSpecifier::BitInt(e) => {
                self.text("_BitInt")?;
                self.text("(")?;
                self.visit_constant_expression(e)?;
                self.text(")")
            }
            TypeSpecifier::Bool => self.text("_Bool"),
            TypeSpecifier::Complex => self.text("_Complex"),
            TypeSpecifier::Decimal32 => self.text("_Decimal32"),
            TypeSpecifier::Decimal64 => self.text("_Decimal64"),
            TypeSpecifier::Decimal128 => self.text("_Decimal128"),
            TypeSpecifier::Atomic(a) => self.visit_atomic_type_specifier(a),
            TypeSpecifier::Struct(s) => self.visit_struct_union_specifier(s),
            TypeSpecifier::Enum(e) => self.visit_enum_specifier(e),
            TypeSpecifier::TypedefName(id) => self.visit_type_name_identifier(id),
            TypeSpecifier::Typeof(t) => self.visit_typeof(t),
        }
    }

    fn visit_struct_union_specifier(&mut self, s: &'a StructOrUnionSpecifier) -> Self::Result {
        let keyword = match s.kind {
            StructOrUnion::Struct => "struct",
            StructOrUnion::Union => "union",
        };
        self.text(keyword)?;

        for a in &s.attributes {
            self.space()?;
            self.visit_attribute_specifier(a)?;
        }

        if let Some(id) = &s.identifier {
            self.space()?;
            self.visit_struct_name(id)?;
        }

        if let Some(members) = &s.members {
            self.space()?;
            self.igroup(2, |pp| {
                pp.text("{")?;
                pp.hard_break()?;
                for member in members {
                    pp.visit_member_declaration(member)?;
                    pp.hard_break()?;
                }
                pp.scan_break(0, -2)?;
                pp.text("}")
            })?;
        }

        Ok(())
    }

    fn visit_member_declaration(&mut self, m: &'a MemberDeclaration) -> Self::Result {
        match m {
            MemberDeclaration::Normal { attributes, specifiers, declarators } => {
                for a in attributes {
                    self.visit_attribute_specifier(a)?;
                    self.space()?;
                }
                self.visit_specifier_qualifier_list(specifiers)?;
                if !declarators.is_empty() {
                    self.space()?;
                    for (i, decl) in declarators.iter().enumerate() {
                        if i > 0 {
                            self.text(",")?;
                            self.space()?;
                        }
                        self.visit_member_declarator(decl)?;
                    }
                }
                self.text(";")
            }
            MemberDeclaration::StaticAssert(sa) => self.visit_static_assert_declaration(sa),
            MemberDeclaration::Error => Ok(()),
        }
    }

    fn visit_member_declarator(&mut self, md: &'a MemberDeclarator) -> Self::Result {
        match md {
            MemberDeclarator::Declarator(d) => self.visit_declarator(d),
            MemberDeclarator::BitField { declarator, width } => {
                if let Some(d) = declarator {
                    self.visit_declarator(d)?;
                }
                self.text(":")?;
                self.space()?;
                self.visit_constant_expression(width)
            }
        }
    }

    fn visit_static_assert_declaration(&mut self, sa: &'a StaticAssertDeclaration) -> Self::Result {
        self.text("_Static_assert")?;
        self.text("(")?;
        self.visit_constant_expression(&sa.condition)?;
        if let Some(msg) = &sa.message {
            self.text(",")?;
            self.space()?;
            self.visit_static_assert_message(msg)?;
        }
        self.text(");")
    }

    fn visit_static_assert_message(&mut self, msg: &'a StringLiterals) -> Self::Result {
        for (i, lit) in msg.0.iter().enumerate() {
            if i > 0 {
                self.space()?;
            }
            print_string_literal(self, lit)?;
        }
        Ok(())
    }

    fn visit_enum_specifier(&mut self, e: &'a EnumSpecifier) -> Self::Result {
        self.text("enum")?;

        for a in &e.attributes {
            self.space()?;
            self.visit_attribute_specifier(a)?;
        }

        if let Some(id) = &e.identifier {
            self.space()?;
            self.visit_enum_name(id)?;
        }

        if let Some(type_spec) = &e.type_specifier {
            self.space()?;
            self.text(":")?;
            self.space()?;
            self.visit_specifier_qualifier_list(type_spec)?;
        }

        if let Some(enumerators) = &e.enumerators {
            self.space()?;
            self.cgroup(2, |pp| {
                pp.text("{")?;
                pp.space()?;
                for (i, e) in enumerators.iter().enumerate() {
                    if i > 0 {
                        pp.text(",")?;
                        pp.space()?;
                    }
                    pp.visit_enumerator(e)?;
                }
                pp.scan_break(0, -2)?;
                pp.text("}")
            })?;
        }

        Ok(())
    }

    fn visit_enumerator(&mut self, e: &'a Enumerator) -> Self::Result {
        self.igroup(2, |pp| {
            for a in &e.attributes {
                pp.visit_attribute_specifier(a)?;
                pp.space()?;
            }
            pp.visit_enumerator_name(&e.name)?;
            if let Some(value) = &e.value {
                pp.space()?;
                pp.text("=")?;
                pp.space()?;
                pp.visit_constant_expression(value)?;
            }
            Ok(())
        })
    }

    fn visit_atomic_type_specifier(&mut self, a: &'a AtomicTypeSpecifier) -> Self::Result {
        self.text("_Atomic")?;
        self.text("(")?;
        self.visit_type_name(&a.type_name)?;
        self.text(")")
    }

    fn visit_typeof(&mut self, t: &'a TypeofSpecifier) -> Self::Result {
        match t {
            TypeofSpecifier::Typeof(_) => self.text("typeof")?,
            TypeofSpecifier::TypeofUnqual(_) => self.text("typeof_unqual")?,
        }
        match t {
            TypeofSpecifier::Typeof(arg) | TypeofSpecifier::TypeofUnqual(arg) => {
                self.text("(")?;
                match arg {
                    TypeofSpecifierArgument::Expression(e) => self.visit_expression(e)?,
                    TypeofSpecifierArgument::TypeName(tn) => self.visit_type_name(tn)?,
                    TypeofSpecifierArgument::Error => {}
                }
                self.text(")")
            }
        }
    }

    fn visit_specifier_qualifier_list(&mut self, s: &'a SpecifierQualifierList) -> Self::Result {
        for (i, item) in s.items.iter().enumerate() {
            if i > 0 {
                self.space()?;
            }
            self.visit_type_specifier_qualifier(item)?;
        }
        for a in &s.attributes {
            self.space()?;
            self.visit_attribute_specifier(a)?;
        }
        Ok(())
    }

    fn visit_type_name(&mut self, tn: &'a TypeName) -> Self::Result {
        match tn {
            TypeName::TypeName { specifiers, abstract_declarator } => {
                self.visit_specifier_qualifier_list(specifiers)?;
                if let Some(ad) = abstract_declarator {
                    self.space()?;
                    self.visit_abstract_declarator(ad)?;
                }
                Ok(())
            }
            TypeName::Error => Ok(()),
        }
    }

    fn visit_abstract_declarator(&mut self, a: &'a AbstractDeclarator) -> Self::Result {
        match a {
            AbstractDeclarator::Direct(d) => self.visit_direct_abstract_declarator(d),
            AbstractDeclarator::Pointer { pointer, abstract_declarator } => {
                self.visit_pointer(pointer)?;
                if let Some(ad) = abstract_declarator {
                    self.space()?;
                    self.visit_abstract_declarator(ad)?;
                }
                Ok(())
            }
            AbstractDeclarator::Error => Ok(()),
        }
    }

    fn visit_direct_abstract_declarator(&mut self, d: &'a DirectAbstractDeclarator) -> Self::Result {
        match d {
            DirectAbstractDeclarator::Parenthesized(ad) => {
                self.text("(")?;
                self.visit_abstract_declarator(ad)?;
                self.text(")")
            }
            DirectAbstractDeclarator::Array { declarator, attributes, array_declarator } => {
                if let Some(dd) = declarator {
                    self.visit_direct_abstract_declarator(dd)?;
                }
                for a in attributes {
                    self.space()?;
                    self.visit_attribute_specifier(a)?;
                }
                self.text("[")?;
                self.visit_array_declarator(array_declarator)?;
                self.text("]")
            }
            DirectAbstractDeclarator::Function { declarator, attributes, parameters } => {
                if let Some(dd) = declarator {
                    self.visit_direct_abstract_declarator(dd)?;
                }
                self.igroup(2, |pp| {
                    pp.text("(")?;
                    pp.visit_parameter_type_list(parameters)?;
                    pp.scan_break(0, -2)?;
                    pp.text(")")
                })?;
                for a in attributes {
                    self.space()?;
                    self.visit_attribute_specifier(a)?;
                }
                Ok(())
            }
        }
    }

    fn visit_braced_initializer(&mut self, b: &'a BracedInitializer) -> Self::Result {
        self.cgroup(2, |pp| {
            pp.text("{")?;
            for (i, init) in b.initializers.iter().enumerate() {
                if i > 0 {
                    pp.text(",")?;
                }
                pp.space()?;
                pp.visit_designated_initializer(init)?;
            }
            pp.scan_break(0, -2)?;
            pp.text("}")
        })
    }

    fn visit_designated_initializer(&mut self, init: &'a DesignatedInitializer) -> Self::Result {
        self.igroup(2, |pp| {
            if let Some(designation) = &init.designation {
                pp.visit_designation(designation)?;
                pp.space()?;
                pp.text("=")?;
                pp.space()?;
            }
            pp.visit_initializer(&init.initializer)
        })
    }

    fn visit_designation(&mut self, d: &'a Designation) -> Self::Result {
        if let Some(next) = &d.designation {
            self.visit_designation(next)?;
        }
        match &d.designator {
            Designator::Array(e) => {
                self.text("[")?;
                self.visit_constant_expression(e)?;
                self.text("]")
            }
            Designator::Member(id) => {
                self.text(".")?;
                self.visit_member_name(id)
            }
        }
    }

    fn visit_pointer(&mut self, p: &'a Pointer) -> Self::Result {
        self.visit_pointer_or_block(&p.pointer_or_block)?;
        for a in &p.attributes {
            self.space()?;
            self.visit_attribute_specifier(a)?;
        }
        for tq in &p.type_qualifiers {
            self.space()?;
            self.visit_type_qualifier(tq)?;
        }
        Ok(())
    }

    fn visit_pointer_or_block(&mut self, pb: &'a PointerOrBlock) -> Self::Result {
        match pb {
            PointerOrBlock::Pointer => self.text("*"),
            PointerOrBlock::Block => self.text("^"),
        }
    }

    fn visit_array_declarator(&mut self, a: &'a ArrayDeclarator) -> Self::Result {
        match a {
            ArrayDeclarator::Normal { type_qualifiers, size } => {
                for (i, tq) in type_qualifiers.iter().enumerate() {
                    if i > 0 {
                        self.space()?;
                    }
                    self.visit_type_qualifier(tq)?;
                }
                if let Some(s) = size {
                    if !type_qualifiers.is_empty() {
                        self.space()?;
                    }
                    self.visit_expression(s)?;
                }
                Ok(())
            }
            ArrayDeclarator::Static { type_qualifiers, size } => {
                self.text("static")?;
                for tq in type_qualifiers {
                    self.space()?;
                    self.visit_type_qualifier(tq)?;
                }
                self.space()?;
                self.visit_expression(size)
            }
            ArrayDeclarator::VLA { type_qualifiers } => {
                for (i, tq) in type_qualifiers.iter().enumerate() {
                    if i > 0 {
                        self.space()?;
                    }
                    self.visit_type_qualifier(tq)?;
                }
                if !type_qualifiers.is_empty() {
                    self.space()?;
                }
                self.text("*")
            }
            ArrayDeclarator::Error => Ok(()),
        }
    }

    fn visit_parameter_type_list(&mut self, p: &'a ParameterTypeList) -> Self::Result {
        match p {
            ParameterTypeList::Parameters(params) => {
                for (i, param) in params.iter().enumerate() {
                    if i > 0 {
                        self.text(",")?;
                        self.space()?;
                    }
                    self.visit_parameter_declaration(param)?;
                }
                Ok(())
            }
            ParameterTypeList::Variadic(params) => {
                for (i, param) in params.iter().enumerate() {
                    if i > 0 {
                        self.text(",")?;
                        self.space()?;
                    }
                    self.visit_parameter_declaration(param)?;
                }
                self.text(",")?;
                self.space()?;
                self.text("...")
            }
            ParameterTypeList::OnlyVariadic => self.text("..."),
        }
    }

    fn visit_parameter_declaration(&mut self, p: &'a ParameterDeclaration) -> Self::Result {
        for a in &p.attributes {
            self.visit_attribute_specifier(a)?;
            self.space()?;
        }
        self.visit_declaration_specifiers(&p.specifiers)?;
        if let Some(kind) = &p.declarator {
            self.space()?;
            self.visit_parameter_declaration_kind(kind)?;
        }
        Ok(())
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
            BalancedToken::Identifier(identifier) => pp.text(&identifier.0)?,
            BalancedToken::StringLiteral(string_literals) => {
                for (i, lit) in string_literals.0.iter().enumerate() {
                    if i > 0 {
                        pp.space()?;
                    }
                    print_string_literal(pp, lit)?;
                }
            }
            BalancedToken::QuotedString(lit) => {
                pp.text("`")?;
                pp.text(lit)?;
                pp.text("`")?;
            }
            BalancedToken::Constant(constant) => print_constant(pp, constant)?,
            BalancedToken::Punctuator(punctuator) => print_punctuator(pp, punctuator)?,
            #[cfg(feature = "quasi-quote")]
            BalancedToken::Interpolation(_) => {}
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
            let value = floating_constant.value;
            // Handle special floating point values
            if value.is_infinite() || value.is_nan() {
                panic!("Cannot print infinite or NaN floating point literals");
            } else {
                let value_str = value.to_string();
                // Ensure we have a decimal point or exponent for floating point literals
                // to maintain type information
                if !value_str.contains('.') && !value_str.contains('e') && !value_str.contains('E') {
                    pp.text_owned(format!("{:.1}", value))?;
                } else {
                    pp.text_owned(value_str)?;
                }
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
