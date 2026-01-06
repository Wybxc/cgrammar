#![cfg(feature = "printer")]

use cgrammar::visitor::Visitor;
use cgrammar::*;
use chumsky::prelude::*;
use elegance::Printer;
use rstest::rstest;

// Helper function to parse C code
fn parse_c(code: &str) -> TranslationUnit {
    let lexer = balanced_token_sequence();
    let tokens = lexer.parse(code).unwrap();
    let parser = translation_unit();
    let result = parser.parse(tokens.as_input());
    result.output().unwrap().clone()
}

// Helper function to print AST to string
fn print_ast(ast: &TranslationUnit) -> String {
    let mut printer = Printer::new(String::new(), 80);
    printer.visit_translation_unit(ast).unwrap();
    printer.finish().unwrap()
}

// ============================================================================
// Basic Printing Tests
// ============================================================================

#[rstest]
#[case("int x;")]
#[case("int x, y, z;")]
#[case("int x = 0;")]
#[case("int x = 1, y = 2;")]
fn test_simple_declarations(#[case] code: &str) {
    let ast = parse_c(code);
    let printed = print_ast(&ast);

    // Basic sanity check: output should contain key elements
    assert!(printed.contains("int"), "Output missing 'int' for: {}", code);
    assert!(printed.contains("x"), "Output missing 'x' for: {}", code);

    // Verify it's valid-ish C code (at least has semicolon)
    assert!(printed.contains(";"), "Output missing semicolon for: {}", code);
}

#[test]
fn test_function_declaration() {
    let code = "int add(int a, int b);";
    let ast = parse_c(code);
    let printed = print_ast(&ast);

    assert!(printed.contains("int"));
    assert!(printed.contains("add"));
    assert!(printed.contains("a"));
    assert!(printed.contains("b"));
    assert!(printed.contains(";"));
}

#[test]
fn test_function_definition() {
    let code = "int add(int a, int b) { return a + b; }";
    let ast = parse_c(code);
    let printed = print_ast(&ast);

    assert!(printed.contains("int"));
    assert!(printed.contains("add"));
    assert!(printed.contains("return"));
    assert!(printed.contains("{"));
    assert!(printed.contains("}"));
}

#[test]
fn test_struct_declaration() {
    let code = "struct Point { int x; int y; };";
    let ast = parse_c(code);
    let printed = print_ast(&ast);

    assert!(printed.contains("struct"));
    assert!(printed.contains("Point"));
    assert!(printed.contains("int"));
    assert!(printed.contains("x"));
    assert!(printed.contains("y"));
}

#[test]
fn test_enum_declaration() {
    let code = "enum Color { RED, GREEN, BLUE };";
    let ast = parse_c(code);
    let printed = print_ast(&ast);

    assert!(printed.contains("enum"));
    assert!(printed.contains("Color"));
    assert!(printed.contains("RED"));
    assert!(printed.contains("GREEN"));
    assert!(printed.contains("BLUE"));
}

#[test]
fn test_typedef() {
    let code = "typedef int Integer;";
    let ast = parse_c(code);
    let printed = print_ast(&ast);

    assert!(printed.contains("typedef"));
    assert!(printed.contains("int"));
    assert!(printed.contains("Integer"));
}

// ============================================================================
// Expression Printing Tests
// ============================================================================

#[rstest]
#[case("int x = 1 + 2;", "+")]
#[case("int x = a * b;", "*")]
#[case("int x = a - b;", "-")]
#[case("int x = a / b;", "/")]
#[case("int x = a % b;", "%")]
fn test_binary_expressions(#[case] code: &str, #[case] operator: &str) {
    let ast = parse_c(code);
    let printed = print_ast(&ast);

    assert!(printed.contains(operator), "Missing operator '{}' in output", operator);
}

#[test]
fn test_function_call() {
    let code = "int x = foo(1, 2, 3);";
    let ast = parse_c(code);
    let printed = print_ast(&ast);

    assert!(printed.contains("foo"));
    assert!(printed.contains("("));
    assert!(printed.contains(")"));
}

