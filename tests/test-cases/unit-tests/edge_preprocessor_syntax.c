// Preprocessor Directive Syntax (for parser)
#define SIMPLE 42
#define ADD(a, b) ((a) + (b))
#define MAX(a, b) ((a) > (b) ? (a) : (b))

#ifdef FEATURE
int feature_enabled = 1;
#endif

#ifndef DISABLED
int not_disabled = 1;
#endif

#if defined(TEST) && !defined(PRODUCTION)
int test_mode = 1;
#endif

#if __STDC_VERSION__ >= 201112L
int c11_or_later = 1;
#endif

// Nested conditionals
#ifdef OUTER
  #ifdef INNER
    int both_defined = 1;
  #else
    int only_outer = 1;
  #endif
#endif

// elif chains
#if defined(OPTION_A)
  int option_a = 1;
#elif defined(OPTION_B)
  int option_b = 1;
#elif defined(OPTION_C)
  int option_c = 1;
#else
  int no_option = 1;
#endif

// Pragma directives
#pragma once
#pragma pack(push, 1)
#pragma pack(pop)

// Line directives
#line 100
#line 200 "filename.c"

// Include guards pattern
#ifndef HEADER_H
#define HEADER_H
int header_content;
#endif

// Stringification and concatenation
#define STRINGIFY(x) #x
#define CONCAT(a, b) a##b

const char *str = STRINGIFY(hello);
int CONCAT(var, _name) = 42;
