use std::cell::{Ref, RefCell, RefMut};

use chumsky::{
    input::{Checkpoint, Cursor, Input},
    inspector::Inspector,
};
use imbl::{GenericHashSet, shared_ptr::RcK};
use rustc_hash::FxBuildHasher;
use slab::Slab;

use crate::Identifier;

/// Parsing state.
#[derive(Clone)]
pub struct State {
    current: usize,
    contexts: RefCell<Slab<Context>>,
}

impl Default for State {
    fn default() -> Self {
        Self::new()
    }
}

impl State {
    /// Create a new parsing state.
    pub fn new() -> Self {
        let mut contexts = Slab::new();
        let current = contexts.insert(Context::default());
        let contexts = RefCell::new(contexts);
        Self { current, contexts }
    }

    /// Get a reference to the current context.
    pub fn ctx(&self) -> ContextRef<'_> {
        ContextRef {
            handle: self.current,
            contexts: &self.contexts,
        }
    }

    /// Get a mutable reference to the current context.
    pub fn ctx_mut(&mut self) -> ContextRefMut<'_> {
        ContextRefMut {
            handle: &mut self.current,
            contexts: &self.contexts,
        }
    }
}

impl<'src, I> Inspector<'src, I> for State
where
    I: Input<'src>,
{
    type Checkpoint = usize;

    fn on_token(&mut self, _token: &I::Token) {}

    fn on_save<'parse>(&self, _cursor: &Cursor<'src, 'parse, I>) -> Self::Checkpoint {
        self.current.clone()
    }

    fn on_rewind<'parse>(&mut self, marker: &Checkpoint<'src, 'parse, I, Self::Checkpoint>) {
        self.current = *marker.inspector();
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

#[derive(Clone, Copy)]
pub struct ContextRef<'a> {
    handle: usize,
    contexts: &'a RefCell<Slab<Context>>,
}

impl ContextRef<'_> {
    fn context(&self) -> Ref<'_, Context> {
        Ref::map(self.contexts.borrow(), |contexts| &contexts[self.handle])
    }

    fn namespaces(&self) -> Ref<'_, Vec<Namespace>> {
        Ref::map(self.context(), |context| &context.namespaces)
    }

    pub fn is_typedef_name(&self, name: &Identifier) -> bool {
        self.namespaces().iter().rev().any(|ns| ns.is_typedef_name(name))
    }

    pub fn is_enum_constant(&self, name: &Identifier) -> bool {
        self.namespaces().iter().rev().any(|ns| ns.is_enum_constant(name))
    }
}

pub struct ContextRefMut<'a> {
    handle: &'a mut usize,
    contexts: &'a RefCell<Slab<Context>>,
}

impl ContextRefMut<'_> {
    fn context(&self) -> Ref<'_, Context> {
        Ref::map(self.contexts.borrow(), |contexts| &contexts[*self.handle])
    }

    /// Clone the underlying `Context` to get a mutable copy.
    /// Kinda like `Rc::make_mut`, but clones every time.
    fn context_mut(&mut self) -> RefMut<'_, Context> {
        let context = self.context().clone();
        let mut contexts = self.contexts.borrow_mut();
        *self.handle = contexts.insert(context);
        RefMut::map(contexts, |contexts| &mut contexts[*self.handle])
    }

    fn namespaces_mut(&mut self) -> RefMut<'_, Vec<Namespace>> {
        RefMut::map(self.context_mut(), |context| &mut context.namespaces)
    }

    fn namespace_mut(&mut self) -> RefMut<'_, Namespace> {
        RefMut::map(self.namespaces_mut(), |namespaces| {
            namespaces.last_mut().expect("No namespace to mutate")
        })
    }

    pub fn add_typedef_name(&mut self, name: Identifier) {
        self.namespace_mut().add_typedef_name(name);
    }

    pub fn add_enum_constant(&mut self, name: Identifier) {
        self.namespace_mut().add_enum_constant(name);
    }

    pub fn push(&mut self) {
        self.namespaces_mut().push(Namespace::default());
    }

    pub fn pop(&mut self) {
        self.namespaces_mut().pop();
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
