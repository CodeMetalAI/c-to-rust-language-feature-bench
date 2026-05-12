/*
 * array_subscripting
 */

#include <stddef.h>
#include <stdio.h>

int main(void) {
  int x[3][5];

  for (int i = 0; i < 3; ++i) {
    for (int j = 0; j < 5; ++j) {
      x[i][j] = 1 * i + j;
    }
  }

  for (int i = 0; i < 3; ++i) {
    for (int j = 0; j < 5; ++j) {
      int a = x[i][j];
      int b = *(*(x + i) + j);
      if (a != b)
        return 1;
    }
  }

  int *p0 = &x[0][0];
  int *p1 = &x[1][0];
  if ((ptrdiff_t)(p1 - p0) != 5)
    return 2;

  printf("PASS\n");
  return 0;
}
