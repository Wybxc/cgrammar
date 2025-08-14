use std::cell::RefCell;

use chumsky::{
    input::{Checkpoint, Cursor, Input},
    inspector::Inspector,
};
use imbl::{GenericHashSet, GenericVector, shared_ptr::RcK};
use rustc_hash::FxBuildHasher;
use slab::Slab;

use crate::Identifier;

/// Parsing state.
#[derive(Default)]
pub struct State {
    current: Context,
    checkpoints: RefCell<Slab<Context>>,
}

impl State {
    pub fn new() -> Self {
        Self {
            current: Context::default(),
            checkpoints: RefCell::new(Slab::new()),
        }
    }

    pub fn ctx(&self) -> &Context {
        &self.current
    }

    pub fn ctx_mut(&mut self) -> &mut Context {
        &mut self.current
    }
}

impl<'src, I> Inspector<'src, I> for State
where
    I: Input<'src>,
{
    type Checkpoint = usize;

    fn on_token(&mut self, _token: &I::Token) {}

    fn on_save<'parse>(&self, _cursor: &Cursor<'src, 'parse, I>) -> Self::Checkpoint {
        let mut checkpoints = self.checkpoints.borrow_mut();
        checkpoints.insert(self.current.clone())
    }

    fn on_rewind<'parse>(&mut self, marker: &Checkpoint<'src, 'parse, I, Self::Checkpoint>) {
        let checkpoints = self.checkpoints.borrow();
        let context = checkpoints.get(*marker.inspector()).expect("Invalid checkpoint");
        self.current = context.clone();
    }
}

#[derive(Clone)]
pub struct Context {
    namespaces: GenericVector<Namespace, RcK>,
}

impl Default for Context {
    fn default() -> Self {
        let mut builtin = Namespace::default();
        builtin.add_typedef_name(Identifier::from("__builtin_va_list")); // TODO: va_arg
        builtin.add_typedef_name(Identifier::from("__uint128_t"));
        builtin.add_typedef_name(Identifier::from("_Float16"));
        builtin.add_typedef_name(Identifier::from("_Float128"));
        builtin.add_typedef_name(Identifier::from("_Bool"));

        let mut namespaces = GenericVector::new();
        namespaces.push_back(builtin);
        namespaces.push_back(Namespace::default());

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
        self.namespaces.back_mut().expect("No namespace to mutate")
    }

    pub fn add_typedef_name(&mut self, name: Identifier) {
        self.namespace_mut().add_typedef_name(name);
    }

    pub fn add_enum_constant(&mut self, name: Identifier) {
        self.namespace_mut().add_enum_constant(name);
    }

    pub fn push(&mut self) {
        self.namespaces.push_back(Namespace::default());
    }

    pub fn pop(&mut self) {
        self.namespaces.pop_back();
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
