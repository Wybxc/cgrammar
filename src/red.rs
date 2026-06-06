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
/// traversal. The green tree is kept alive as long as the [`SyntaxTree`]
/// exists.
#[derive(Debug, Clone)]
pub struct SyntaxTree {
    root: GreenNode,
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
        SyntaxTree { root, nodes }
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

    fn data(&self, node: SyntaxNode) -> &SyntaxNodeData {
        &self.nodes[node.idx as usize]
    }
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
        GreenChild::Node(green_node) => {
            build_recursive(green_node, parent, prev_sibling, offset, nodes)
        }
    }
}

// =============================================================================
// TreeVisitor
// =============================================================================

/// A visitor trait for traversing a [`SyntaxTree`].
///
/// The default [`visit`](TreeVisitor::visit) method dispatches on
/// [`SyntaxKind`] and delegates to the appropriate `visit_*` method.
/// The default `visit_*` methods walk all children.
pub trait TreeVisitor {
    /// The result type produced by visitation.
    type Result;

    /// Visit a node, dispatching on its [`SyntaxKind`].
    fn visit(&mut self, tree: &SyntaxTree, node: SyntaxNode) -> Self::Result {
        let kind = tree.kind(node);
        match kind {
            SyntaxKind::TranslationUnit => self.visit_translation_unit(tree, node),
            SyntaxKind::ExternalDeclaration => self.visit_external_declaration(tree, node),
            SyntaxKind::FunctionDefinition => self.visit_function_definition(tree, node),
            SyntaxKind::PrimaryExpr => self.visit_primary_expr(tree, node),
            SyntaxKind::PostfixExpr => self.visit_postfix_expr(tree, node),
            SyntaxKind::UnaryExpr => self.visit_unary_expr(tree, node),
            SyntaxKind::CastExpr => self.visit_cast_expr(tree, node),
            SyntaxKind::BinaryExpr => self.visit_binary_expr(tree, node),
            SyntaxKind::ConditionalExpr => self.visit_conditional_expr(tree, node),
            SyntaxKind::AssignmentExpr => self.visit_assignment_expr(tree, node),
            SyntaxKind::CommaExpr => self.visit_comma_expr(tree, node),
            SyntaxKind::Declaration | SyntaxKind::NormalDecl | SyntaxKind::TypedefDecl => {
                self.visit_declaration(tree, node)
            }
            SyntaxKind::CompoundStatement => self.visit_compound_statement(tree, node),
            SyntaxKind::IfStatement => self.visit_if_statement(tree, node),
            SyntaxKind::WhileStatement => self.visit_while_statement(tree, node),
            SyntaxKind::ForStatement => self.visit_for_statement(tree, node),
            SyntaxKind::ReturnStatement => self.visit_return_statement(tree, node),
            _ => self.visit_default(tree, node),
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

    // Default visit methods
    fn visit_translation_unit(&mut self, tree: &SyntaxTree, node: SyntaxNode) -> Self::Result {
        self.visit_children(tree, node)
    }
    fn visit_external_declaration(&mut self, tree: &SyntaxTree, node: SyntaxNode) -> Self::Result {
        self.visit_children(tree, node)
    }
    fn visit_function_definition(&mut self, tree: &SyntaxTree, node: SyntaxNode) -> Self::Result {
        self.visit_children(tree, node)
    }
    fn visit_primary_expr(&mut self, tree: &SyntaxTree, node: SyntaxNode) -> Self::Result {
        self.visit_children(tree, node)
    }
    fn visit_postfix_expr(&mut self, tree: &SyntaxTree, node: SyntaxNode) -> Self::Result {
        self.visit_children(tree, node)
    }
    fn visit_unary_expr(&mut self, tree: &SyntaxTree, node: SyntaxNode) -> Self::Result {
        self.visit_children(tree, node)
    }
    fn visit_cast_expr(&mut self, tree: &SyntaxTree, node: SyntaxNode) -> Self::Result {
        self.visit_children(tree, node)
    }
    fn visit_binary_expr(&mut self, tree: &SyntaxTree, node: SyntaxNode) -> Self::Result {
        self.visit_children(tree, node)
    }
    fn visit_conditional_expr(&mut self, tree: &SyntaxTree, node: SyntaxNode) -> Self::Result {
        self.visit_children(tree, node)
    }
    fn visit_assignment_expr(&mut self, tree: &SyntaxTree, node: SyntaxNode) -> Self::Result {
        self.visit_children(tree, node)
    }
    fn visit_comma_expr(&mut self, tree: &SyntaxTree, node: SyntaxNode) -> Self::Result {
        self.visit_children(tree, node)
    }
    fn visit_declaration(&mut self, tree: &SyntaxTree, node: SyntaxNode) -> Self::Result {
        self.visit_children(tree, node)
    }
    fn visit_compound_statement(&mut self, tree: &SyntaxTree, node: SyntaxNode) -> Self::Result {
        self.visit_children(tree, node)
    }
    fn visit_if_statement(&mut self, tree: &SyntaxTree, node: SyntaxNode) -> Self::Result {
        self.visit_children(tree, node)
    }
    fn visit_while_statement(&mut self, tree: &SyntaxTree, node: SyntaxNode) -> Self::Result {
        self.visit_children(tree, node)
    }
    fn visit_for_statement(&mut self, tree: &SyntaxTree, node: SyntaxNode) -> Self::Result {
        self.visit_children(tree, node)
    }
    fn visit_return_statement(&mut self, tree: &SyntaxTree, node: SyntaxNode) -> Self::Result {
        self.visit_children(tree, node)
    }
    fn visit_default(&mut self, tree: &SyntaxTree, node: SyntaxNode) -> Self::Result {
        self.visit_children(tree, node)
    }
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
    use crate::green::{GreenBuilder, GreenChild as GC, GreenToken as GT};

    fn token(kind: SyntaxKind, len: u32) -> GC {
        GC::Token(GT { kind, len, leading_trivia: 0, trailing_trivia: 0 })
    }

    #[test]
    fn test_syntax_tree_navigation() {
        let mut b = GreenBuilder::new();
        b.start_node(SyntaxKind::TranslationUnit);
        b.start_node(SyntaxKind::BinaryExpr);
        b.token(SyntaxKind::Ident, 1, 0, 0);
        b.token(SyntaxKind::Plus, 1, 0, 0);
        b.token(SyntaxKind::Ident, 1, 0, 0);
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
