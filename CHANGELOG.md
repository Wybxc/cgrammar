# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

<!-- markdownlint-disable MD024 -->

## [Unreleased]

## [0.7.1](https://github.com/Wybxc/cgrammar/compare/v0.7.0...v0.7.1) - 2026-01-16

### Other

- Example in README updated.

## [0.7.0](https://github.com/Wybxc/cgrammar/compare/v0.6.0...v0.7.0) - 2026-01-16

### Changed

- **Breaking**: The implementation of the lexer is no longer publicly visible. Instead, a unified `lex` API is provided for lexing.

## [0.6.0](https://github.com/Wybxc/cgrammar/compare/v0.5.1...v0.6.0) - 2026-01-15

### Added

- Printer now respects operator and declarator precedence, generating minimal parentheses in output.
- AST nodes now implement `Send` and `Sync` traits.
- `AsRef<str>` implementation for `Identifier`.

### Changed

- **Breaking**: `Identifier` now uses `Arc<str>` internally instead of `String` for improved memory efficiency and cloning performance.
- **Breaking**: `DeclarationSpecifiers` now has a separate `attributes` field; `FunctionSpecifier` no longer carries attributes inline.
- Upgraded `elegance` dependency to 0.4.0.
- Removed `imbl` dependency.
- Internal parser and lexer state optimizations for better performance.

## [0.5.1](https://github.com/Wybxc/cgrammar/compare/v0.5.0...v0.5.1) - 2026-01-11

### Fixed

- Fixed quasi-quote feature: identifier interpolation now works correctly in declarations (e.g., `quote!{declaration: "int @var;", var => Identifier(...)}`).

## [0.5.0](https://github.com/Wybxc/cgrammar/compare/v0.4.0...v0.5.0) - 2026-01-11

### Added

- `printer` module (behind `printer` feature): Pretty printer for AST nodes via `Visitor` trait implementation for `elegance::Printer`. (Currently not "pretty" but functional yet.)
- `quasi-quote` feature: Token interpolation support with `BalancedToken::Interpolation`, `BalancedTokenSequence::interpolate()`, and `interpolate!` macro.
- `VisitorMut` trait: Mutable visitor pattern for in-place AST modification.
- Re-export `chumsky::Parser` from crate root for convenience.
- Re-export `Visitor` and `VisitorMut` from crate root.
- `Eq` derive for most AST types (`Constant`, `IntegerConstant`, `FloatingConstant`, etc.).
- `From` implementations for convenient AST construction: `i128` → `IntegerConstant`, `f64` → `FloatingConstant`, `char` → `CharacterConstant`, `bool` → `PredefinedConstant`, `String` → `StringLiterals`.

### Changed

- `FloatingConstant::value` changed from `f64` to `ordered_float::NotNan<f64>` (enables `Eq`).
- Visitor walk functions now accept `?Sized` visitors.
- Removed digraph punctuator variants (`LeftBracketAlt`, etc.) from `Punctuator` enum.

## [0.4.0](https://github.com/Wybxc/cgrammar/compare/v0.3.0...v0.4.0) - 2025-12-26

### Added

- Documentation comments for public APIs in `lexer`, `parser`, `span`, and `context` modules.
- `#[warn(missing_docs)]` lint enabled at crate level.

### Changed

- `cached` macro became hidden again.
- `Copy` derive added to small enum/struct types: `IntegerConstant`, `FloatingConstant`, `IntegerSuffix`, `FloatingSuffix`, `EncodingPrefix`, `PredefinedConstant`, `Punctuator`, `UnaryOperator`, `StorageClassSpecifier`, `StructOrUnion`, `TypeQualifier`, `FunctionSpecifier`.

## [0.3.0](https://github.com/Wybxc/cgrammar/compare/v0.2.3...v0.3.0) - 2025-12-23

### Added

- `cached` macro: now publicly exported (no longer `doc(hidden)`), usable directly downstream.
- `StringLiterals::to_joined()`: join multiple string literals into a single `String`.
- `AttributeSpecifier` helpers: `try_into_attributes()` and `try_into_asm()`.
- `Declarator`/`DirectDeclarator` parameter accessors: `parameters()` for retrieving a function's parameter type list.
- Parser utilities are now public: `identifier_or_keyword()`, `identifier()`, `constant()`, `string_literal()`, `quoted_string()`, `keyword()`, `punctuator()`, `no_recover()`, `allow_recover()`, `recover_via_parser()`, `recover_parenthesized()`, `recover_bracketed()`, `expected_found()`.
- `ParserExt` trait is public: provides convenient combinators `parenthesized()`, `bracketed()`, and `braced()`.

### Changed

- Export structure: `lexer` and `parser` are now public modules (`pub mod`). The crate root no longer glob-exports `lexer::*`; instead, it re-exports only `balanced_token_sequence` and `lexer_utils::State` (aliased as `LexerState`). Consumers should use `cgrammar::lexer::...` for lexer symbols.
- Dependency upgrade: `chumsky` to `0.12`.

## [0.2.3](https://github.com/Wybxc/cgrammar/compare/v0.2.2...v0.2.3) - 2025-12-17

### Added

- C-style comment parsing in lexer (single-line `//` and multi-line `/* */`)
- Expose `LexerState` for specifying initial filename in lexing
- Utility methods on `AttributeToken` for checking and extracting attribute types (`is_prefixed()`, `is_standard()`, `as_prefixed()`, `as_standard()`)

## [0.2.2](https://github.com/Wybxc/cgrammar/compare/v0.2.1...v0.2.2) - 2025-12-12

### Added

- Visitor pattern implementation for AST traversal with semantic-aware identifier visiting (variables, types, labels, etc.)

## Changed

- Example demonstrating custom attribute parsing
- Comprehensive test coverage for C99-C23 syntax features ([#5](https://github.com/Wybxc/cgrammar/pull/5))

## [0.2.1](https://github.com/Wybxc/cgrammar/compare/v0.2.0...v0.2.1) - 2025-12-11

### Added

- Export `State` so parsers can be run with caller-provided context (e.g., registering typedef names).

### Changed

- `ast_dump` example now demonstrates `parse_with_state` and seeding typedef names.
- README quickstart uses `CLexer`/`CParser`, aligns dependency snippet with 0.2.x, and notes Apache-2.0 license.
- Upgraded `chumsky` to 0.11.2.

## [0.2.0](https://github.com/Wybxc/cgrammar/compare/v0.1.0...v0.2.0) - 2025-12-10

### Added

- Lexer recovery for early EOF so tokenization continues with diagnostics.
- Support for quoted strings in lexer and parser.
- Parser recovery for declarations, member declarations, and expression statements to keep parsing after errors.
- Usage guide for the `ast_dump` example including error printing.

### Fixed

- `State` implements `Default` again for ergonomic construction.

### Other

- Simplified error reporting by removing the optional `ariadne` dependency.
- Switched crate metadata to Apache-2.0 licensing and added release automation config.

## [0.1.0] - 2025-08-15

### Added

- Initial release of the C23 lexer/parser with AST generation and error reporting helpers (`balanced_token_sequence`, `translation_unit`, `report` utilities).
