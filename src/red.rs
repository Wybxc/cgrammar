//! Red tree — position-aware cursors over the green tree.
//!
//! The red tree wraps a [`GreenNode`] with absolute source positions, parent
//! pointers, and sibling navigation. It provides a uniform API for traversing
//! the syntax tree regardless of node type.
//!
//! # Design
//!
//! [`SyntaxTree`] stores a flat array of [`SyntaxNodeData`] built by a
//! pre-order DFS walk of the green tree. Each node stores its absolute byte
//! offset, parent index, and first-child/next-sibling indices. [`SyntaxNode`]
//! is a `Copy` newtype around an index into this array, making traversal
//! cheap and allocation-free.
//!
//! The "first child / next sibling" representation enables O(1) sibling
//! iteration without storing a separate children array.

#![allow(missing_docs)]

use crate::green::{GreenChild, GreenNode, GreenToken};
use crate::syntax::SyntaxKind;

/// Sentinel index for absent relationships.
const NONE: u32 = u32::MAX;

/// A position-aware syntax tree built from a green tree.
///
/// Stores pre-computed offsets and parent/sibling indices for efficient
/// traversal.
#[derive(Debug, Clone)]
pub struct SyntaxTree {
    nodes: Vec<SyntaxNodeData>,
}

#[derive(Debug, Clone)]
struct SyntaxNodeData {
    /// Absolute byte offset from the start of the source.
    offset: u32,
    /// Index of the parent node, or [`NONE`] for the root.
    parent: u32,
    /// Index of the first child, or [`NONE`] if leaf.
    first_child: u32,
    /// Index of the next sibling, or [`NONE`] if last.
    next_sibling: u32,
    /// The green node data (shared via the root).
    green: GreenNode,
}

/// A reference to a node in a [`SyntaxTree`].
///
/// `SyntaxNode` is a cheap `Copy` type. All operations are performed on the
/// associated [`SyntaxTree`], which owns the data.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SyntaxNode {
    idx: u32,
}

impl SyntaxTree {
    /// Build a [`SyntaxTree`] from a root green node.
    ///
    /// Walks the green tree in pre-order, computing absolute byte offsets
    /// and establishing parent/sibling relationships.
    pub fn new(root: GreenNode) -> Self {
        let mut nodes = Vec::new();
        build_recursive(&root, NONE, NONE, 0, &mut nodes);
        SyntaxTree { nodes }
    }

    /// Returns the root node.
    pub fn root(&self) -> SyntaxNode {
        SyntaxNode { idx: 0 }
    }

    /// Returns the [`SyntaxKind`] of a node.
    pub fn kind(&self, node: SyntaxNode) -> SyntaxKind {
        self.data(node).green.kind
    }

    /// Returns the absolute byte offset of a node.
    pub fn offset(&self, node: SyntaxNode) -> u32 {
        self.data(node).offset
    }

    /// Returns the text length of a node in bytes.
    pub fn len(&self, node: SyntaxNode) -> u32 {
        self.data(node).green.len
    }

    /// Returns `true` if the node is a leaf token.
    pub fn is_token(&self, node: SyntaxNode) -> bool {
        self.data(node).green.is_token()
    }

    /// Returns the token data if the node is a token.
    pub fn as_token(&self, node: SyntaxNode) -> Option<&GreenToken> {
        self.data(node).green.as_token()
    }

    /// Returns the parent of a node, or `None` if it is the root.
    pub fn parent(&self, node: SyntaxNode) -> Option<SyntaxNode> {
        let p = self.data(node).parent;
        if p == NONE { None } else { Some(SyntaxNode { idx: p }) }
    }

    /// Returns the first child of a node, or `None` if it has no children.
    pub fn first_child(&self, node: SyntaxNode) -> Option<SyntaxNode> {
        let c = self.data(node).first_child;
        if c == NONE { None } else { Some(SyntaxNode { idx: c }) }
    }

    /// Returns the next sibling of a node, or `None` if it is the last child.
    pub fn next_sibling(&self, node: SyntaxNode) -> Option<SyntaxNode> {
        let s = self.data(node).next_sibling;
        if s == NONE { None } else { Some(SyntaxNode { idx: s }) }
    }

    /// Reconstruct the original source text from the green tree.
    ///
    /// This is a convenience wrapper around [`print_lossless`].
    pub fn lossless_text(&self, source: &str) -> String {
        print_lossless(self, source)
    }

    /// Returns an iterator over the children of a node.
    pub fn children(&self, node: SyntaxNode) -> SyntaxChildren<'_> {
        SyntaxChildren {
            tree: self,
            current: self.data(node).first_child,
        }
    }

    /// Returns the green node behind this syntax node.
    pub fn green(&self, node: SyntaxNode) -> &GreenNode {
        &self.data(node).green
    }

    /// Returns the text of a token node using the source string.
    ///
    /// Returns `None` if the node is not a token.
    pub fn text<'a>(&self, node: SyntaxNode, source: &'a str) -> Option<&'a str> {
        let token = self.as_token(node)?;
        let start = self.offset(node) as usize + token.leading_trivia as usize;
        let end = start + token.len as usize;
        Some(&source[start..end])
    }

    /// Returns the number of nodes in the tree.
    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    /// Reconstruct a flat [`BalancedTokenSequence`] from the subtree rooted at
    /// `node`. Walks the tree pre-order, mapping each token's [`SyntaxKind`]
    /// and source text back to a [`BalancedToken`].
    ///
    /// Interior nodes are traversed transparently — only token leaves appear
    /// in the output. The result can be fed back to the parser via
    /// [`BalancedTokenSequence::as_input`].
    pub fn to_token_sequence(&self, node: SyntaxNode, source: &str) -> crate::token::BalancedTokenSequence {
        let mut tokens: Vec<crate::span::Spanned<crate::token::BalancedToken>> = Vec::new();
        collect_tokens(self, node, source, &mut tokens);
        let eoi_pos = self.offset(node) as usize + self.len(node) as usize;
        let eoi = crate::span::Span::new_eoi(eoi_pos, crate::span::ContextId::from(0usize));
        crate::token::BalancedTokenSequence { tokens, eoi }
    }

    fn data(&self, node: SyntaxNode) -> &SyntaxNodeData {
        &self.nodes[node.idx as usize]
    }
}

