/*
 * add_op_c9
 */

#include <stddef.h>

int main(void) {
  int a[4] = {10, 20, 30, 40};

  int *p = &a[1];
  int *q = &a[3];
  int *one_past = q + 1;

  ptrdiff_t d_pq = p - q;
  if (d_pq != -2)
    return 1;

  ptrdiff_t d_qp = q - p;
  if (d_qp != 2)
    return 2;

  ptrdiff_t lhs = (one_past - p);
  ptrdiff_t rhs1 = (q - p) + 1;
  if (lhs != rhs1)
    return 3;

  ptrdiff_t rhs2 = -(p - one_past);
  if (lhs != rhs2)
    return 4;

  if ((one_past - one_past) != 0)
    return 5;

  return 0;
}
