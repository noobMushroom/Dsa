#include <stdio.h>

int minimum(int *numbers, int *length);

int main(void) {
  int s[] = {2, 43, 43, 2, 3, 4, 5, 9, 0, 2};
  int length, min;

  length = sizeof(s) / sizeof(s[0]);

  min = minimum(s, &length);

  printf("%d", min);
  return 0;
}

int minimum(int *numbers, int *length) {
  int i, min;
  min = *numbers;

  for (i = 0; i < *length; i++) {
    if (*(numbers + i) < min) {
      min = *(numbers + i);
    }
  }
  return min;
}
