/*
 * size_of_3
 */

#include <stddef.h>

static size_t fsize3(int n) {
  char b[n + 3];
  return sizeof b;
}

int main(void) {
  size_t size = fsize3(10);

  if (size != 13)
    return 1;

  return 0;
}