/// Recursively collect tokens from a syntax subtree.
fn collect_tokens(
    tree: &SyntaxTree,
    node: SyntaxNode,
    source: &str,
    tokens: &mut Vec<crate::span::Spanned<crate::token::BalancedToken>>,
) {
    use crate::token::{BalancedToken, Constant, Identifier, IntegerConstant, StringLiterals};
    use crate::span::Spanned;

    if tree.is_token(node) {
        let kind = tree.kind(node);
        let offset = tree.offset(node) as usize;
        let text = tree.text(node, source).unwrap_or("");
        let ctx = crate::span::ContextId::from(0usize);
        let span = crate::span::Span::new(offset..offset + text.len(), ctx);

        let token = match kind {
            crate::syntax::SyntaxKind::Ident => {
                BalancedToken::Identifier(Identifier(text.into()))
            }
            crate::syntax::SyntaxKind::IntegerConst => {
                BalancedToken::Constant(Constant::Integer(IntegerConstant::from(0)))
            }
            crate::syntax::SyntaxKind::FloatConst => {
                BalancedToken::Constant(Constant::Floating(Default::default()))
            }
            crate::syntax::SyntaxKind::CharConst => {
                BalancedToken::Constant(Constant::Character(Default::default()))
            }
            crate::syntax::SyntaxKind::PredefinedConst => {
                BalancedToken::Constant(Constant::Predefined(match text {
                    "true" => crate::token::PredefinedConstant::True,
                    "false" => crate::token::PredefinedConstant::False,
                    _ => crate::token::PredefinedConstant::Nullptr,
                }))
            }
            crate::syntax::SyntaxKind::StringLiteral => {
                BalancedToken::StringLiteral(StringLiterals::from(text.to_string()))
            }
            crate::syntax::SyntaxKind::QuotedString => {
                BalancedToken::QuotedString(text.to_string())
            }
            crate::syntax::SyntaxKind::Unknown => BalancedToken::Unknown,
            _ => {
                // Must be a punctuator
                if let Some(p) = syntax_kind_to_punctuator(kind) {
                    BalancedToken::Punctuator(p)
                } else {
                    return; // Skip non-token, non-punctuator kinds (trivia, etc.)
                }
            }
        };
        tokens.push(Spanned::new(token, span));
    } else {
        for child in tree.children(node) {
            collect_tokens(tree, child, source, tokens);
        }
    }
}

/// Map a [`SyntaxKind`] to its corresponding [`Punctuator`].
fn syntax_kind_to_punctuator(kind: crate::syntax::SyntaxKind) -> Option<crate::token::Punctuator> {
    use crate::{syntax::SyntaxKind, token::Punctuator};
    Some(match kind {
        SyntaxKind::LeftParen => Punctuator::LeftParen,
        SyntaxKind::RightParen => Punctuator::RightParen,
        SyntaxKind::LeftBracket => Punctuator::LeftBracket,
        SyntaxKind::RightBracket => Punctuator::RightBracket,
        SyntaxKind::LeftBrace => Punctuator::LeftBrace,
        SyntaxKind::RightBrace => Punctuator::RightBrace,
        SyntaxKind::Dot => Punctuator::Dot,
        SyntaxKind::Arrow => Punctuator::Arrow,
        SyntaxKind::Increment => Punctuator::Increment,
        SyntaxKind::Decrement => Punctuator::Decrement,
        SyntaxKind::Ampersand => Punctuator::Ampersand,
        SyntaxKind::Star => Punctuator::Star,
        SyntaxKind::Plus => Punctuator::Plus,
        SyntaxKind::Minus => Punctuator::Minus,
        SyntaxKind::Tilde => Punctuator::Tilde,
        SyntaxKind::Bang => Punctuator::Bang,
        SyntaxKind::Slash => Punctuator::Slash,
        SyntaxKind::Percent => Punctuator::Percent,
        SyntaxKind::LeftShift => Punctuator::LeftShift,
        SyntaxKind::RightShift => Punctuator::RightShift,
        SyntaxKind::Less => Punctuator::Less,
        SyntaxKind::Greater => Punctuator::Greater,
        SyntaxKind::LessEqual => Punctuator::LessEqual,
        SyntaxKind::GreaterEqual => Punctuator::GreaterEqual,
        SyntaxKind::Equal => Punctuator::Equal,
        SyntaxKind::NotEqual => Punctuator::NotEqual,
        SyntaxKind::Caret => Punctuator::Caret,
        SyntaxKind::Pipe => Punctuator::Pipe,
        SyntaxKind::LogicalAnd => Punctuator::LogicalAnd,
        SyntaxKind::LogicalOr => Punctuator::LogicalOr,
        SyntaxKind::Question => Punctuator::Question,
        SyntaxKind::Colon => Punctuator::Colon,
        SyntaxKind::Scope => Punctuator::Scope,
        SyntaxKind::Semicolon => Punctuator::Semicolon,
        SyntaxKind::Ellipsis => Punctuator::Ellipsis,
        SyntaxKind::Assign => Punctuator::Assign,
        SyntaxKind::MulAssign => Punctuator::MulAssign,
        SyntaxKind::DivAssign => Punctuator::DivAssign,
        SyntaxKind::ModAssign => Punctuator::ModAssign,
        SyntaxKind::AddAssign => Punctuator::AddAssign,
        SyntaxKind::SubAssign => Punctuator::SubAssign,
        SyntaxKind::LeftShiftAssign => Punctuator::LeftShiftAssign,
        SyntaxKind::RightShiftAssign => Punctuator::RightShiftAssign,
        SyntaxKind::AndAssign => Punctuator::AndAssign,
        SyntaxKind::XorAssign => Punctuator::XorAssign,
        SyntaxKind::OrAssign => Punctuator::OrAssign,
        SyntaxKind::Comma => Punctuator::Comma,
        SyntaxKind::Hash => Punctuator::Hash,
        SyntaxKind::HashHash => Punctuator::HashHash,
        _ => return None,
    })
}

