//! Lexer for C source code, producing balanced token sequences (hand-written version).

use ordered_float::NotNan;
use regex_automata::{Anchored, Input, meta::Regex};

#[cfg(feature = "quasi-quote")]
use crate::quasi_quote::Template;
use crate::{
    ast::*,
    span::{ContextId, ContextMapping, SourceContext, Span, Spanned},
};

/// Lexes the input source code into a balanced token sequence.
///
/// This function tokenizes the input string and returns the result along with
/// any errors encountered during lexing.
pub fn lex<'a>(source: &'a str, filename: Option<&str>) -> (BalancedTokenSequence, ContextMapping<'a>) {
    let mut lexer = Lexer::new(source, filename);
    let result = lexer.balanced_token_sequence();
    (result, lexer.ctx_map)
}

trait Pattern {
    fn matches(self, string: &str) -> Option<usize>;
}

impl Pattern for &'_ str {
    fn matches(self, string: &str) -> Option<usize> {
        string.starts_with(self).then_some(self.len())
    }
}

impl Pattern for char {
    fn matches(self, string: &str) -> Option<usize> {
        string.chars().next().filter(|c| *c == self).map(|c| c.len_utf8())
    }
}

impl Pattern for &'_ Regex {
    fn matches(self, string: &str) -> Option<usize> {
        let input = Input::new(string).anchored(Anchored::Yes);
        self.find(input).map(|mat| mat.len())
    }
}

struct Lexer<'a> {
    string: &'a str,
    cursor: usize,
    /// Whether the cursor is at the beginning of a line.
    line_begin: bool,
    /// Current line number.
    lineno: i32,
    /// Current source context.
    ctx_id: ContextId,
    /// Source collection for context tracking.
    ctx_map: ContextMapping<'a>,
}

impl<'a> Lexer<'a> {
    fn new(string: &'a str, filename: Option<&str>) -> Self {
        let mut ctx_map = ContextMapping::new(string);
        Self {
            string,
            cursor: 0,
            line_begin: true,
            lineno: 1,
            ctx_id: filename.map_or(ContextId::none(), |filename| {
                ctx_map.insert_context(SourceContext {
                    filename: filename.into(),
                    line_offset: 0,
                })
            }),
            ctx_map,
        }
    }

