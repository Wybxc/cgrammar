# CGrammar

A comprehensive C language grammar parser library written in Rust, implementing the C23 standard (ISO/IEC 9899:2023).

## Language Support

This library supports all major C23 features including:

- **Lexical Elements**: Keywords, identifiers, constants, string literals, and punctuators
- **Expressions**: All expression types from primary expressions to complex compound expressions
- **Declarations**: Variable declarations, function declarations, and type declarations
- **Statements**: Control flow, loops, jumps, and compound statements
- **Functions**: Function definitions with parameter lists and variadic arguments
- **Preprocessor**: Basic preprocessing token support
- **Modern C Features**:
  - Binary constants (`0b` prefix)
  - Digit separators in numeric literals
  - `_BitInt` type specifier
  - `typeof` and `typeof_unqual` operators
  - `_Decimal128`, `_Decimal32`, `_Decimal64` types
  - And many more C23 additions

## Quick Start

Add this to your `Cargo.toml`:

```toml
[dependencies]
cgrammar = "0.2.0"
```

### Basic Usage

```rust
use cgrammar::*;

fn main() {
    let source_code = r#"
        int main() {
            printf("Hello, World!\n");
            return 0;
        }
    "#;

    // Tokenize the source code
    let (tokens, _) = lex(source_code, None);

    // Parse into AST
    let result = translation_unit().parse(tokens.as_input());

    if let Some(ast) = result.output() {
        println!("Successfully parsed!");
        println!("{:#?}", ast);
    } else {
        eprintln!("Parse failed!");
        // Handle errors...
    }
}
```

## License

This project is licensed under the Apache-2.0 License - see the LICENSE file for details.
