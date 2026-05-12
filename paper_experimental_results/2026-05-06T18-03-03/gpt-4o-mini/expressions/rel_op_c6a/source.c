/*
 * rel_op_c6a
 */

int main(void) {
  int a = 1;
  int b = 2;
  int c = 2;

  if (!(a < b < c))
    return 1;

  if (!((a < b) < c))
    return 2;

  a = 3;
  b = 2;
  c = 2;

  if (!(a < b < c))
    return 3;

  return 0;
}
