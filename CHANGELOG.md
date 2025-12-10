# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.2.0](https://github.com/Wybxc/cgrammar/compare/v0.1.0...v0.2.0) - 2025-12-10

### Added

- error recovery in lexer for early EOF
- add support for quoted strings in lexer and parser
- error recovery in declaration
- add release workflow for automated package releases and PR creation

### Fixed

- removed cargo feature in ci

### Other

- add release configuration in release-plz.toml
- update license config in Cargo.toml
- add usage doc for ast_dump
- maunally impl Default for State
- remove optional dependency on ariadne and simplify error reporting
