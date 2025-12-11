// C11 Thread-local Storage
_Thread_local int tls_var;
_Thread_local int tls_init = 42;

struct ThreadData {
    int id;
    char name[32];
};

_Thread_local struct ThreadData thread_data;

void test_thread_local() {
    _Thread_local static int counter = 0;
    _Thread_local int value;
}

// Thread-local with extern
extern _Thread_local int external_tls;

// Thread-local with static
static _Thread_local int static_tls = 100;