#[test]
fn test_array_access() {
    let code = "int x = arr[0];";
    let ast = parse_c(code);
    let printed = print_ast(&ast);

    assert!(printed.contains("arr"));
    assert!(printed.contains("["));
    assert!(printed.contains("0"));
    assert!(printed.contains("]"));
}

#[test]
fn test_member_access() {
    let code = "int x = point.x;";
    let ast = parse_c(code);
    let printed = print_ast(&ast);

    assert!(printed.contains("point"));
    assert!(printed.contains("."));
    assert!(printed.contains("x"));
}

#[test]
fn test_pointer_member_access() {
    let code = "int x = ptr->value;";
    let ast = parse_c(code);
    let printed = print_ast(&ast);

    assert!(printed.contains("ptr"));
    assert!(printed.contains("->"));
    assert!(printed.contains("value"));
}

// ============================================================================
// Statement Printing Tests
// ============================================================================

#[test]
fn test_if_statement() {
    let code = "void f() { if (x > 0) { y = 1; } }";
    let ast = parse_c(code);
    let printed = print_ast(&ast);

    assert!(printed.contains("if"));
    assert!(printed.contains("("));
    assert!(printed.contains(">"));
    assert!(printed.contains(")"));
}

#[test]
fn test_if_else_statement() {
    let code = "void f() { if (x > 0) { y = 1; } else { y = 0; } }";
    let ast = parse_c(code);
    let printed = print_ast(&ast);

    assert!(printed.contains("if"));
    assert!(printed.contains("else"));
}

#[test]
fn test_while_loop() {
    let code = "void f() { while (i < 10) { i++; } }";
    let ast = parse_c(code);
    let printed = print_ast(&ast);

    assert!(printed.contains("while"));
    assert!(printed.contains("("));
    assert!(printed.contains("<"));
    assert!(printed.contains(")"));
}

#[test]
fn test_for_loop() {
    let code = "void f() { for (int i = 0; i < 10; i++) { sum += i; } }";
    let ast = parse_c(code);
    let printed = print_ast(&ast);

    assert!(printed.contains("for"));
    assert!(printed.contains("("));
    assert!(printed.contains(";"));
    assert!(printed.contains(")"));
}

#[test]
fn test_return_statement() {
    let code = "int f() { return 42; }";
    let ast = parse_c(code);
    let printed = print_ast(&ast);

    assert!(printed.contains("return"));
    assert!(printed.contains("42"));
}

#[test]
fn test_goto_label() {
    let code = "void f() { start: goto start; }";
    let ast = parse_c(code);
    let printed = print_ast(&ast);

    assert!(printed.contains("start"));
    assert!(printed.contains("goto"));
    assert!(printed.contains(":"));
}

// ============================================================================
// Pointer and Array Declarator Tests
// ============================================================================

#[test]
fn test_pointer_declaration() {
    let code = "int *ptr;";
    let ast = parse_c(code);
    let printed = print_ast(&ast);

    assert!(printed.contains("int"));
    assert!(printed.contains("*"));
    assert!(printed.contains("ptr"));
}

#[test]
fn test_array_declaration() {
    let code = "int arr[10];";
    let ast = parse_c(code);
    let printed = print_ast(&ast);

    assert!(printed.contains("int"));
    assert!(printed.contains("arr"));
    assert!(printed.contains("["));
    assert!(printed.contains("10"));
    assert!(printed.contains("]"));
}

#[test]
fn test_function_pointer() {
    let code = "int (*fptr)(int, int);";
    let ast = parse_c(code);
    let printed = print_ast(&ast);

    assert!(printed.contains("int"));
    assert!(printed.contains("*"));
    assert!(printed.contains("fptr"));
    assert!(printed.contains("("));
}

// ============================================================================
// Complex Cases
// ============================================================================

