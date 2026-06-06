//! Dump the AST and green tree of a C source file.
//!
//! Usage: `cargo run --example ast_dump --all-features -- path/to/source.c`

#[cfg(feature = "dbg-pls")]
fn main() {
    use cgrammar::*;

    let file = std::env::args().nth(1).unwrap();
    let src = std::fs::read_to_string(file.as_str()).unwrap();

    let (tree, ctx_map, ast) = parse_tree_with_map(src.as_str());

    println!("=== AST ===");
    println!("{}", dbg_pls::pretty(&ast));

    println!("\n=== Green Tree ===");
    println!("Node count: {}", tree.node_count());
    println!("Root kind: {:?}", tree.kind(tree.root()));

    println!("\n=== Lossless Reconstructed ===");
    println!("{}", print_lossless(&tree, ctx_map.source));
}

#[cfg(not(feature = "dbg-pls"))]
fn main() {
    println!("Please run with --features dbg-pls");
}