/// Iterator over the children of a [`SyntaxNode`].
pub struct SyntaxChildren<'a> {
    tree: &'a SyntaxTree,
    current: u32,
}

impl<'a> Iterator for SyntaxChildren<'a> {
    type Item = SyntaxNode;

    fn next(&mut self) -> Option<SyntaxNode> {
        if self.current == NONE {
            return None;
        }
        let node = SyntaxNode { idx: self.current };
        self.current = self.tree.data(node).next_sibling;
        Some(node)
    }
}

/// Build the flat node array recursively.
fn build_recursive(
    green: &GreenNode,
    parent: u32,
    prev_sibling: u32,
    offset: u32,
    nodes: &mut Vec<SyntaxNodeData>,
) -> u32 {
    let my_idx = nodes.len() as u32;
    nodes.push(SyntaxNodeData {
        offset,
        parent,
        first_child: NONE,
        next_sibling: NONE,
        green: green.clone(),
    });

    // Link previous sibling to this node
    if prev_sibling != NONE {
        nodes[prev_sibling as usize].next_sibling = my_idx;
    }

    // Build children if internal node
    if let Some(children) = green.children_slice() {
        let mut child_offset = offset;
        let mut prev_child: u32 = NONE;
        let mut first: u32 = NONE;

        for child in children {
            let child_len = child.len();
            let child_idx = build_recursive_green_child(child, my_idx, prev_child, child_offset, nodes);
            if first == NONE {
                first = child_idx;
            }
            prev_child = child_idx;
            child_offset += child_len;
        }

        if first != NONE {
            nodes[my_idx as usize].first_child = first;
        }
    } else if let Some(token) = green.as_token() {
        // Token node: children already handled (there are none)
        // The token's length includes trivia; offset is already set
        let _ = token;
    }

    my_idx
}

fn build_recursive_green_child(
    child: &GreenChild,
    parent: u32,
    prev_sibling: u32,
    offset: u32,
    nodes: &mut Vec<SyntaxNodeData>,
) -> u32 {
    match child {
        GreenChild::Token(token) => {
            let my_idx = nodes.len() as u32;
            let green = GreenNode::token(token.kind, token.len, token.leading_trivia, token.trailing_trivia);
            nodes.push(SyntaxNodeData {
                offset,
                parent,
                first_child: NONE,
                next_sibling: NONE,
                green,
            });
            if prev_sibling != NONE {
                nodes[prev_sibling as usize].next_sibling = my_idx;
            }
            my_idx
        }
        GreenChild::Node(green_node) => build_recursive(green_node, parent, prev_sibling, offset, nodes),
    }
}

// =============================================================================
// TreeVisitor
// =============================================================================

/// A visitor trait for traversing a [`SyntaxTree`].
///
/// The default [`visit`](TreeVisitor::visit) method dispatches on
/// [`SyntaxKind`] and delegates to the appropriate `visit_*` method.
/// Every default `visit_*` method walks all children via [`visit_children`].
///
/// # Identifier hooks
///
/// Override [`visit_ident`](TreeVisitor::visit_ident) to handle identifier
/// tokens. The default dispatches to [`visit_children`].
pub trait TreeVisitor {
    /// The result type produced by visitation.
    type Result;

