// Tentative Definitions
int tentative1;
int tentative2;
int tentative3;

// Multiple tentative definitions are allowed
extern int external1;
int external1;

extern int external2;
int external2 = 42;

// Tentative with different scopes
int global1;
int global1;  // OK, same scope

// Array tentative definitions
int arr1[];
int arr1[10];

int arr2[];
int arr2[] = {1, 2, 3};

// Incomplete types that will be completed
struct Forward;
extern struct Forward *ptr;

struct Forward {
    int x;
};

// Multiple external declarations
extern void func1(void);
extern void func1(void);
void func1(void) {}
