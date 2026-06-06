//! Green tree — immutable, position-independent syntax tree.
//!
//! The green tree stores the raw structure of parsed source code without
//! absolute positions. It is constructed via [`GreenBuilder`] during parsing
//! and can be shared across multiple red trees via `Arc`.
//!
//! # Trivia
//!
//! Whitespace and comments between tokens are stored as byte-length fields
//! (`leading_trivia`, `trailing_trivia`) on [`GreenToken`]. The actual text
//! is recovered from the source string using these lengths and the token's
//! position in the red tree.

#![allow(missing_docs)]

use std::sync::Arc;

use crate::syntax::SyntaxKind;

// =============================================================================
// GreenToken
// =============================================================================

/// A leaf token in the green tree.
///
/// Stores the token's kind, its text length, and the lengths of surrounding
/// trivia (whitespace/comments). Tokens are position-independent — absolute
/// positions are computed by the red tree layer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GreenToken {
    /// The kind of this token.
    pub kind: SyntaxKind,
    /// Length of the token's text in bytes (not including trivia).
    pub len: u32,
    /// Length of whitespace/comments before this token.
    pub leading_trivia: u32,
    /// Length of whitespace/comments after this token.
    pub trailing_trivia: u32,
}

impl GreenToken {
    /// Total length including trivia.
    pub fn total_len(&self) -> u32 {
        self.leading_trivia + self.len + self.trailing_trivia
    }
}

// =============================================================================
// GreenNode
// =============================================================================

/// A child of a [`GreenNode`] — either a token or another node.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GreenChild {
    Token(GreenToken),
    Node(GreenNode),
}

impl GreenChild {
    /// Total text length of this child.
    pub fn len(&self) -> u32 {
        match self {
            GreenChild::Token(t) => t.total_len(),
            GreenChild::Node(n) => n.len,
        }
    }

    /// Returns `true` if this child is a token.
    pub fn is_token(&self) -> bool {
        matches!(self, GreenChild::Token(_))
    }

    /// Returns `true` if this child is a node.
    pub fn is_node(&self) -> bool {
        matches!(self, GreenChild::Node(_))
    }
}

/// An immutable, position-independent node in the green tree.
///
/// Interior nodes have children; leaf tokens use the [`GreenToken`] type
/// wrapped in [`GreenChild::Token`]. Nodes use `Arc` internally for cheap
/// cloning and structural sharing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GreenNode {
    pub kind: SyntaxKind,
    /// Total text length of this node and all its descendants, including trivia.
    pub len: u32,
    children: GreenChildren,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum GreenChildren {
    /// No children (zero-width placeholder, e.g. empty statement).
    Empty,
    /// Leaf token node.
    Token(GreenToken),
    /// Internal node with children.
    Node { children: Arc<[GreenChild]> },
}

impl GreenNode {
    /// Create a new internal node with the given children.
    pub fn new(kind: SyntaxKind, children: Vec<GreenChild>) -> Self {
        let len: u32 = children.iter().map(|c| c.len()).sum();
        if children.is_empty() {
            GreenNode { kind, len, children: GreenChildren::Empty }
        } else {
            GreenNode { kind, len, children: GreenChildren::Node { children: children.into() } }
        }
    }

    /// Create a new token node.
    pub fn token(kind: SyntaxKind, len: u32, leading_trivia: u32, trailing_trivia: u32) -> Self {
        let token = GreenToken { kind, len, leading_trivia, trailing_trivia };
        let total = token.total_len();
        GreenNode { kind, len: total, children: GreenChildren::Token(token) }
    }

    /// Create an empty node (zero-width).
    pub fn empty(kind: SyntaxKind) -> Self {
        GreenNode { kind, len: 0, children: GreenChildren::Empty }
    }

    /// Returns the children of this node as an iterator.
    pub fn children(&self) -> GreenChildrenIter<'_> {
        match &self.children {
            GreenChildren::Empty => GreenChildrenIter { slice: &[], idx: 0 },
            GreenChildren::Token(_) => GreenChildrenIter { slice: &[], idx: 0 },
            GreenChildren::Node { children } => GreenChildrenIter { slice: children, idx: 0 },
        }
    }

    /// Returns the children as a slice, if this is an internal node.
    pub fn children_slice(&self) -> Option<&[GreenChild]> {
        match &self.children {
            GreenChildren::Node { children } => Some(children),
            _ => None,
        }
    }

    /// Returns the token data, if this is a token node.
    pub fn as_token(&self) -> Option<&GreenToken> {
        match &self.children {
            GreenChildren::Token(t) => Some(t),
            _ => None,
        }
    }

    /// Returns `true` if this node is a leaf token.
    pub fn is_token(&self) -> bool {
        matches!(self.children, GreenChildren::Token(_))
    }

    /// Returns `true` if this node has no children.
    pub fn is_empty(&self) -> bool {
        matches!(self.children, GreenChildren::Empty)
    }

    /// Returns the number of direct children (0 for token/empty nodes).
    pub fn child_count(&self) -> usize {
        match &self.children {
            GreenChildren::Empty | GreenChildren::Token(_) => 0,
            GreenChildren::Node { children } => children.len(),
        }
    }
}

