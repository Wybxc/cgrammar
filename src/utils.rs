use std::{
    any::{Any, TypeId},
    cell::{OnceCell, RefCell},
    collections::HashMap,
    marker::PhantomData,
    rc::Rc,
};

use chumsky::{
    extension::v1::{Ext, ExtParser},
    input::InputRef,
    prelude::*,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Brand<T, B>(T, PhantomData<B>);

impl<T, B> Brand<T, B> {
    pub fn new(value: T) -> Self {
        Brand(value, PhantomData)
    }

    pub fn into_inner(self) -> T {
        self.0
    }
}

pub trait Cacher {
    type Parser<'src>;
    fn make_parser<'src>() -> Self::Parser<'src>;
}

pub struct Cached<P>(OnceCell<P>);

pub fn cached_recursive<'src, C>(cacher: C) -> Rc<Ext<Cached<C::Parser<'src>>>>
where
    C: Cacher + 'static,
{
    thread_local! {
        static CACHE: RefCell<HashMap<TypeId, Rc<dyn Any>>> = RefCell::new(HashMap::new());
    }

    macro_rules! parser {
        ($l:lifetime) => { Rc<Ext<Cached<C::Parser<$l>>>>};
    }

    // TODO: miri said that there is memory leak

    let key = cacher.type_id();

    if let Some(parser) = CACHE.with(|cache| {
        let cache = cache.borrow();
        cache
            .get(&key)
            .and_then(|b| b.clone().downcast::<Ext<Cached<C::Parser<'static>>>>().ok())
    }) {
        // SAFETY: The parser created by `cacher` is guaranteed to be valid for any lifetime,
        // so we can safely transmute it to the desired lifetime.
        let parser = unsafe { std::mem::transmute::<parser!['static], parser!['src]>(parser) };
        return parser;
    }

    let parser: Rc<Ext<Cached<C::Parser<'src>>>> = Rc::new(Ext(Cached(OnceCell::new())));
    CACHE.with(|cache| {
        let mut cache = cache.borrow_mut();
        let parser = parser.clone();
        // SAFETY: The parser created by `cacher` is guaranteed to be valid for any lifetime,
        // so we can safely transmute it to the desired lifetime.
        let parser = unsafe { std::mem::transmute::<parser!['src], parser!['static]>(parser) };
        cache.insert(key, parser.clone() as Rc<dyn Any>);
    });
    parser
        .0
        .0
        .set(C::make_parser())
        .ok()
        .expect("Parser is already initalized");
    parser
}

impl<'src, P, I, O, E> ExtParser<'src, I, O, E> for Cached<P>
where
    P: Parser<'src, I, O, E>,
    I: Input<'src>,
    E: extra::ParserExtra<'src, I>,
{
    fn parse(&self, inp: &mut InputRef<'src, '_, I, E>) -> Result<O, E::Error> {
        let parser = self.0.get().expect("Parser not initialized");
        inp.parse(parser)
    }

    fn check(&self, inp: &mut InputRef<'src, '_, I, E>) -> Result<(), E::Error> {
        let parser = self.0.get().expect("Parser not initialized");
        inp.check(parser)
    }
}

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
