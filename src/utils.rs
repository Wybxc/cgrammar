use std::{
    any::{Any, TypeId},
    cell::{OnceCell, RefCell},
    marker::PhantomData,
    rc::{Rc, Weak},
};

use chumsky::{
    extension::v1::{Ext, ExtParser},
    input::InputRef,
    prelude::*,
};
use rustc_hash::FxHashMap;

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

pub trait Cacher {
    type Parser<'src, T: ContextTweaker> where T: 'static;
    fn make_parser<'src, T: ContextTweaker + 'static>() -> Self::Parser<'src, T>;
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

pub fn cached_recursive<'src, C, T>(cacher: C) -> Ext<Shared<Cached<C::Parser<'src, T>>>>
where
    C: Cacher + 'static,
    T: ContextTweaker + 'static,
{
    thread_local! {
        static CACHE: RefCell<FxHashMap<TypeId, Rc<dyn Any>>> = RefCell::new(FxHashMap::default());
    }

    macro_rules! P {
        ($l:lifetime) => { Cached<C::Parser<$l, T>> };
    }

    let key = cacher.type_id();

    if let Some(parser) = CACHE.with(|cache| {
        let cache = cache.borrow();
        cache.get(&key).and_then(|b| b.clone().downcast::<P!['static]>().ok())
    }) {
        // SAFETY: The parser created by `cacher` is guaranteed to be valid for any lifetime,
        // so we can safely transmute it to the desired lifetime.
        let parser = unsafe { std::mem::transmute::<Rc<P!['static]>, Rc<P!['src]>>(parser) };
        let parser = Rc::downgrade(&parser);
        return Ext(Shared::Weak(parser));
    }

    let parser: Rc<P!['src]> = Rc::new(Cached(OnceCell::new()));
    CACHE.with(|cache| {
        let mut cache = cache.borrow_mut();
        let parser = parser.clone();
        // SAFETY: The parser created by `cacher` is guaranteed to be valid for any lifetime,
        // so we can safely transmute it to the desired lifetime.
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
        $pub:vis fn $name:ident <$a:lifetime , $tn:ident : $tt:tt + $tl:lifetime> () -> impl Parser<$b:lifetime, $input:ty, $output:ty, $extra:ty> + Clone
            $body:expr
    ) => {
        $( #[$attrs] )*
        $pub fn $name<$a, $tn : $tt + $tl>() -> impl Parser<$b, $input, $output, $extra> + Clone {
            struct C;
            impl $crate::utils::Cacher for C {
                type Parser<$b, T: $crate::ContextTweaker + 'static> = ::chumsky::Boxed<$b, $b, $input, $output, $extra>;
                fn make_parser<'src, T: $crate::ContextTweaker + 'static>() -> Self::Parser<'src, T> {
                    ::chumsky::Parser::boxed($body)
                }
            }
            $crate::utils::cached_recursive(C)
        }
    };
}

use crate::ContextTweaker;
pub use crate::cached;
