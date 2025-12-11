// C11 _Noreturn and noreturn
#include <stdnoreturn.h>

_Noreturn void exit_program(int status);
_Noreturn void abort_program(void);

noreturn void terminate(void);

_Noreturn void infinite_loop(void) {
    while (1) {
        // Never returns
    }
}

// Function that calls noreturn function
void cleanup_and_exit(void) {
    // Do cleanup
    exit_program(0);
}

// Noreturn function pointer
_Noreturn void (*exit_func_ptr)(int);

// Noreturn in typedef
typedef _Noreturn void (*noreturn_func_t)(void);
