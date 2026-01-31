#[cfg(feature = "fuzz")]
fn main() {
    afl::fuzz!(|data: &[u8]| {
        let src = match std::str::from_utf8(data) {
            Ok(s) => s,
            Err(_) => return,
        };
        let _ = cgrammar::lex(src, None);
    });
}

#[cfg(not(feature = "fuzz"))]
fn main() {}
