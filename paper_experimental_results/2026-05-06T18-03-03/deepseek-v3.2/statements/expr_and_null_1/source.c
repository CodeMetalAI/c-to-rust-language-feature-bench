/* expr_and_null_1.c
 * Test for EXAMPLE 1 from 6.8.3.
 */

#include <stdio.h>

int p(int *t) {
  ++(*t);
  return 0;
}

int main(void) {
  int x = 0;
  (void)p(&x);
  return x == 1 ? 0 : 1;
}