    /// Visit a node, dispatching on its [`SyntaxKind`].
    fn visit(&mut self, tree: &SyntaxTree, node: SyntaxNode) -> Self::Result {
        let kind = tree.kind(node);
        match kind {
            // Top-level
            SyntaxKind::TranslationUnit => self.visit_translation_unit(tree, node),
            SyntaxKind::ExternalDeclaration => self.visit_external_declaration(tree, node),
            SyntaxKind::FunctionDefinition => self.visit_function_definition(tree, node),

            // Bracket groups
            SyntaxKind::ParenGroup => self.visit_paren_group(tree, node),
            SyntaxKind::BracketGroup => self.visit_bracket_group(tree, node),
            SyntaxKind::BraceGroup => self.visit_brace_group(tree, node),

            // Expressions
            SyntaxKind::Expr => self.visit_expr(tree, node),
            SyntaxKind::ExprError => self.visit_expr_error(tree, node),
            SyntaxKind::ConstantExpr => self.visit_constant_expr(tree, node),
            SyntaxKind::PrimaryExpr => self.visit_primary_expr(tree, node),
            SyntaxKind::GenericSelection => self.visit_generic_selection(tree, node),
            SyntaxKind::GenericAssociationType => self.visit_generic_association_type(tree, node),
            SyntaxKind::GenericAssociationDefault => self.visit_generic_association_default(tree, node),
            SyntaxKind::PostfixExpr => self.visit_postfix_expr(tree, node),
            SyntaxKind::ArrayAccess => self.visit_array_access(tree, node),
            SyntaxKind::FunctionCall => self.visit_function_call(tree, node),
            SyntaxKind::MemberAccess => self.visit_member_access(tree, node),
            SyntaxKind::MemberAccessPtr => self.visit_member_access_ptr(tree, node),
            SyntaxKind::PostIncrement => self.visit_post_increment(tree, node),
            SyntaxKind::PostDecrement => self.visit_post_decrement(tree, node),
            SyntaxKind::CompoundLiteral => self.visit_compound_literal(tree, node),
            SyntaxKind::UnaryExpr => self.visit_unary_expr(tree, node),
            SyntaxKind::PreIncrement => self.visit_pre_increment(tree, node),
            SyntaxKind::PreDecrement => self.visit_pre_decrement(tree, node),
            SyntaxKind::UnaryOp => self.visit_unary_op(tree, node),
            SyntaxKind::SizeofExpr => self.visit_sizeof_expr(tree, node),
            SyntaxKind::SizeofType => self.visit_sizeof_type(tree, node),
            SyntaxKind::Alignof => self.visit_alignof(tree, node),
            SyntaxKind::CastExpr => self.visit_cast_expr(tree, node),
            SyntaxKind::Cast => self.visit_cast(tree, node),
            SyntaxKind::BinaryExpr => self.visit_binary_expr(tree, node),
            SyntaxKind::ConditionalExpr => self.visit_conditional_expr(tree, node),
            SyntaxKind::AssignmentExpr => self.visit_assignment_expr(tree, node),
            SyntaxKind::CommaExpr => self.visit_comma_expr(tree, node),

            // Declarations
            SyntaxKind::Declaration | SyntaxKind::NormalDecl | SyntaxKind::TypedefDecl => {
                self.visit_declaration(tree, node)
            }
            SyntaxKind::StaticAssertDecl => self.visit_static_assert_decl(tree, node),
            SyntaxKind::AttributeDecl => self.visit_attribute_decl(tree, node),
            SyntaxKind::DeclError => self.visit_decl_error(tree, node),
            SyntaxKind::DeclarationSpecifiers => self.visit_declaration_specifiers(tree, node),
            SyntaxKind::DeclarationSpecifier => self.visit_declaration_specifier(tree, node),
            SyntaxKind::InitDeclarator => self.visit_init_declarator(tree, node),
            SyntaxKind::TypeSpecifier => self.visit_type_specifier(tree, node),
            SyntaxKind::StructSpecifier => self.visit_struct_specifier(tree, node),
            SyntaxKind::EnumSpecifier => self.visit_enum_specifier(tree, node),
            SyntaxKind::AtomicTypeSpecifier => self.visit_atomic_type_specifier(tree, node),
            SyntaxKind::TypeofSpecifier => self.visit_typeof_specifier(tree, node),
            SyntaxKind::TypeofSpecifierArg => self.visit_typeof_specifier_arg(tree, node),
            SyntaxKind::TypeQualifier => self.visit_type_qualifier(tree, node),
            SyntaxKind::StorageClassSpecifier => self.visit_storage_class_specifier(tree, node),
            SyntaxKind::FunctionSpecifier => self.visit_function_specifier(tree, node),
            SyntaxKind::AlignmentSpecifier => self.visit_alignment_specifier(tree, node),
            SyntaxKind::TypeSpecifierQualifier => self.visit_type_specifier_qualifier(tree, node),
            SyntaxKind::MemberDeclaration => self.visit_member_declaration(tree, node),
            SyntaxKind::SpecifierQualifierList => self.visit_specifier_qualifier_list(tree, node),
            SyntaxKind::MemberDeclarator => self.visit_member_declarator(tree, node),
            SyntaxKind::MemberBitField => self.visit_member_bit_field(tree, node),
            SyntaxKind::StructOrUnion => self.visit_struct_or_union(tree, node),
            SyntaxKind::Enumerator => self.visit_enumerator(tree, node),
            SyntaxKind::Declarator => self.visit_declarator(tree, node),
            SyntaxKind::DirectDeclarator => self.visit_direct_declarator(tree, node),
            SyntaxKind::DirectDeclaratorIdent => self.visit_direct_declarator_ident(tree, node),
            SyntaxKind::DirectDeclaratorParen => self.visit_direct_declarator_paren(tree, node),
            SyntaxKind::DirectDeclaratorArray => self.visit_direct_declarator_array(tree, node),
            SyntaxKind::DirectDeclaratorFunc => self.visit_direct_declarator_func(tree, node),
            SyntaxKind::Pointer => self.visit_pointer(tree, node),
            SyntaxKind::PointerOrBlock => self.visit_pointer_or_block(tree, node),
            SyntaxKind::ArrayDeclarator => self.visit_array_declarator(tree, node),
            SyntaxKind::ArrayDeclaratorNormal => self.visit_array_declarator_normal(tree, node),
            SyntaxKind::ArrayDeclaratorStatic => self.visit_array_declarator_static(tree, node),
            SyntaxKind::ArrayDeclaratorVLA => self.visit_array_declarator_vla(tree, node),
            SyntaxKind::ParameterTypeList => self.visit_parameter_type_list(tree, node),
            SyntaxKind::ParameterDeclaration => self.visit_parameter_declaration(tree, node),
            SyntaxKind::ParameterDeclarationKind => self.visit_parameter_declaration_kind(tree, node),
            SyntaxKind::TypeName => self.visit_type_name(tree, node),
            SyntaxKind::AbstractDeclarator => self.visit_abstract_declarator(tree, node),
            SyntaxKind::DirectAbstractDeclarator => self.visit_direct_abstract_declarator(tree, node),
            SyntaxKind::DirectAbstractDeclaratorParen => self.visit_direct_abstract_declarator_paren(tree, node),
            SyntaxKind::DirectAbstractDeclaratorArray => self.visit_direct_abstract_declarator_array(tree, node),
            SyntaxKind::DirectAbstractDeclaratorFunc => self.visit_direct_abstract_declarator_func(tree, node),
            SyntaxKind::Initializer => self.visit_initializer(tree, node),
            SyntaxKind::BracedInitializer => self.visit_braced_initializer(tree, node),
            SyntaxKind::DesignatedInitializer => self.visit_designated_initializer(tree, node),
            SyntaxKind::Designation => self.visit_designation(tree, node),
            SyntaxKind::Designator => self.visit_designator(tree, node),
            SyntaxKind::AttributeSpecifier => self.visit_attribute_specifier(tree, node),
            SyntaxKind::Attribute => self.visit_attribute(tree, node),
            SyntaxKind::AttributeToken => self.visit_attribute_token(tree, node),
            SyntaxKind::AsmAttribute => self.visit_asm_attribute(tree, node),

            // Statements
            SyntaxKind::Statement => self.visit_statement(tree, node),
            SyntaxKind::LabeledStatement => self.visit_labeled_statement(tree, node),
            SyntaxKind::UnlabeledStatement => self.visit_unlabeled_statement(tree, node),
            SyntaxKind::Label => self.visit_label(tree, node),
            SyntaxKind::ExpressionStatement => self.visit_expression_statement(tree, node),
            SyntaxKind::PrimaryBlock => self.visit_primary_block(tree, node),
            SyntaxKind::CompoundStatement => self.visit_compound_statement(tree, node),
            SyntaxKind::BlockItem => self.visit_block_item(tree, node),
            SyntaxKind::SelectionStatement => self.visit_selection_statement(tree, node),
            SyntaxKind::IfStatement => self.visit_if_statement(tree, node),
            SyntaxKind::SwitchStatement => self.visit_switch_statement(tree, node),
            SyntaxKind::IterationStatement => self.visit_iteration_statement(tree, node),
            SyntaxKind::WhileStatement => self.visit_while_statement(tree, node),
            SyntaxKind::DoWhileStatement => self.visit_do_while_statement(tree, node),
            SyntaxKind::ForStatement => self.visit_for_statement(tree, node),
            SyntaxKind::ForInit => self.visit_for_init(tree, node),
            SyntaxKind::JumpStatement => self.visit_jump_statement(tree, node),
            SyntaxKind::GotoStatement => self.visit_goto_statement(tree, node),
            SyntaxKind::ContinueStatement => self.visit_continue_statement(tree, node),
            SyntaxKind::BreakStatement => self.visit_break_statement(tree, node),
            SyntaxKind::ReturnStatement => self.visit_return_statement(tree, node),
            SyntaxKind::IterError => self.visit_iter_error(tree, node),
            SyntaxKind::StmtError => self.visit_stmt_error(tree, node),

            // Identifiers and tokens
            SyntaxKind::Ident => self.visit_ident(tree, node),

            // Error
            SyntaxKind::Error => self.visit_error(tree, node),

            // Trivia and tokens — skip (caught by visit_children in parent)
            _ => self.visit_token_default(tree, node),
        }
    }