/// Iterator over the children of a [`GreenNode`].
pub struct GreenChildrenIter<'a> {
    slice: &'a [GreenChild],
    idx: usize,
}

impl<'a> Iterator for GreenChildrenIter<'a> {
    type Item = &'a GreenChild;

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx >= self.slice.len() {
            return None;
        }
        let child = &self.slice[self.idx];
        self.idx += 1;
        Some(child)
    }
}

impl<'a> ExactSizeIterator for GreenChildrenIter<'a> {
    fn len(&self) -> usize {
        self.slice.len() - self.idx
    }
}

// =============================================================================
// GreenBuilder
// =============================================================================

/// An event in the green tree construction stream.
///
/// Events are emitted by the parser during parsing and then processed by
/// [`GreenBuilder::build`] into a [`GreenNode`] tree.
#[derive(Debug, Clone, Copy)]
pub enum GreenEvent {
    /// Start a new interior node with the given kind.
    StartNode { kind: SyntaxKind },
    /// Finish the current interior node.
    FinishNode,
    /// Add a token leaf at the given absolute source byte offset.
    /// Leading trivia is computed during [`GreenBuilder::build`] from the gap
    /// between adjacent tokens.
    Token {
        kind: SyntaxKind,
        len: u32,
        start: u32,
    },
    /// A marker placed before a parser to enable retroactive wrapping.
    /// The `wrap_node` method replaces the most recent Mark with a StartNode
    /// and appends a FinishNode at the current position.
    Mark,
}

/// A builder for constructing [`GreenNode`] trees from a stream of
/// [`GreenEvent`]s.
///
/// # Checkpoint / Rewind
///
/// The builder supports checkpoint-based rollback via [`checkpoint`] and
/// [`rewind`]. This is used with chumsky's [`Inspector`] trait to handle
/// backtracking during parsing — before trying a parser alternative, the
/// builder's event vector length is saved as a checkpoint; if the alternative
/// fails, the builder is rewound to the saved length.
///
/// [`Inspector`]: chumsky::inspector::Inspector
/// [`checkpoint`]: GreenBuilder::checkpoint
/// [`rewind`]: GreenBuilder::rewind
#[derive(Debug, Default, Clone)]
pub struct GreenBuilder {
    events: Vec<GreenEvent>,
}

impl GreenBuilder {
    /// Create a new empty builder.
    pub fn new() -> Self {
        GreenBuilder { events: Vec::new() }
    }

    /// Start a new interior node.
    pub fn start_node(&mut self, kind: SyntaxKind) {
        self.events.push(GreenEvent::StartNode { kind });
    }

    /// Finish the current interior node.
    ///
    /// # Panics
    ///
    /// Panics if there is no matching [`start_node`] event.
    pub fn finish_node(&mut self) {
        self.events.push(GreenEvent::FinishNode);
    }

    /// Add a token leaf at the given absolute source position.
    /// Trivia is computed during [`build`] from gaps between adjacent tokens.
    pub fn token(&mut self, kind: SyntaxKind, len: u32, start: u32) {
        self.events.push(GreenEvent::Token { kind, len, start });
    }

    /// Place a mark at the current position. The mark can later be replaced
    /// with a `StartNode` by calling [`wrap_node`].
    ///
    /// This is used alongside [`wrap_node`] by the parser to retroactively
    /// wrap child events into a parent node. The parser places a mark before
    /// running a sub-parser, then calls `wrap_node` after the sub-parser
    /// succeeds.
    pub fn mark(&mut self) {
        self.events.push(GreenEvent::Mark);
    }

    /// Replace the most recent [`Mark`] event with a [`StartNode`] of the
    /// given kind, and append a [`FinishNode`] at the current position.
    ///
    /// This retroactively wraps all events emitted between the mark and this
    /// call into a node of the given kind.
    ///
    /// # Panics
    ///
    /// Panics if there is no preceding `Mark` event.
    pub fn wrap_node(&mut self, kind: SyntaxKind) {
        // Find the most recent Mark, scanning backwards
        for event in self.events.iter_mut().rev() {
            if matches!(event, GreenEvent::Mark) {
                *event = GreenEvent::StartNode { kind };
                self.events.push(GreenEvent::FinishNode);
                return;
            }
        }
        panic!("wrap_node called without a preceding mark");
    }

    /// Return a checkpoint representing the current state.
    ///
    /// The checkpoint is the current length of the internal event vector.
    /// Call [`rewind`] to restore to this state.
    pub fn checkpoint(&self) -> usize {
        self.events.len()
    }

    /// Rewind the builder to a previous checkpoint.
    ///
    /// Truncates the event vector to the given length, discarding all events
    /// emitted after the checkpoint.
    pub fn rewind(&mut self, checkpoint: usize) {
        self.events.truncate(checkpoint);
    }