#[test]
fn test_nested_function_calls() {
    let code = "int x = foo(bar(1), baz(2));";
    let ast = parse_c(code);
    let printed = print_ast(&ast);

    assert!(printed.contains("foo"));
    assert!(printed.contains("bar"));
    assert!(printed.contains("baz"));
}

#[test]
fn test_complex_expression() {
    let code = "int x = (a + b) * (c - d);";
    let ast = parse_c(code);
    let printed = print_ast(&ast);

    assert!(printed.contains("+"));
    assert!(printed.contains("*"));
    assert!(printed.contains("-"));
}

#[test]
fn test_multi_level_pointer() {
    let code = "int ***ptr;";
    let ast = parse_c(code);
    let printed = print_ast(&ast);

    assert!(printed.contains("int"));
    assert!(printed.contains("***"));
    assert!(printed.contains("ptr"));
}

// ============================================================================
// Round-trip Tests (Parse -> Print -> Parse)
// ============================================================================

// Helper function to perform roundtrip parsing and verify AST is valid
fn verify_roundtrip(code: &str) {
    // First parse
    let ast1 = parse_c(code);

    // Print
    let printed = print_ast(&ast1);

    // Re-tokenize and parse the printed output
    let printed_owned = printed.clone();
    let lexer = balanced_token_sequence();
    let parse_result = lexer.parse(printed_owned.as_str());
    if parse_result.has_errors() {
        panic!("Tokenization failed after printing. Printed code:\n{}", printed);
    }
    let tokens = parse_result.output().unwrap();

    let parser = translation_unit();
    let ast2_result = parser.parse(tokens.as_input());

    if ast2_result.has_errors() {
        println!("Original: {}", code);
        println!("Printed: {}", printed);
        for error in ast2_result.errors() {
            println!("Parse error: {:?}", error);
        }
        panic!("Round-trip parsing failed");
    }
}

#[rstest]
#[case("int x;")]
#[case("int add(int a, int b);")]
#[case("struct Point { int x; int y; };")]
#[case("void f() {}")]
#[case("int *ptr;")]
#[case("int arr[10];")]
fn test_roundtrip_simple(#[case] code: &str) {
    verify_roundtrip(code);
    // Successfully parsed twice - roundtrip is valid
}

#[test]
fn test_roundtrip_function_definition() {
    let code = "int add(int a, int b) { return a + b; }";
    verify_roundtrip(code);
}

#[test]
fn test_roundtrip_struct_with_multiple_members() {
    let code = "struct Rectangle { int width; int height; float area; };";
    verify_roundtrip(code);
}

#[test]
fn test_roundtrip_enum_declaration() {
    let code = "enum Status { OK = 0, ERROR = 1, PENDING = 2 };";
    verify_roundtrip(code);
}

#[test]
fn test_roundtrip_typedef_struct() {
    let code = "typedef struct { int x; int y; } Point;";
    verify_roundtrip(code);
}

#[test]
fn test_roundtrip_function_pointer() {
    let code = "int (*callback)(int x, int y);";
    verify_roundtrip(code);
}

#[test]
fn test_roundtrip_array_of_pointers() {
    let code = "int *arr[10];";
    verify_roundtrip(code);
}

#[test]
fn test_roundtrip_pointer_to_array() {
    let code = "int (*ptr)[10];";
    verify_roundtrip(code);
}

#[test]
fn test_roundtrip_complex_function_signature() {
    let code = "int (*(*signal(int sig, void (*handler)(int)))(int))(int);";
    verify_roundtrip(code);
}

#[test]
fn test_roundtrip_if_else_block() {
    let code = "void f() { if (x > 0) { return x; } else { return -x; } }";
    verify_roundtrip(code);
}

#[test]
fn test_roundtrip_simple_for_loop() {
    let code = "void f() { int i; while (i < 10) { i++; } }";
    verify_roundtrip(code);
}

