/*
 * size_of_1
 */

#include <stddef.h>
#include <stdlib.h>

static void *alloc(size_t n) { return malloc(n); }

int main(void) {
  double *dp = (double *)alloc(sizeof *dp);

  if (!dp)
    return 1;

  *dp = 3.14;
  if (*dp != 3.14) {
    free(dp);
    return 2;
  }

  free(dp);
  return 0;
}