    fn remaining(&self) -> &'a str {
        &self.string[self.cursor..]
    }

    fn is_eof(&self) -> bool {
        self.cursor >= self.string.len()
    }

    fn peek(&self) -> Option<char> {
        self.remaining().chars().next()
    }

    fn eat(&mut self) -> Option<char> {
        let ch = self.peek()?;
        self.cursor += ch.len_utf8();
        self.on_token(ch);
        Some(ch)
    }

    fn on_token(&mut self, ch: char) {
        if ch == '\n' {
            self.line_begin = true;
            self.lineno += 1;
        } else if self.line_begin && !ch.is_whitespace() {
            self.line_begin = false;
        }
    }

    fn eat_if(&mut self, pat: impl Pattern) -> Option<&'a str> {
        let remaining = self.remaining();
        if let Some(len) = pat.matches(remaining) {
            let matched = &remaining[..len];
            // Update line tracking for each character
            for ch in matched.chars() {
                self.on_token(ch);
            }
            self.cursor += len;
            Some(matched)
        } else {
            None
        }
    }

    fn make_span(&self, start: usize) -> Span {
        Span::new(start..self.cursor, self.ctx_id)
    }

    /// (6.4.2.1) identifier
    fn identifier(&mut self) -> Option<Identifier> {
        // C identifiers can start with underscore or XID_Start, followed by XID_Continue
        let ident = self.eat_if(re!(r"[_\p{XID_Start}]\p{XID_Continue}*"))?;
        Some(Identifier(ident.into()))
    }

    /// (6.4.4.1) integer constant
    fn integer_constant(&mut self) -> Option<IntegerConstant> {
        // Order matters: hexadecimal and binary must come before octal
        // to avoid matching "0" from "0x" or "0b" as octal
        let value = self
            .hexadecimal_constant()
            .or_else(|| self.binary_constant())
            .or_else(|| self.octal_constant())
            .or_else(|| self.decimal_constant())?;
        let suffix = self.integer_suffix();
        Some(IntegerConstant { value, suffix })
    }

    /// (6.4.4.1) decimal constant
    fn decimal_constant(&mut self) -> Option<i128> {
        let value = self.eat_if(re!(r"[1-9](?:'?[0-9])*"))?;
        let value = value.replace("'", "").parse().unwrap_or(i128::MAX);
        Some(value)
    }

    /// (6.4.4.1) octal constant
    fn octal_constant(&mut self) -> Option<i128> {
        // Try 0o/0O prefix first, then traditional octal (0 followed by octal digits)
        if let Some(value) = self.eat_if(re!(r"0[oO][0-7](?:'?[0-7])*")) {
            let digits = &value[2..];
            return Some(i128::from_str_radix(&digits.replace("'", ""), 8).unwrap_or(i128::MAX));
        }
        if let Some(value) = self.eat_if(re!(r"0(?:'?[0-7])*")) {
            return Some(i128::from_str_radix(&value.replace("'", ""), 8).unwrap_or(i128::MAX));
        }
        None
    }

    /// (6.4.4.1) hexadecimal constant
    fn hexadecimal_constant(&mut self) -> Option<i128> {
        let value = self.eat_if(re!(r"0[xX][0-9a-fA-F](?:'?[0-9a-fA-F])*"))?;
        let value = i128::from_str_radix(&value[2..].replace("'", ""), 16).unwrap_or(i128::MAX);
        Some(value)
    }

    /// (6.4.4.1) binary constant
    fn binary_constant(&mut self) -> Option<i128> {
        let value = self.eat_if(re!(r"0[bB][01](?:'?[01])*"))?;
        let value = i128::from_str_radix(&value[2..].replace("'", ""), 2).unwrap_or(i128::MAX);
        Some(value)
    }

    /// (6.4.4.1) integer suffix
    fn integer_suffix(&mut self) -> Option<IntegerSuffix> {
        if self.eat_if(re!(r"(u|U)(ll|LL)|(ll|LL)(u|U)")).is_some() {
            Some(IntegerSuffix::UnsignedLongLong)
        } else if self.eat_if(re!(r"(u|U)(l|L)|(l|L)(u|U)")).is_some() {
            Some(IntegerSuffix::UnsignedLong)
        } else if self.eat_if(re!(r"(u|U)(wb|WB)|(wb|WB)(u|U)")).is_some() {
            Some(IntegerSuffix::UnsignedBitPrecise)
        } else if self.eat_if(re!(r"u|U")).is_some() {
            Some(IntegerSuffix::Unsigned)
        } else if self.eat_if(re!(r"ll|LL")).is_some() {
            Some(IntegerSuffix::LongLong)
        } else if self.eat_if(re!(r"l|L")).is_some() {
            Some(IntegerSuffix::Long)
        } else if self.eat_if(re!(r"wb|WB")).is_some() {
            Some(IntegerSuffix::BitPrecise)
        } else {
            None
        }
    }

    /// (6.4.4.2) floating constant
    fn floating_constant(&mut self) -> Option<FloatingConstant> {
        // Hexadecimal floating constant must come first to avoid
        // matching "0" from "0x" as decimal
        let value = self
            .hexadecimal_floating_constant()
            .or_else(|| self.decimal_floating_constant())?;
        let suffix = self.floating_suffix();
        Some(FloatingConstant { value, suffix })
    }

    /// (6.4.4.2) decimal floating constant
    fn decimal_floating_constant(&mut self) -> Option<NotNan<f64>> {
        let value = self.eat_if(re!(r"(?:(?:\d+(?:'?\d+)*)?\.(?:\d+(?:'?\d+)*)|(?:\d+(?:'?\d+)*)\.)(?:[eE][+-]?(?:\d+(?:'?\d+)*))?|(?:\d+(?:'?\d+)*)(?:[eE][+-]?(?:\d+(?:'?\d+)*))"))?;
        let value = value.replace("'", "").parse().unwrap();
        Some(value)
    }

    /// (6.4.4.2) hexadecimal floating constant
    fn hexadecimal_floating_constant(&mut self) -> Option<NotNan<f64>> {
        let value = self.eat_if(re!(r"(?:0[xX])(?:(?:[0-9a-fA-F]+(?:'?[0-9a-fA-F]+)*)?\.(?:[0-9a-fA-F]+(?:'?[0-9a-fA-F]+)*)|(?:[0-9a-fA-F]+(?:'?[0-9a-fA-F]+)*)\.?)(?:[pP][+-]?(?:\d+(?:'?\d+)*))"))?;
        let value = hexf_parse::parse_hexf64(&value.replace("'", ""), false).unwrap();
        Some(value.try_into().unwrap())
    }

    /// (6.4.4.2) floating suffix
    fn floating_suffix(&mut self) -> Option<FloatingSuffix> {
        if self.eat_if(re!(r"df|DF")).is_some() {
            Some(FloatingSuffix::DF)
        } else if self.eat_if(re!(r"dd|DD")).is_some() {
            Some(FloatingSuffix::DD)
        } else if self.eat_if(re!(r"dl|DL")).is_some() {
            Some(FloatingSuffix::DL)
        } else if self.eat_if(re!(r"f|F")).is_some() {
            Some(FloatingSuffix::F)
        } else if self.eat_if(re!(r"l|L")).is_some() {
            Some(FloatingSuffix::L)
        } else {
            None
        }
    }

    /// (6.4.4.4) encoding prefix
    fn encoding_prefix(&mut self) -> Option<EncodingPrefix> {
        if self.eat_if("u8").is_some() {
            Some(EncodingPrefix::U8)
        } else if self.eat_if('u').is_some() {
            Some(EncodingPrefix::U)
        } else if self.eat_if('U').is_some() {
            Some(EncodingPrefix::CapitalU)
        } else if self.eat_if('L').is_some() {
            Some(EncodingPrefix::L)
        } else {
            None
        }
    }

    /// (6.4.4.4) escape sequence
    fn escape_sequence(&mut self) -> Option<char> {
        self.eat_if('\\')?;
        match self.peek()? {
            // Simple escape sequences
            '\'' => {
                self.eat();
                Some('\'')
            }
            '"' => {
                self.eat();
                Some('"')
            }
            '?' => {
                self.eat();
                Some('?')
            }
            '\\' => {
                self.eat();
                Some('\\')
            }
            'a' => {
                self.eat();
                Some('\x07')
            }
            'b' => {
                self.eat();
                Some('\x08')
            }
            'f' => {
                self.eat();
                Some('\x0C')
            }
            'n' => {
                self.eat();
                Some('\n')
            }
            'r' => {
                self.eat();
                Some('\r')
            }
            't' => {
                self.eat();
                Some('\t')
            }
            'v' => {
                self.eat();
                Some('\x0B')
            }
            // Octal escape sequence (\ooo)
            '0'..='7' => {
                let digits = self.eat_if(re!(r"[0-7]{1,3}"))?;
                char::from_u32(u32::from_str_radix(digits, 8).ok()?)
            }
            // Hexadecimal escape sequence (\xhh)
            'x' => {
                self.eat();
                let digits = self.eat_if(re!(r"[0-9a-fA-F]+"))?;
                char::from_u32(u32::from_str_radix(digits, 16).ok()?)
            }
            // Universal character names (\uxxxx)
            'u' => {
                self.eat();
                let digits = self.eat_if(re!(r"[0-9a-fA-F]{4}"))?;
                char::from_u32(u32::from_str_radix(digits, 16).ok()?)
            }
            // Universal character names (\Uxxxxxxxx)
            'U' => {
                self.eat();
                let digits = self.eat_if(re!(r"[0-9a-fA-F]{8}"))?;
                char::from_u32(u32::from_str_radix(digits, 16).ok()?)
            }
            // Fallback: just return the character itself
            _ => self.eat(),
        }
    }

    /// (6.4.4.4) character constant
    fn character_constant(&mut self) -> Option<CharacterConstant> {
        let start = self.cursor;
        let encoding_prefix = self.encoding_prefix();

        if self.eat_if('\'').is_none() {
            self.cursor = start;
            return None;
        }

        let mut value = String::new();
        loop {
            match self.peek() {
                Some('\'') => {
                    self.eat();
                    break;
                }
                Some('\\') => {
                    if let Some(ch) = self.escape_sequence() {
                        value.push(ch);
                    }
                }
                Some(ch) if ch != '\n' => {
                    value.push(ch);
                    self.eat();
                }
                _ => break, // EOF or newline - unclosed character constant
            }
        }

        Some(CharacterConstant { encoding_prefix, value })
    }

    /// (6.4.4.5) predefined constant
    fn predefined_constant(&mut self) -> Option<PredefinedConstant> {
        if self.eat_if("false").is_some() {
            // Make sure it's not a prefix of a longer identifier
            if self.peek().is_none_or(|c| !c.is_alphanumeric() && c != '_') {
                return Some(PredefinedConstant::False);
            }
        }
        if self.eat_if("true").is_some() && self.peek().is_none_or(|c| !c.is_alphanumeric() && c != '_') {
            return Some(PredefinedConstant::True);
        }
        if self.eat_if("nullptr").is_some() && self.peek().is_none_or(|c| !c.is_alphanumeric() && c != '_') {
            return Some(PredefinedConstant::Nullptr);
        }
        None
    }

    /// (6.4.4) constant
    fn constant(&mut self) -> Option<Constant> {
        let start = self.cursor;

        // Try predefined constant first
        if let Some(pc) = self.predefined_constant() {
            return Some(Constant::Predefined(pc));
        }
        self.cursor = start;

        // Try floating constant (must be before integer for proper parsing of "0.5")
        if let Some(fc) = self.floating_constant() {
            return Some(Constant::Floating(fc));
        }
        self.cursor = start;

        // Try character constant
        if let Some(cc) = self.character_constant() {
            return Some(Constant::Character(cc));
        }
        self.cursor = start;

        // Try integer constant
        if let Some(ic) = self.integer_constant() {
            return Some(Constant::Integer(ic));
        }
        self.cursor = start;

        None
    }

    /// (6.4.5) string-literal
    fn string_literal(&mut self) -> Option<StringLiterals> {
        let mut literals = Vec::new();

        loop {
            let start = self.cursor;
            let encoding_prefix = self.encoding_prefix();

            if self.eat_if('"').is_none() {
                self.cursor = start;
                break;
            }

            let mut value = String::new();
            loop {
                match self.peek() {
                    Some('"') => {
                        self.eat();
                        break;
                    }
                    Some('\\') => {
                        if let Some(ch) = self.escape_sequence() {
                            value.push(ch);
                        }
                    }
                    Some(ch) if ch != '\n' => {
                        value.push(ch);
                        self.eat();
                    }
                    _ => break, // EOF or newline - unclosed string
                }
            }

            literals.push(StringLiteral { encoding_prefix, value });

            // Skip whitespace between adjacent string literals
            self.skip_whitespace();
        }

        if literals.is_empty() {
            None
        } else {
            Some(StringLiterals(literals))
        }
    }

    /// extension syntax: `xxx` for quoted strings
    fn quoted_string(&mut self) -> Option<String> {
        self.eat_if('`')?;

        let mut content = String::new();
        loop {
            match self.peek() {
                Some('`') => {
                    self.eat();
                    break;
                }
                Some(ch) => {
                    content.push(ch);
                    self.eat();
                }
                None => break, // EOF - unclosed quoted string
            }
        }

        Some(content)
    }

    /// (6.4.6) punctuator (excluding parentheses and brackets)
    fn punctuator(&mut self) -> Option<Punctuator> {
        // Put longer operators first to avoid partial matches
        // Assignment and compound operators (3 chars)
        if self.eat_if("<<=").is_some() {
            return Some(Punctuator::LeftShiftAssign);
        }
        if self.eat_if(">>=").is_some() {
            return Some(Punctuator::RightShiftAssign);
        }
        if self.eat_if("...").is_some() {
            return Some(Punctuator::Ellipsis);
        }

        // Compound operators (2 chars)
        if self.eat_if("*=").is_some() {
            return Some(Punctuator::MulAssign);
        }
        if self.eat_if("/=").is_some() {
            return Some(Punctuator::DivAssign);
        }
        if self.eat_if("%=").is_some() {
            return Some(Punctuator::ModAssign);
        }
        if self.eat_if("+=").is_some() {
            return Some(Punctuator::AddAssign);
        }
        if self.eat_if("-=").is_some() {
            return Some(Punctuator::SubAssign);
        }
        if self.eat_if("&=").is_some() {
            return Some(Punctuator::AndAssign);
        }
        if self.eat_if("^=").is_some() {
            return Some(Punctuator::XorAssign);
        }
        if self.eat_if("|=").is_some() {
            return Some(Punctuator::OrAssign);
        }
        if self.eat_if("##").is_some() {
            return Some(Punctuator::HashHash);
        }
        if self.eat_if("++").is_some() {
            return Some(Punctuator::Increment);
        }
        if self.eat_if("--").is_some() {
            return Some(Punctuator::Decrement);
        }
        if self.eat_if("<<").is_some() {
            return Some(Punctuator::LeftShift);
        }
        if self.eat_if(">>").is_some() {
            return Some(Punctuator::RightShift);
        }
        if self.eat_if("<=").is_some() {
            return Some(Punctuator::LessEqual);
        }
        if self.eat_if(">=").is_some() {
            return Some(Punctuator::GreaterEqual);
        }
        if self.eat_if("==").is_some() {
            return Some(Punctuator::Equal);
        }
        if self.eat_if("!=").is_some() {
            return Some(Punctuator::NotEqual);
        }
        if self.eat_if("&&").is_some() {
            return Some(Punctuator::LogicalAnd);
        }
        if self.eat_if("||").is_some() {
            return Some(Punctuator::LogicalOr);
        }
        if self.eat_if("->").is_some() {
            return Some(Punctuator::Arrow);
        }
        if self.eat_if("::").is_some() {
            return Some(Punctuator::Scope);
        }

        // Simple operators (1 char)
        if self.eat_if('.').is_some() {
            return Some(Punctuator::Dot);
        }
        if self.eat_if('&').is_some() {
            return Some(Punctuator::Ampersand);
        }
        if self.eat_if('*').is_some() {
            return Some(Punctuator::Star);
        }
        if self.eat_if('+').is_some() {
            return Some(Punctuator::Plus);
        }
        if self.eat_if('-').is_some() {
            return Some(Punctuator::Minus);
        }
        if self.eat_if('~').is_some() {
            return Some(Punctuator::Tilde);
        }
        if self.eat_if('!').is_some() {
            return Some(Punctuator::Bang);
        }
        if self.eat_if('/').is_some() {
            return Some(Punctuator::Slash);
        }
        if self.eat_if('%').is_some() {
            return Some(Punctuator::Percent);
        }
        if self.eat_if('<').is_some() {
            return Some(Punctuator::Less);
        }
        if self.eat_if('>').is_some() {
            return Some(Punctuator::Greater);
        }
        if self.eat_if('^').is_some() {
            return Some(Punctuator::Caret);
        }
        if self.eat_if('|').is_some() {
            return Some(Punctuator::Pipe);
        }
        if self.eat_if('?').is_some() {
            return Some(Punctuator::Question);
        }
        if self.eat_if(':').is_some() {
            return Some(Punctuator::Colon);
        }
        if self.eat_if(';').is_some() {
            return Some(Punctuator::Semicolon);
        }
        if self.eat_if('=').is_some() {
            return Some(Punctuator::Assign);
        }
        if self.eat_if(',').is_some() {
            return Some(Punctuator::Comma);
        }
        if self.eat_if('#').is_some() {
            return Some(Punctuator::Hash);
        }

        None
    }

    /// quasi-quote template
    #[cfg(feature = "quasi-quote")]
    fn template(&mut self) -> Option<Template> {
        self.eat_if('@')?;
        let id = self.identifier()?;
        Some(Template { name: id.0 })
    }

    /// Skip single-line comment
    fn skip_line_comment(&mut self) -> bool {
        if self.eat_if("//").is_some() {
            while let Some(ch) = self.peek() {
                if ch == '\n' {
                    break;
                }
                self.eat();
            }
            true
        } else {
            false
        }
    }

    /// Skip multi-line comment
    fn skip_block_comment(&mut self) -> bool {
        if self.eat_if("/*").is_some() {
            loop {
                if self.eat_if("*/").is_some() {
                    break;
                }
                if self.eat().is_none() {
                    break; // EOF
                }
            }
            true
        } else {
            false
        }
    }

    /// Skip line directive (#line, #pragma, etc.)
    fn skip_line_directive(&mut self) -> bool {
        if !self.line_begin {
            return false;
        }

        let start = self.cursor;

        // Skip leading whitespace on the line
        while let Some(ch) = self.peek() {
            if ch == ' ' || ch == '\t' {
                self.eat();
            } else {
                break;
            }
        }

        if self.eat_if('#').is_none() {
            self.cursor = start;
            return false;
        }

        // Check for #pragma
        let is_pragma = self.eat_if("pragma").is_some();

        // Read until end of line
        let mut directive = String::new();
        while let Some(ch) = self.peek() {
            if ch == '\n' {
                self.eat();
                break;
            }
            directive.push(ch);
            self.eat();
        }

        if !is_pragma {
            // Parse #line directive: # <line> "<file>"
            let parts: Vec<&str> = directive.split_whitespace().collect();
            if let [line, file, ..] = &parts[..]
                && let Ok(line_num) = line.parse::<i32>()
            {
                self.ctx_id = self.ctx_map.insert_context(SourceContext {
                    filename: file.trim_matches('"').to_string(),
                    line_offset: self.lineno - line_num,
                });
            }
        }

        true
    }

    /// Skip whitespace, comments, and line directives
    fn skip_whitespace(&mut self) {
        loop {
            let start = self.cursor;

            // Skip whitespace characters
            while let Some(ch) = self.peek() {
                if ch.is_whitespace() {
                    self.eat();
                } else {
                    break;
                }
            }

            // Skip comments
            if self.skip_line_comment() || self.skip_block_comment() {
                continue;
            }

            // Skip line directives
            if self.skip_line_directive() {
                continue;
            }

            if self.cursor == start {
                break;
            }
        }
    }

    /// (6.7.12.1) balanced token
    fn balanced_token(&mut self) -> Option<Spanned<BalancedToken>> {
        self.skip_whitespace();

        let start = self.cursor;

        // Parenthesized: ( balanced-token-sequence? )
        if self.eat_if('(').is_some() {
            let mut inner = self.balanced_token_sequence();
            if self.eat_if(')').is_none() {
                inner.closed = false;
            }
            let span = self.make_span(start);
            return Some(Spanned::new(BalancedToken::Parenthesized(inner), span));
        }

        // Bracketed: [ balanced-token-sequence? ]
        if self.eat_if('[').is_some() {
            let mut inner = self.balanced_token_sequence();
            if self.eat_if(']').is_none() {
                inner.closed = false;
            }
            let span = self.make_span(start);
            return Some(Spanned::new(BalancedToken::Bracketed(inner), span));
        }

        // Braced: { balanced-token-sequence? }
        if self.eat_if('{').is_some() {
            let mut inner = self.balanced_token_sequence();
            if self.eat_if('}').is_none() {
                inner.closed = false;
            }
            let span = self.make_span(start);
            return Some(Spanned::new(BalancedToken::Braced(inner), span));
        }

        // String literal
        if let Some(sl) = self.string_literal() {
            let span = self.make_span(start);
            return Some(Spanned::new(BalancedToken::StringLiteral(sl), span));
        }

        // Quoted string (backtick)
        if let Some(qs) = self.quoted_string() {
            let span = self.make_span(start);
            return Some(Spanned::new(BalancedToken::QuotedString(qs), span));
        }

        // Template (quasi-quote)
        #[cfg(feature = "quasi-quote")]
        if let Some(t) = self.template() {
            let span = self.make_span(start);
            return Some(Spanned::new(BalancedToken::Template(t), span));
        }

        // Constant (must try before identifier for true/false/nullptr)
        if let Some(c) = self.constant() {
            let span = self.make_span(start);
            return Some(Spanned::new(BalancedToken::Constant(c), span));
        }

        // Identifier
        if let Some(id) = self.identifier() {
            let span = self.make_span(start);
            return Some(Spanned::new(BalancedToken::Identifier(id), span));
        }

        // Punctuator
        if let Some(p) = self.punctuator() {
            let span = self.make_span(start);
            return Some(Spanned::new(BalancedToken::Punctuator(p), span));
        }

        // Unknown token - any single character that doesn't match anything
        if !self.is_eof() && self.peek().is_some_and(|c| !c.is_whitespace()) {
            self.eat();
            let span = self.make_span(start);
            return Some(Spanned::new(BalancedToken::Unknown, span));
        }

        None
    }

    /// (6.7.12.1) balanced token sequence
    fn balanced_token_sequence(&mut self) -> BalancedTokenSequence {
        let mut tokens = Vec::new();

        loop {
            self.skip_whitespace();

            // Check for closing brackets or EOF
            match self.peek() {
                Some(')') | Some(']') | Some('}') | None => break,
                _ => {}
            }

            if let Some(token) = self.balanced_token() {
                tokens.push(token);
            } else {
                break;
            }
        }

        self.skip_whitespace();
        let eoi = Span::new_eoi(self.cursor, self.ctx_id);

        BalancedTokenSequence { tokens, closed: true, eoi }
    }
}