    /// Visit all children of a node.
    fn visit_children(&mut self, tree: &SyntaxTree, node: SyntaxNode) -> Self::Result {
        for child in tree.children(node) {
            self.visit(tree, child);
        }
        self.default_result()
    }

    /// Default result value.
    fn default_result(&self) -> Self::Result;

    /// Fallback for token kinds not explicitly dispatched.
    fn visit_token_default(&mut self, tree: &SyntaxTree, node: SyntaxNode) -> Self::Result {
        self.visit_children(tree, node)
    }

    // --- Default visit methods ---

    fn visit_translation_unit(&mut self, tree: &SyntaxTree, node: SyntaxNode) -> Self::Result { self.visit_children(tree, node) }
    fn visit_external_declaration(&mut self, tree: &SyntaxTree, node: SyntaxNode) -> Self::Result { self.visit_children(tree, node) }
    fn visit_function_definition(&mut self, tree: &SyntaxTree, node: SyntaxNode) -> Self::Result { self.visit_children(tree, node) }
    fn visit_paren_group(&mut self, tree: &SyntaxTree, node: SyntaxNode) -> Self::Result { self.visit_children(tree, node) }
    fn visit_bracket_group(&mut self, tree: &SyntaxTree, node: SyntaxNode) -> Self::Result { self.visit_children(tree, node) }
    fn visit_brace_group(&mut self, tree: &SyntaxTree, node: SyntaxNode) -> Self::Result { self.visit_children(tree, node) }
    fn visit_expr(&mut self, tree: &SyntaxTree, node: SyntaxNode) -> Self::Result { self.visit_children(tree, node) }
    fn visit_expr_error(&mut self, tree: &SyntaxTree, node: SyntaxNode) -> Self::Result { self.visit_children(tree, node) }
    fn visit_constant_expr(&mut self, tree: &SyntaxTree, node: SyntaxNode) -> Self::Result { self.visit_children(tree, node) }
    fn visit_primary_expr(&mut self, tree: &SyntaxTree, node: SyntaxNode) -> Self::Result { self.visit_children(tree, node) }
    fn visit_generic_selection(&mut self, tree: &SyntaxTree, node: SyntaxNode) -> Self::Result { self.visit_children(tree, node) }
    fn visit_generic_association_type(&mut self, tree: &SyntaxTree, node: SyntaxNode) -> Self::Result { self.visit_children(tree, node) }
    fn visit_generic_association_default(&mut self, tree: &SyntaxTree, node: SyntaxNode) -> Self::Result { self.visit_children(tree, node) }
    fn visit_postfix_expr(&mut self, tree: &SyntaxTree, node: SyntaxNode) -> Self::Result { self.visit_children(tree, node) }
    fn visit_array_access(&mut self, tree: &SyntaxTree, node: SyntaxNode) -> Self::Result { self.visit_children(tree, node) }
    fn visit_function_call(&mut self, tree: &SyntaxTree, node: SyntaxNode) -> Self::Result { self.visit_children(tree, node) }
    fn visit_member_access(&mut self, tree: &SyntaxTree, node: SyntaxNode) -> Self::Result { self.visit_children(tree, node) }
    fn visit_member_access_ptr(&mut self, tree: &SyntaxTree, node: SyntaxNode) -> Self::Result { self.visit_children(tree, node) }
    fn visit_post_increment(&mut self, tree: &SyntaxTree, node: SyntaxNode) -> Self::Result { self.visit_children(tree, node) }
    fn visit_post_decrement(&mut self, tree: &SyntaxTree, node: SyntaxNode) -> Self::Result { self.visit_children(tree, node) }
    fn visit_compound_literal(&mut self, tree: &SyntaxTree, node: SyntaxNode) -> Self::Result { self.visit_children(tree, node) }
    fn visit_unary_expr(&mut self, tree: &SyntaxTree, node: SyntaxNode) -> Self::Result { self.visit_children(tree, node) }
    fn visit_pre_increment(&mut self, tree: &SyntaxTree, node: SyntaxNode) -> Self::Result { self.visit_children(tree, node) }
    fn visit_pre_decrement(&mut self, tree: &SyntaxTree, node: SyntaxNode) -> Self::Result { self.visit_children(tree, node) }
    fn visit_unary_op(&mut self, tree: &SyntaxTree, node: SyntaxNode) -> Self::Result { self.visit_children(tree, node) }
    fn visit_sizeof_expr(&mut self, tree: &SyntaxTree, node: SyntaxNode) -> Self::Result { self.visit_children(tree, node) }
    fn visit_sizeof_type(&mut self, tree: &SyntaxTree, node: SyntaxNode) -> Self::Result { self.visit_children(tree, node) }
    fn visit_alignof(&mut self, tree: &SyntaxTree, node: SyntaxNode) -> Self::Result { self.visit_children(tree, node) }
    fn visit_cast_expr(&mut self, tree: &SyntaxTree, node: SyntaxNode) -> Self::Result { self.visit_children(tree, node) }
    fn visit_cast(&mut self, tree: &SyntaxTree, node: SyntaxNode) -> Self::Result { self.visit_children(tree, node) }
    fn visit_binary_expr(&mut self, tree: &SyntaxTree, node: SyntaxNode) -> Self::Result { self.visit_children(tree, node) }
    fn visit_conditional_expr(&mut self, tree: &SyntaxTree, node: SyntaxNode) -> Self::Result { self.visit_children(tree, node) }
    fn visit_assignment_expr(&mut self, tree: &SyntaxTree, node: SyntaxNode) -> Self::Result { self.visit_children(tree, node) }
    fn visit_comma_expr(&mut self, tree: &SyntaxTree, node: SyntaxNode) -> Self::Result { self.visit_children(tree, node) }
    fn visit_declaration(&mut self, tree: &SyntaxTree, node: SyntaxNode) -> Self::Result { self.visit_children(tree, node) }
    fn visit_static_assert_decl(&mut self, tree: &SyntaxTree, node: SyntaxNode) -> Self::Result { self.visit_children(tree, node) }
    fn visit_attribute_decl(&mut self, tree: &SyntaxTree, node: SyntaxNode) -> Self::Result { self.visit_children(tree, node) }
    fn visit_decl_error(&mut self, tree: &SyntaxTree, node: SyntaxNode) -> Self::Result { self.visit_children(tree, node) }
    fn visit_declaration_specifiers(&mut self, tree: &SyntaxTree, node: SyntaxNode) -> Self::Result { self.visit_children(tree, node) }
    fn visit_declaration_specifier(&mut self, tree: &SyntaxTree, node: SyntaxNode) -> Self::Result { self.visit_children(tree, node) }
    fn visit_init_declarator(&mut self, tree: &SyntaxTree, node: SyntaxNode) -> Self::Result { self.visit_children(tree, node) }
    fn visit_type_specifier(&mut self, tree: &SyntaxTree, node: SyntaxNode) -> Self::Result { self.visit_children(tree, node) }
    fn visit_struct_specifier(&mut self, tree: &SyntaxTree, node: SyntaxNode) -> Self::Result { self.visit_children(tree, node) }
    fn visit_enum_specifier(&mut self, tree: &SyntaxTree, node: SyntaxNode) -> Self::Result { self.visit_children(tree, node) }
    fn visit_atomic_type_specifier(&mut self, tree: &SyntaxTree, node: SyntaxNode) -> Self::Result { self.visit_children(tree, node) }
    fn visit_typeof_specifier(&mut self, tree: &SyntaxTree, node: SyntaxNode) -> Self::Result { self.visit_children(tree, node) }
    fn visit_typeof_specifier_arg(&mut self, tree: &SyntaxTree, node: SyntaxNode) -> Self::Result { self.visit_children(tree, node) }
    fn visit_type_qualifier(&mut self, tree: &SyntaxTree, node: SyntaxNode) -> Self::Result { self.visit_children(tree, node) }
    fn visit_storage_class_specifier(&mut self, tree: &SyntaxTree, node: SyntaxNode) -> Self::Result { self.visit_children(tree, node) }
    fn visit_function_specifier(&mut self, tree: &SyntaxTree, node: SyntaxNode) -> Self::Result { self.visit_children(tree, node) }
    fn visit_alignment_specifier(&mut self, tree: &SyntaxTree, node: SyntaxNode) -> Self::Result { self.visit_children(tree, node) }
    fn visit_type_specifier_qualifier(&mut self, tree: &SyntaxTree, node: SyntaxNode) -> Self::Result { self.visit_children(tree, node) }
    fn visit_member_declaration(&mut self, tree: &SyntaxTree, node: SyntaxNode) -> Self::Result { self.visit_children(tree, node) }
    fn visit_specifier_qualifier_list(&mut self, tree: &SyntaxTree, node: SyntaxNode) -> Self::Result { self.visit_children(tree, node) }
    fn visit_member_declarator(&mut self, tree: &SyntaxTree, node: SyntaxNode) -> Self::Result { self.visit_children(tree, node) }
    fn visit_member_bit_field(&mut self, tree: &SyntaxTree, node: SyntaxNode) -> Self::Result { self.visit_children(tree, node) }
    fn visit_struct_or_union(&mut self, tree: &SyntaxTree, node: SyntaxNode) -> Self::Result { self.visit_children(tree, node) }
    fn visit_enumerator(&mut self, tree: &SyntaxTree, node: SyntaxNode) -> Self::Result { self.visit_children(tree, node) }
    fn visit_declarator(&mut self, tree: &SyntaxTree, node: SyntaxNode) -> Self::Result { self.visit_children(tree, node) }
    fn visit_direct_declarator(&mut self, tree: &SyntaxTree, node: SyntaxNode) -> Self::Result { self.visit_children(tree, node) }
    fn visit_direct_declarator_ident(&mut self, tree: &SyntaxTree, node: SyntaxNode) -> Self::Result { self.visit_children(tree, node) }
    fn visit_direct_declarator_paren(&mut self, tree: &SyntaxTree, node: SyntaxNode) -> Self::Result { self.visit_children(tree, node) }
    fn visit_direct_declarator_array(&mut self, tree: &SyntaxTree, node: SyntaxNode) -> Self::Result { self.visit_children(tree, node) }
    fn visit_direct_declarator_func(&mut self, tree: &SyntaxTree, node: SyntaxNode) -> Self::Result { self.visit_children(tree, node) }
    fn visit_pointer(&mut self, tree: &SyntaxTree, node: SyntaxNode) -> Self::Result { self.visit_children(tree, node) }
    fn visit_pointer_or_block(&mut self, tree: &SyntaxTree, node: SyntaxNode) -> Self::Result { self.visit_children(tree, node) }
    fn visit_array_declarator(&mut self, tree: &SyntaxTree, node: SyntaxNode) -> Self::Result { self.visit_children(tree, node) }
    fn visit_array_declarator_normal(&mut self, tree: &SyntaxTree, node: SyntaxNode) -> Self::Result { self.visit_children(tree, node) }
    fn visit_array_declarator_static(&mut self, tree: &SyntaxTree, node: SyntaxNode) -> Self::Result { self.visit_children(tree, node) }
    fn visit_array_declarator_vla(&mut self, tree: &SyntaxTree, node: SyntaxNode) -> Self::Result { self.visit_children(tree, node) }
    fn visit_parameter_type_list(&mut self, tree: &SyntaxTree, node: SyntaxNode) -> Self::Result { self.visit_children(tree, node) }
    fn visit_parameter_declaration(&mut self, tree: &SyntaxTree, node: SyntaxNode) -> Self::Result { self.visit_children(tree, node) }
    fn visit_parameter_declaration_kind(&mut self, tree: &SyntaxTree, node: SyntaxNode) -> Self::Result { self.visit_children(tree, node) }
    fn visit_type_name(&mut self, tree: &SyntaxTree, node: SyntaxNode) -> Self::Result { self.visit_children(tree, node) }
    fn visit_abstract_declarator(&mut self, tree: &SyntaxTree, node: SyntaxNode) -> Self::Result { self.visit_children(tree, node) }
    fn visit_direct_abstract_declarator(&mut self, tree: &SyntaxTree, node: SyntaxNode) -> Self::Result { self.visit_children(tree, node) }
    fn visit_direct_abstract_declarator_paren(&mut self, tree: &SyntaxTree, node: SyntaxNode) -> Self::Result { self.visit_children(tree, node) }
    fn visit_direct_abstract_declarator_array(&mut self, tree: &SyntaxTree, node: SyntaxNode) -> Self::Result { self.visit_children(tree, node) }
    fn visit_direct_abstract_declarator_func(&mut self, tree: &SyntaxTree, node: SyntaxNode) -> Self::Result { self.visit_children(tree, node) }
    fn visit_initializer(&mut self, tree: &SyntaxTree, node: SyntaxNode) -> Self::Result { self.visit_children(tree, node) }
    fn visit_braced_initializer(&mut self, tree: &SyntaxTree, node: SyntaxNode) -> Self::Result { self.visit_children(tree, node) }
    fn visit_designated_initializer(&mut self, tree: &SyntaxTree, node: SyntaxNode) -> Self::Result { self.visit_children(tree, node) }
    fn visit_designation(&mut self, tree: &SyntaxTree, node: SyntaxNode) -> Self::Result { self.visit_children(tree, node) }
    fn visit_designator(&mut self, tree: &SyntaxTree, node: SyntaxNode) -> Self::Result { self.visit_children(tree, node) }
    fn visit_attribute_specifier(&mut self, tree: &SyntaxTree, node: SyntaxNode) -> Self::Result { self.visit_children(tree, node) }
    fn visit_attribute(&mut self, tree: &SyntaxTree, node: SyntaxNode) -> Self::Result { self.visit_children(tree, node) }
    fn visit_attribute_token(&mut self, tree: &SyntaxTree, node: SyntaxNode) -> Self::Result { self.visit_children(tree, node) }
    fn visit_asm_attribute(&mut self, tree: &SyntaxTree, node: SyntaxNode) -> Self::Result { self.visit_children(tree, node) }
    fn visit_statement(&mut self, tree: &SyntaxTree, node: SyntaxNode) -> Self::Result { self.visit_children(tree, node) }
    fn visit_labeled_statement(&mut self, tree: &SyntaxTree, node: SyntaxNode) -> Self::Result { self.visit_children(tree, node) }
    fn visit_unlabeled_statement(&mut self, tree: &SyntaxTree, node: SyntaxNode) -> Self::Result { self.visit_children(tree, node) }
    fn visit_label(&mut self, tree: &SyntaxTree, node: SyntaxNode) -> Self::Result { self.visit_children(tree, node) }
    fn visit_expression_statement(&mut self, tree: &SyntaxTree, node: SyntaxNode) -> Self::Result { self.visit_children(tree, node) }
    fn visit_primary_block(&mut self, tree: &SyntaxTree, node: SyntaxNode) -> Self::Result { self.visit_children(tree, node) }
    fn visit_compound_statement(&mut self, tree: &SyntaxTree, node: SyntaxNode) -> Self::Result { self.visit_children(tree, node) }
    fn visit_block_item(&mut self, tree: &SyntaxTree, node: SyntaxNode) -> Self::Result { self.visit_children(tree, node) }
    fn visit_selection_statement(&mut self, tree: &SyntaxTree, node: SyntaxNode) -> Self::Result { self.visit_children(tree, node) }
    fn visit_if_statement(&mut self, tree: &SyntaxTree, node: SyntaxNode) -> Self::Result { self.visit_children(tree, node) }
    fn visit_switch_statement(&mut self, tree: &SyntaxTree, node: SyntaxNode) -> Self::Result { self.visit_children(tree, node) }
    fn visit_iteration_statement(&mut self, tree: &SyntaxTree, node: SyntaxNode) -> Self::Result { self.visit_children(tree, node) }
    fn visit_while_statement(&mut self, tree: &SyntaxTree, node: SyntaxNode) -> Self::Result { self.visit_children(tree, node) }
    fn visit_do_while_statement(&mut self, tree: &SyntaxTree, node: SyntaxNode) -> Self::Result { self.visit_children(tree, node) }
    fn visit_for_statement(&mut self, tree: &SyntaxTree, node: SyntaxNode) -> Self::Result { self.visit_children(tree, node) }
    fn visit_for_init(&mut self, tree: &SyntaxTree, node: SyntaxNode) -> Self::Result { self.visit_children(tree, node) }
    fn visit_jump_statement(&mut self, tree: &SyntaxTree, node: SyntaxNode) -> Self::Result { self.visit_children(tree, node) }
    fn visit_goto_statement(&mut self, tree: &SyntaxTree, node: SyntaxNode) -> Self::Result { self.visit_children(tree, node) }
    fn visit_continue_statement(&mut self, tree: &SyntaxTree, node: SyntaxNode) -> Self::Result { self.visit_children(tree, node) }
    fn visit_break_statement(&mut self, tree: &SyntaxTree, node: SyntaxNode) -> Self::Result { self.visit_children(tree, node) }
    fn visit_return_statement(&mut self, tree: &SyntaxTree, node: SyntaxNode) -> Self::Result { self.visit_children(tree, node) }
    fn visit_iter_error(&mut self, tree: &SyntaxTree, node: SyntaxNode) -> Self::Result { self.visit_children(tree, node) }
    fn visit_stmt_error(&mut self, tree: &SyntaxTree, node: SyntaxNode) -> Self::Result { self.visit_children(tree, node) }
    fn visit_ident(&mut self, tree: &SyntaxTree, node: SyntaxNode) -> Self::Result { self.visit_children(tree, node) }
    fn visit_error(&mut self, tree: &SyntaxTree, node: SyntaxNode) -> Self::Result { self.visit_children(tree, node) }
}

