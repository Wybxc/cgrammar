use std::rc::Rc;

use chumsky::{
    input::{Checkpoint, Cursor, Input},
    inspector::Inspector,
};
use imbl::{GenericHashSet, shared_ptr::RcK};
use rustc_hash::FxBuildHasher;

use crate::Identifier;

/// Parsing state.
#[derive(Clone)]
pub struct State {
    current: Context,
}

impl Default for State {
    fn default() -> Self {
        Self::new()
    }
}

impl State {
    /// Create a new parsing state.
    pub fn new() -> Self {
        Self { current: Context::default() }
    }

    /// Get a reference to the current context.
    pub fn ctx(&self) -> &Context {
        &self.current
    }

    /// Get a mutable reference to the current context.
    pub fn ctx_mut(&mut self) -> &mut Context {
        &mut self.current
    }
}

impl<'src, I> Inspector<'src, I> for State
where
    I: Input<'src>,
{
    type Checkpoint = Context;

    fn on_token(&mut self, _token: &I::Token) {}

    fn on_save<'parse>(&self, _cursor: &Cursor<'src, 'parse, I>) -> Self::Checkpoint {
        self.current.clone()
    }

    fn on_rewind<'parse>(&mut self, marker: &Checkpoint<'src, 'parse, I, Self::Checkpoint>) {
        self.current = marker.inspector().clone();
    }
}

#[derive(Clone)]
pub struct Context {
    namespaces: Rc<Vec<Namespace>>,
}

impl Default for Context {
    fn default() -> Self {
        let mut builtin = Namespace::default();
        builtin.add_typedef_name(Identifier::from("__builtin_va_list")); // TODO: va_arg
        builtin.add_typedef_name(Identifier::from("__uint128_t"));
        builtin.add_typedef_name(Identifier::from("_Float16"));
        builtin.add_typedef_name(Identifier::from("_Float128"));
        builtin.add_typedef_name(Identifier::from("_Bool"));

        let namespaces = Rc::new(vec![builtin, Namespace::default()]);
        Self { namespaces }
    }
}

impl Context {
    pub fn is_typedef_name(&self, name: &Identifier) -> bool {
        self.namespaces.iter().rev().any(|ns| ns.is_typedef_name(name))
    }

    pub fn is_enum_constant(&self, name: &Identifier) -> bool {
        self.namespaces.iter().rev().any(|ns| ns.is_enum_constant(name))
    }

    fn namespace_mut(&mut self) -> &mut Namespace {
        Rc::make_mut(&mut self.namespaces)
            .last_mut()
            .expect("No namespace to mutate")
    }

    pub fn add_typedef_name(&mut self, name: Identifier) {
        self.namespace_mut().add_typedef_name(name);
    }

    pub fn add_enum_constant(&mut self, name: Identifier) {
        self.namespace_mut().add_enum_constant(name);
    }

    pub fn push(&mut self) {
        Rc::make_mut(&mut self.namespaces).push(Namespace::default());
    }

    pub fn pop(&mut self) {
        Rc::make_mut(&mut self.namespaces).pop();
    }
}

#[derive(Default, Clone)]
pub struct Namespace {
    typedef_names: GenericHashSet<Identifier, FxBuildHasher, RcK>,
    enum_constants: GenericHashSet<Identifier, FxBuildHasher, RcK>,
}

impl Namespace {
    pub fn is_typedef_name(&self, name: &Identifier) -> bool {
        self.typedef_names.contains(name)
    }

    pub fn is_enum_constant(&self, name: &Identifier) -> bool {
        self.enum_constants.contains(name)
    }

    pub fn add_typedef_name(&mut self, name: Identifier) {
        self.typedef_names.insert(name);
    }

    pub fn add_enum_constant(&mut self, name: Identifier) {
        self.enum_constants.insert(name);
    }
}
