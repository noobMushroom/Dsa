#include <stdio.h>
#include <stdlib.h>

typedef struct node {
  int val;
  struct node *next;
} node_t;

void prepend(node_t **head, int value);
void append(node_t **head, int value);
void print_list(node_t *head);
void pop(node_t **head);

void prepend(node_t **head, int value) {
  node_t *new_node = (node_t *)malloc(sizeof(node_t));

  if (new_node == NULL) {
    return;
  }

  new_node->val = value;
  new_node->next = *head;

  *head = new_node;
}

void append(node_t **head, int value) {
  node_t *new_node = (node_t *)malloc(sizeof(node_t));
  if (new_node == NULL) {
    return;
  }

  new_node->val = value;
  new_node->next = NULL;

  if (*head == NULL) {
    *head = new_node;
    return;
  }

  node_t *current = *head;

  while (current->next != NULL) {
    current = current->next;
  }

  current->next = new_node;
}

void pop(node_t **head) {
  if (*head == NULL) {
    return;
  }

  if ((*head)->next == NULL) {
    free(*head);
    *head = NULL;
    return;
  }

  node_t *current = *head;
  node_t *previous = NULL;

  while (current->next != NULL) {
    previous = current;
    current = current->next;
  }

  previous->next = NULL;
  free(current);
}

void print_list(node_t *head) {
  node_t *current = head;
  while (current != NULL) {
    printf("%d -> ", current->val);
    current = current->next;
  }
  printf("NULL\n");
}

int main(void) {
  node_t *head = NULL;
  int i;

  for (i = 0; i <= 100; i += 5) {
    append(&head, i);
  }

  print_list(head);

  pop(&head);

  print_list(head);

  return 0;
}
