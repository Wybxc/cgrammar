use std::{collections::hash_map::Entry, fmt, path::PathBuf, rc::Rc};

use crate::{ast::*, span::*};

use ariadne::{Cache, Label, Report, ReportKind, Source};
use chumsky::error::Rich;
use rustc_hash::FxHashMap;

pub type Error<'a> = Rich<'a, BalancedToken, SourceRange>;

#[derive(Clone)]
pub struct Span {
    filename: Option<Rc<str>>,
    start_byte: usize,
    end_byte: usize,
}

impl Span {
    pub fn new(range: SourceRange, cache: &mut FileCache) -> std::io::Result<Self> {
        let filename = range.context.file;

        let line = range.context.line - 1;
        let start_col = range.start - range.context.bol;
        let end_col = range.end - range.context.bol;
        let content = cache.fetch(&filename)?;

        let line = content.line(line).ok_or_else(|| {
            std::io::Error::new(
                std::io::ErrorKind::UnexpectedEof,
                format!("Line {} not found in file {:?}", line, filename),
            )
        })?;
        let start_byte = line.offset() + start_col;
        let end_byte = line.offset() + end_col;

        Ok(Self {
            filename: filename.clone(),
            start_byte,
            end_byte,
        })
    }
}

impl ariadne::Span for Span {
    type SourceId = Option<Rc<str>>;

    fn source(&self) -> &Self::SourceId {
        &self.filename
    }

    fn start(&self) -> usize {
        self.start_byte
    }

    fn end(&self) -> usize {
        self.end_byte
    }
}

pub struct FileCache {
    cache: FxHashMap<PathBuf, Source<Rc<str>>>,
    default: Source<Rc<str>>,
}

impl FileCache {
    pub fn new(default: Rc<str>) -> Self {
        Self {
            cache: FxHashMap::default(),
            default: Source::from(default),
        }
    }
}

impl Cache<Option<Rc<str>>> for FileCache {
    type Storage = Rc<str>;

    #[allow(refining_impl_trait)]
    fn fetch(&mut self, id: &Option<Rc<str>>) -> std::io::Result<&Source<Self::Storage>> {
        let Some(id) = id else {
            return Ok::<_, std::io::Error>(&self.default);
        };
        let id = PathBuf::from(id.as_ref());
        match self.cache.entry(id) {
            Entry::Occupied(entry) => Ok(entry.into_mut()),
            Entry::Vacant(entry) => {
                let file = std::fs::read_to_string(entry.key())?;
                let file: Rc<str> = file.into();
                Ok(entry.insert(Source::from(file)))
            }
        }
    }

    fn display<'a>(&self, id: &'a Option<Rc<str>>) -> Option<impl fmt::Display + 'a> {
        id.as_ref()
    }
}

struct DiagnosticToken(BalancedToken);

impl fmt::Display for DiagnosticToken {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.0 {
            BalancedToken::Parenthesized(_) => write!(f, "(...)"),
            BalancedToken::Bracketed(_) => write!(f, "[...]"),
            BalancedToken::Braced(_) => write!(f, "{{...}}"),
            BalancedToken::Identifier(_) => write!(f, "identifier"),
            BalancedToken::StringLiteral(_) => write!(f, "string literal"),
            BalancedToken::Constant(_) => write!(f, "constant"),
            BalancedToken::Punctuator(_) => write!(f, "punctuator"),
            BalancedToken::Unknown => todo!(),
        }
    }
}

pub fn report<'a>(error: Error<'a>, cache: &mut FileCache) -> std::io::Result<Report<'a, Span>> {
    let error = error.map_token(DiagnosticToken);
    let span = Span::new(error.span().clone(), cache)?;
    let report = Report::build(ReportKind::Error, span.clone())
        .with_label(Label::new(span).with_message(error.reason().to_string()))
        .with_message(format!("syntax error: {}", error.reason()))
        .finish();
    Ok(report)
}