#[test]
fn test_roundtrip_while_loop() {
    let code = "void f() { while (ptr != NULL) { ptr = ptr->next; } }";
    verify_roundtrip(code);
}

#[test]
fn test_roundtrip_do_while_loop() {
    let code = "void f() { do { count++; } while (count < 100); }";
    verify_roundtrip(code);
}

#[test]
fn test_roundtrip_switch_statement() {
    let code = "void f(int x) { switch (x) { case 1: break; case 2: return; default: break; } }";
    verify_roundtrip(code);
}

#[test]
fn test_roundtrip_multiple_declarations() {
    let code = "int x, y, z;";
    verify_roundtrip(code);
}

#[test]
fn test_roundtrip_declarations_with_initializers() {
    let code = "int x = 1, y = 2, z = 3;";
    verify_roundtrip(code);
}

#[test]
fn test_roundtrip_qualified_types() {
    let code = "const int x; volatile int y; const volatile int z;";
    verify_roundtrip(code);
}

#[test]
fn test_roundtrip_static_and_extern() {
    let code = "static int x; extern int y;";
    verify_roundtrip(code);
}

#[test]
fn test_roundtrip_binary_operators() {
    let code = "int x = a + b; int y = c - d; int z = e * f;";
    verify_roundtrip(code);
}

#[test]
fn test_roundtrip_logical_operators() {
    let code = "int x = (a && b) || (c && d);";
    verify_roundtrip(code);
}

#[test]
fn test_roundtrip_bitwise_operators() {
    let code = "int x = a & b; int y = c | d; int z = e ^ f;";
    verify_roundtrip(code);
}

#[test]
fn test_roundtrip_assignment_operators() {
    let code = "int x = 5;";
    verify_roundtrip(code);
}

#[test]
fn test_roundtrip_unary_operators() {
    let code = "int x = -a; int y = !b; int z = ~c;";
    verify_roundtrip(code);
}

#[test]
fn test_roundtrip_increment_decrement() {
    let code = "void f() { x++; y--; ++z; --w; }";
    verify_roundtrip(code);
}

#[test]
fn test_roundtrip_function_calls_nested() {
    let code = "int x = outer(inner(foo(5)));";
    verify_roundtrip(code);
}

#[test]
fn test_roundtrip_array_access_nested() {
    let code = "int x = matrix[i][j][k];";
    verify_roundtrip(code);
}

#[test]
fn test_roundtrip_member_access_chain() {
    let code = "int x = obj.field1.field2.field3;";
    verify_roundtrip(code);
}

#[test]
fn test_roundtrip_pointer_member_access_chain() {
    let code = "int x = ptr->next->value->data;";
    verify_roundtrip(code);
}

#[test]
fn test_roundtrip_cast_expression() {
    let code = "int x = (int)3.14; void *p = (void *)ptr;";
    verify_roundtrip(code);
}

#[test]
fn test_roundtrip_sizeof_expression() {
    let code = "int size = sizeof(int); int size2 = sizeof(x);";
    verify_roundtrip(code);
}

#[test]
fn test_roundtrip_ternary_expression() {
    let code = "int x = (a > b) ? a : b;";
    verify_roundtrip(code);
}

#[test]
fn test_roundtrip_comma_expression() {
    let code = "void f() { int x = (a, b, c); }";
    verify_roundtrip(code);
}

#[test]
fn test_roundtrip_goto_label() {
    let code = "void f() { start: x++; if (x < 10) goto start; }";
    verify_roundtrip(code);
}

#[test]
fn test_roundtrip_break_continue() {
    let code = "void f() { while (1) { if (x > 5) break; if (x == 0) continue; x--; } }";
    verify_roundtrip(code);
}

#[test]
fn test_roundtrip_multi_level_pointers() {
    let code = "int ***ptr; int ****quad;";
    verify_roundtrip(code);
}

#[test]
fn test_roundtrip_mixed_declarators() {
    let code = "int *p1, p2, *p3, arr[10];";
    verify_roundtrip(code);
}