// =============================================================================
// Lossless printer
// =============================================================================

/// Print a syntax tree as lossless source text.
///
/// Walks the green tree in pre-order, concatenating leading trivia, token text,
/// and trailing trivia from the source string.
pub fn print_lossless(tree: &SyntaxTree, source: &str) -> String {
    let mut output = String::new();
    print_node_lossless(tree, tree.root(), source, &mut output);
    output
}

fn print_node_lossless(tree: &SyntaxTree, node: SyntaxNode, source: &str, output: &mut String) {
    if let Some(token) = tree.as_token(node) {
        let offset = tree.offset(node) as usize;
        // Leading trivia
        if token.leading_trivia > 0 {
            let trivia_start = offset;
            let trivia_end = trivia_start + token.leading_trivia as usize;
            output.push_str(&source[trivia_start..trivia_end]);
        }
        // Token text
        let token_start = offset + token.leading_trivia as usize;
        let token_end = token_start + token.len as usize;
        output.push_str(&source[token_start..token_end]);
        // Trailing trivia
        if token.trailing_trivia > 0 {
            let trivia_start = token_end;
            let trivia_end = trivia_start + token.trailing_trivia as usize;
            output.push_str(&source[trivia_start..trivia_end]);
        }
    } else {
        for child in tree.children(node) {
            print_node_lossless(tree, child, source, output);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::green::GreenBuilder;

    #[test]
    fn test_syntax_tree_navigation() {
        let mut b = GreenBuilder::new();
        b.start_node(SyntaxKind::TranslationUnit);
        b.start_node(SyntaxKind::BinaryExpr);
        b.token(SyntaxKind::Ident, 1, 0);
        b.token(SyntaxKind::Plus, 1, 1);
        b.token(SyntaxKind::Ident, 1, 2);
        b.finish_node();
        b.finish_node();

        let root = b.build();
        let tree = SyntaxTree::new(root);

        let tu = tree.root();
        assert_eq!(tree.kind(tu), SyntaxKind::TranslationUnit);
        assert_eq!(tree.offset(tu), 0);
        assert_eq!(tree.len(tu), 3);

        let children: Vec<_> = tree.children(tu).collect();
        assert_eq!(children.len(), 1);

        let binary = children[0];
        assert_eq!(tree.kind(binary), SyntaxKind::BinaryExpr);
        assert_eq!(tree.offset(binary), 0);

        let bin_children: Vec<_> = tree.children(binary).collect();
        assert_eq!(bin_children.len(), 3);
        assert_eq!(tree.kind(bin_children[0]), SyntaxKind::Ident);
        assert_eq!(tree.offset(bin_children[0]), 0);
        assert_eq!(tree.kind(bin_children[1]), SyntaxKind::Plus);
        assert_eq!(tree.offset(bin_children[1]), 1);
        assert_eq!(tree.kind(bin_children[2]), SyntaxKind::Ident);
        assert_eq!(tree.offset(bin_children[2]), 2);

        // Parent links
        assert_eq!(tree.parent(binary), Some(tu));
        assert_eq!(tree.parent(bin_children[0]), Some(binary));

        // Sibling links
        assert_eq!(tree.next_sibling(bin_children[0]), Some(bin_children[1]));
        assert_eq!(tree.next_sibling(bin_children[1]), Some(bin_children[2]));
        assert_eq!(tree.next_sibling(bin_children[2]), None);
    }
}
