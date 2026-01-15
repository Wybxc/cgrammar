use std::{
    any::{Any, TypeId},
    cell::{OnceCell, RefCell},
    marker::PhantomData,
    ops::Deref,
    rc::{Rc, Weak},
};

use chumsky::{
    extension::v1::{Ext, ExtParser},
    input::InputRef,
    prelude::*,
};
#[cfg(feature = "dbg-pls")]
use dbg_pls::DebugPls;
use rustc_hash::{FxHashMap, FxHashSet};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Brand<T, B>(T, PhantomData<B>);

impl<T, B> Brand<T, B> {
    pub fn new(value: T) -> Self {
        Brand(value, PhantomData)
    }

    pub fn into_inner(self) -> T {
        self.0
    }

    pub fn inner(&self) -> &T {
        &self.0
    }
}

#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "dbg-pls", derive(DebugPls))]
pub struct StringRef(&'static str);

impl StringRef {
    pub fn new(s: &str) -> Self {
        thread_local! {
            static STRINGS: RefCell<FxHashSet<&'static str>> = RefCell::new(FxHashSet::default());
        }
        STRINGS.with(|strings| {
            let mut strings = strings.borrow_mut();
            if let Some(&s) = strings.get(s) {
                StringRef(s)
            } else {
                let s: &'static str = Box::leak(Box::<str>::from(s));
                strings.insert(s);
                StringRef(s)
            }
        })
    }
}

impl Deref for StringRef {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        self.0
    }
}

pub trait Cacher {
    type Parser<'src>;
    fn make_parser<'src>() -> Self::Parser<'src>;
}

pub struct Cached<P>(OnceCell<P>);

pub enum Shared<T> {
    Strong(Rc<T>),
    Weak(Weak<T>),
}

impl<T> Clone for Shared<T> {
    fn clone(&self) -> Self {
        match self {
            Shared::Strong(p) => Shared::Strong(p.clone()),
            Shared::Weak(weak) => Shared::Weak(weak.clone()),
        }
    }
}

pub fn cached_recursive<'src, C>(cacher: C) -> Ext<Shared<Cached<C::Parser<'src>>>>
where
    C: Cacher + 'static,
{
    thread_local! {
        static CACHE: RefCell<FxHashMap<TypeId, Rc<dyn Any>>> = RefCell::new(FxHashMap::default());
    }

    macro_rules! P {
        ($l:lifetime) => { Cached<C::Parser<$l>> };
    }

    let key = cacher.type_id();

    if let Some(parser) = CACHE.with(|cache| {
        let cache = cache.borrow();
        cache.get(&key).and_then(|b| b.clone().downcast::<P!['static]>().ok())
    }) {
        // SAFETY: The parser created by `cacher` is guaranteed to be valid for any
        // lifetime, so we can safely transmute it to the desired lifetime.
        let parser = unsafe { std::mem::transmute::<Rc<P!['static]>, Rc<P!['src]>>(parser) };
        let parser = Rc::downgrade(&parser);
        return Ext(Shared::Weak(parser));
    }

    let parser: Rc<P!['src]> = Rc::new(Cached(OnceCell::new()));
    CACHE.with(|cache| {
        let mut cache = cache.borrow_mut();
        let parser = parser.clone();
        // SAFETY: The parser created by `cacher` is guaranteed to be valid for any
        // lifetime, so we can safely transmute it to the desired lifetime.
        let parser = unsafe { std::mem::transmute::<Rc<P!['src]>, Rc<P!['static]>>(parser) };
        cache.insert(key, parser as Rc<dyn Any>);
    });
    parser
        .0
        .set(C::make_parser())
        .ok()
        .expect("Parser is already initalized");
    Ext(Shared::Strong(parser))
}

impl<T> Shared<T> {
    fn upgrade(&self) -> Option<Rc<T>> {
        match self {
            Shared::Strong(p) => Some(p.clone()),
            Shared::Weak(weak) => weak.upgrade(),
        }
    }
}

impl<'src, P, I, O, E> ExtParser<'src, I, O, E> for Shared<Cached<P>>
where
    P: Parser<'src, I, O, E>,
    I: Input<'src>,
    E: extra::ParserExtra<'src, I>,
{
    fn parse(&self, inp: &mut InputRef<'src, '_, I, E>) -> Result<O, E::Error> {
        let parser = self.upgrade().expect("Parser dropped");
        let parser = parser.0.get().expect("Parser not initialized");
        inp.parse(parser)
    }

    fn check(&self, inp: &mut InputRef<'src, '_, I, E>) -> Result<(), E::Error> {
        let parser = self.upgrade().expect("Parser dropped");
        let parser = parser.0.get().expect("Parser not initialized");
        inp.check(parser)
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! cached {
    (
        $( #[$attrs:meta] )*
        $pub:vis fn $name:ident <$a:lifetime> () -> impl Parser<$b:lifetime, $input:ty, $output:ty, $extra:ty> + Clone
            $body:expr
    ) => {
        $( #[$attrs] )*
        $pub fn $name<$a>() -> impl Parser<$b, $input, $output, $extra> + Clone {
            struct C;
            impl $crate::utils::Cacher for C {
                type Parser<$b> = ::chumsky::Boxed<$b, $b, $input, $output, $extra>;
                fn make_parser<'src>() -> Self::Parser<'src> {
                    ::chumsky::Parser::boxed($body)
                }
            }
            $crate::utils::cached_recursive(C)
        }
    };
}

pub use crate::cached;
