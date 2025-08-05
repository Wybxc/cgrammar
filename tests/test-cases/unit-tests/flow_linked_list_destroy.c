#pragma safety enable


struct item {
    struct item* next;
};
void item_delete(struct item* p);

struct list {
    struct item* head;
    struct item* tail;
};

void list_destroy(struct list*  list)
{
    struct item* p = list->head;
    while (p)
    {
        struct item* next = p->next;
        p->next = 0;
        item_delete(p);
        p = next;
    }
}

int main()
{
    struct list list = { 0 };
    list_destroy(&list);
}
