# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

<!-- markdownlint-disable MD024 -->

## [Unreleased]

## [0.3.0](https://github.com/Wybxc/cgrammar/compare/v0.2.3...v0.3.0) - 2025-12-23

### Added

- export the `cached` macro
- accessors for AttributeSpecifier
- add StringLiterials::to_joined
- re-organize the export structure
- expose more parser utilities
- add parameters retrieval methods to Declarator
- disable GNU C features in tests
- upgrade to chumsky 0.12

### Other

- format

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
