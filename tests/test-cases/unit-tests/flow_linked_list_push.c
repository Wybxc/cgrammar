#pragma safety enable



#define NULL ((void*)0)

struct item {
  int i;
  struct item * next;
  struct item * previous;
};

struct list
{
    struct item* head;
    struct item* tail;
};
void list_push(struct list* list, struct item* pnew)
{
    if (list->head == NULL)
    {
        list->head = pnew;
        list->tail = pnew;
    }
    else
    {
        assert(list->tail != NULL);
        assert(list->tail->next == NULL);
        pnew->previous = list->tail;
        list->tail->next = pnew;
        list->tail = pnew;
    }

}