/*
 * rel_op_c7
 */

int main(void) {
  int x = 42;
  int *p = &x;

  if (p != &x)
    return 1;

  int *p_end1 = p + 1;
  int *p_end2 = &x + 1;

  if (p_end1 != p_end2)
    return 2;

  if (p_end1 == p)
    return 3;

  return 0;
}