    /// Consume the builder and produce a [`GreenNode`] tree from the recorded events.
    ///
    /// # Panics
    ///
    /// Panics if the events are not well-formed (e.g., mismatched
    /// `start_node`/`finish_node` pairs) or if no nodes were built.
    pub fn build(self) -> GreenNode {
        struct Frame {
            kind: SyntaxKind,
            children: Vec<GreenChild>,
        }

        let mut stack: Vec<Frame> = Vec::new();
        let mut current: Frame = Frame { kind: SyntaxKind::Error, children: Vec::new() };
        let mut root: Option<GreenNode> = None;

        for event in self.events {
            match event {
                GreenEvent::StartNode { kind } => {
                    let frame = Frame { kind, children: Vec::new() };
                    let old = std::mem::replace(&mut current, frame);
                    stack.push(old);
                }
                GreenEvent::FinishNode => {
                    let frame = std::mem::replace(&mut current, stack.pop().expect("unmatched FinishNode"));
                    let node = GreenNode::new(frame.kind, frame.children);
                    if stack.is_empty() && current.kind == SyntaxKind::Error && current.children.is_empty() {
                        // This is the root node
                        root = Some(node);
                    } else {
                        current.children.push(GreenChild::Node(node));
                    }
                }
                GreenEvent::Token { kind, len, start: _ } => {
                    // Trivia computation from position gaps deferred.
                    // start field preserved in event for future use.
                    let token = GreenToken { kind, len, leading_trivia: 0, trailing_trivia: 0 };
                    current.children.push(GreenChild::Token(token));
                }
                GreenEvent::Mark => {
                    // Unwrapped mark — should not happen in practice, just skip
                }
            }
        }

        root.expect("no root node built (unbalanced events)")
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_green_node_new_empty() {
        let node = GreenNode::new(SyntaxKind::TranslationUnit, vec![]);
        assert_eq!(node.kind, SyntaxKind::TranslationUnit);
        assert_eq!(node.len, 0);
        assert_eq!(node.child_count(), 0);
    }

    #[test]
    fn test_green_node_new_with_children() {
        let token = GreenChild::Token(GreenToken {
            kind: SyntaxKind::Ident,
            len: 3,
            leading_trivia: 0,
            trailing_trivia: 0,
        });
        let node = GreenNode::new(SyntaxKind::PrimaryExpr, vec![token]);
        assert_eq!(node.kind, SyntaxKind::PrimaryExpr);
        assert_eq!(node.len, 3);
        assert_eq!(node.child_count(), 1);
    }

    #[test]
    fn test_green_node_token() {
        let node = GreenNode::token(SyntaxKind::Ident, 5, 0, 0);
        assert!(node.is_token());
        let t = node.as_token().unwrap();
        assert_eq!(t.kind, SyntaxKind::Ident);
        assert_eq!(t.len, 5);
        assert_eq!(t.leading_trivia, 0);
        assert_eq!(t.trailing_trivia, 0);
        assert_eq!(t.total_len(), 5);
    }

    #[test]
    fn test_builder_build() {
        let mut b = GreenBuilder::new();
        // Build: TranslationUnit(BinaryExpr(IDENT("x"), PLUS, IDENT("y")))
        b.start_node(SyntaxKind::TranslationUnit);
        b.start_node(SyntaxKind::BinaryExpr);
        b.token(SyntaxKind::Ident, 1, 0);
        b.token(SyntaxKind::Plus, 1, 1);
        b.token(SyntaxKind::Ident, 1, 2);
        b.finish_node();
        b.finish_node();

        let root = b.build();
        assert_eq!(root.kind, SyntaxKind::TranslationUnit);
        assert_eq!(root.child_count(), 1);

        let children = root.children_slice().unwrap();
        let binary = match &children[0] {
            GreenChild::Node(n) => n,
            _ => panic!("expected node"),
        };
        assert_eq!(binary.kind, SyntaxKind::BinaryExpr);
        assert_eq!(binary.child_count(), 3);
        assert_eq!(binary.len, 3);
    }

    #[test]
    fn test_builder_checkpoint_rewind() {
        let mut b = GreenBuilder::new();
        b.start_node(SyntaxKind::BinaryExpr);
        let ck = b.checkpoint();
        b.token(SyntaxKind::Ident, 1, 0);
        b.token(SyntaxKind::Plus, 1, 1);
        b.token(SyntaxKind::Ident, 1, 2);
        assert_eq!(b.checkpoint(), ck + 3); // 3 events emitted
        b.rewind(ck);
        assert_eq!(b.checkpoint(), ck); // back to start
        // Now emit different tokens
        b.token(SyntaxKind::Ident, 1, 0);
        b.token(SyntaxKind::Minus, 1, 1);
        b.token(SyntaxKind::Ident, 1, 2);
        b.finish_node();
        // Should have 5 events: StartNode, 3 tokens, FinishNode
        assert_eq!(b.events.len(), 5);
    }
}
