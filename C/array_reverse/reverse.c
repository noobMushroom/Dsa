#include <stdio.h>

void reverse(int *numbers, int start, int end);

int main(void) {
  int numbers[] = {1, 2, 3, 4, 5, 6, 7, 8, 9, 10};
  int i, length, last;

  length = sizeof(numbers) / sizeof(numbers[0]);
  last = length - 1;

  for (i = 0; i < length; i++) {
    printf("%d ", *(numbers + i));
  }

  printf("\n");
  reverse(numbers, 0, last);

  for (i = 0; i < length; i++) {
    printf("%d ", *(numbers + i));
  }
  printf("\n");

  return 0;
}

void reverse(int *numbers, int start, int end) {
  while (start < end) {
    int temp = *(numbers + start);
    *(numbers + start) = *(numbers + end);
    *(numbers + end) = temp;
    start++;
    end--;
  }
}