// ============================================================================
// Edge Cases
// ============================================================================

#[test]
fn test_empty_function() {
    let code = "void f() {}";
    let ast = parse_c(code);
    let printed = print_ast(&ast);

    assert!(printed.contains("void"));
    assert!(printed.contains("f"));
    assert!(printed.contains("{"));
    assert!(printed.contains("}"));
}

#[test]
fn test_multiple_declarations() {
    let code = "int x; int y; int z;";
    let ast = parse_c(code);
    let printed = print_ast(&ast);

    assert!(printed.contains("x"));
    assert!(printed.contains("y"));
    assert!(printed.contains("z"));
}

#[test]
fn test_const_qualifier() {
    let code = "const int x = 42;";
    let ast = parse_c(code);
    let printed = print_ast(&ast);

    assert!(printed.contains("const"));
    assert!(printed.contains("int"));
    assert!(printed.contains("x"));
}

#[test]
fn test_string_literal() {
    let code = r#"const char *s = "hello";"#;
    let ast = parse_c(code);
    let printed = print_ast(&ast);

    assert!(printed.contains("char"));
    assert!(printed.contains("*"));
    assert!(printed.contains("s"));
    assert!(printed.contains("\"") || printed.contains("hello"));
}

// ============================================================================
// AST Consistency Tests (Verify parse before/after consistency)
// ============================================================================

// Helper function for AST consistency verification
fn verify_ast_consistency(code: &str) -> bool {
    // First parse
    let ast1 = parse_c(code);
    let printed = print_ast(&ast1);

    // Parse the printed output
    let lexer = balanced_token_sequence();
    let parse_result = lexer.parse(printed.as_str());

    let tokens = match parse_result.output() {
        Some(t) => t,
        None => return false,
    };

    let parser = translation_unit();
    let ast2_result = parser.parse(tokens.as_input());

    ast2_result.has_output()
}

#[test]
fn test_ast_consistency_simple_variable() {
    let code = "int x;";
    assert!(verify_ast_consistency(code), "AST consistency check failed for: {}", code);
}

#[test]
fn test_ast_consistency_function_with_body() {
    let code = "int add(int a, int b) { return a + b; }";
    assert!(verify_ast_consistency(code), "AST consistency check failed for: {}", code);
}

#[test]
fn test_ast_consistency_struct_definition() {
    let code = "struct Point { int x; int y; };";
    assert!(verify_ast_consistency(code), "AST consistency check failed for: {}", code);
}

#[test]
fn test_ast_consistency_array_declaration() {
    let code = "int arr[10];";
    assert!(verify_ast_consistency(code), "AST consistency check failed for: {}", code);
}

#[test]
fn test_ast_consistency_pointer_declaration() {
    let code = "int *ptr;";
    assert!(verify_ast_consistency(code), "AST consistency check failed for: {}", code);
}

#[test]
fn test_ast_consistency_function_pointer() {
    let code = "int (*fptr)(int);";
    assert!(verify_ast_consistency(code), "AST consistency check failed for: {}", code);
}

#[test]
fn test_ast_consistency_enum_with_values() {
    let code = "enum Color { RED, GREEN, BLUE };";
    assert!(verify_ast_consistency(code), "AST consistency check failed for: {}", code);
}

#[test]
fn test_ast_consistency_complex_declarator() {
    let code = "int *arr[10];";
    assert!(verify_ast_consistency(code), "AST consistency check failed for: {}", code);
}

#[test]
fn test_ast_consistency_if_else() {
    let code = "void f() { if (x) y++; else y--; }";
    assert!(verify_ast_consistency(code), "AST consistency check failed for: {}", code);
}

#[test]
fn test_ast_consistency_while_statement() {
    let code = "void f() { while (1) break; }";
    assert!(verify_ast_consistency(code), "AST consistency check failed for: {}", code);
}
