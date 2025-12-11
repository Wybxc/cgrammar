// Incomplete Types - Forward Declarations
struct ForwardDeclared;

// Pointers to incomplete types are valid
struct ForwardDeclared *ptr;
struct ForwardDeclared **ptr_ptr;

// Function declarations with incomplete types
struct ForwardDeclared *get_forward(void);
void process_forward(struct ForwardDeclared *p);

// Later completion
struct ForwardDeclared {
    int x;
    int y;
};

// Incomplete union
union IncompleteUnion;
union IncompleteUnion *u_ptr;

// Incomplete enum
enum IncompleteEnum;
enum IncompleteEnum *e_ptr;

// Self-referential with incomplete
struct ListNode {
    int data;
    struct ListNode *next;
};

// Mutually referential
struct A;
struct B;

struct A {
    struct B *b_ptr;
};

struct B {
    struct A *a_ptr;
};
