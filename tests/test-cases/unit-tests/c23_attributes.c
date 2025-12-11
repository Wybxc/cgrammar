// C23 Standard Attributes
[[deprecated]] void old_function(void);

[[deprecated("Use new_function instead")]] 
void another_old_function(void);

[[nodiscard]] int important_function(void);

[[maybe_unused]] static int unused_var = 42;

[[noreturn]] void exit_program(void);

// Attributes on struct members
struct AttrStruct {
    [[deprecated]] int old_field;
    [[maybe_unused]] int optional_field;
};

// Multiple attributes
[[deprecated, nodiscard]] int multi_attr_function(void);

// Attributes on parameters
void func([[maybe_unused]] int param);

// Attributes on types
[[maybe_unused]] typedef int my_int_t;

// Fallthrough attribute in switch
void test_fallthrough(int x) {
    switch(x) {
        case 1:
            x++;
            [[fallthrough]];
        case 2:
            x += 2;
            break;
    }
}
