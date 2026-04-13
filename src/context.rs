use std::{cell::RefCell, rc::Rc};

use chumsky::{
    input::{Checkpoint, Cursor, Input},
    inspector::Inspector,
};
use rustc_hash::FxHashSet;
use snapshottable::{Ref, Snapshot, Store};

use crate::Identifier;

/// Parsing state.
#[derive(Clone)]
pub struct State {
    pub context: Ref<Context>,
    store: Rc<RefCell<Store>>,
}

impl Default for State {
    fn default() -> Self {
        Self::new()
    }
}

impl State {
    /// Create a new parsing state.
    pub fn new() -> Self {
        Self {
            context: Ref::new(Context::default()),
            store: Rc::new(RefCell::new(Store::new())),
        }
    }

    /// Get the current context.
    pub fn ctx(&self) -> Context {
        self.context.get()
    }

    /// Set the current context.
    pub fn set_ctx(&mut self, ctx: Context) {
        self.context.set(&mut self.store.borrow_mut(), ctx);
    }
}

impl<'src, I> Inspector<'src, I> for State
where
    I: Input<'src>,
{
    type Checkpoint = Snapshot;

    fn on_token(&mut self, _token: &I::Token) {}

    fn on_save<'parse>(&self, _cursor: &Cursor<'src, 'parse, I>) -> Self::Checkpoint {
        self.store.borrow_mut().capture()
    }

    fn on_rewind<'parse>(&mut self, marker: &Checkpoint<'src, 'parse, I, Self::Checkpoint>) {
        self.store.borrow_mut().restore(marker.inspector().clone());
    }
}

#[derive(Clone)]
pub struct Context {
    namespaces: Vec<Namespace>,
}

impl Default for Context {
    fn default() -> Self {
        let mut builtin = Namespace::default();
        builtin.add_typedef_name(Identifier::from("__builtin_va_list")); // TODO: va_arg
        builtin.add_typedef_name(Identifier::from("__uint128_t"));
        builtin.add_typedef_name(Identifier::from("_Float16"));
        builtin.add_typedef_name(Identifier::from("_Float128"));
        builtin.add_typedef_name(Identifier::from("_Bool"));

        let namespaces = vec![builtin, Namespace::default()];
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

    pub fn add_typedef_name(&mut self, name: Identifier) {
        self.namespaces
            .last_mut()
            .expect("No namespace to add typedef name")
            .add_typedef_name(name);
    }

    pub fn add_enum_constant(&mut self, name: Identifier) {
        self.namespaces
            .last_mut()
            .expect("No namespace to add enum constant")
            .add_enum_constant(name);
    }

    pub fn push(&mut self) {
        self.namespaces.push(Namespace::default());
    }

    pub fn pop(&mut self) {
        self.namespaces.pop();
    }
}

#[derive(Default, Clone)]
pub struct Namespace {
    typedef_names: Rc<FxHashSet<Identifier>>,
    enum_constants: Rc<FxHashSet<Identifier>>,
}

impl Namespace {
    pub fn is_typedef_name(&self, name: &Identifier) -> bool {
        self.typedef_names.contains(name)
    }

    pub fn is_enum_constant(&self, name: &Identifier) -> bool {
        self.enum_constants.contains(name)
    }

    pub fn add_typedef_name(&mut self, name: Identifier) {
        Rc::make_mut(&mut self.typedef_names).insert(name);
    }

    pub fn add_enum_constant(&mut self, name: Identifier) {
        Rc::make_mut(&mut self.enum_constants).insert(name);
    }
}
