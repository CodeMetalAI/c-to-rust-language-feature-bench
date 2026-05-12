/*
 * rel_op_c4
 */

int main(void) {
  int x = 10;
  int y = 20;

  int *px = &x;
  int *py = &y;

  int *px_end = px + 1;
  int *py_end = py + 1;

  if (px != &x)
    return 1;

  if (!(px_end > px))
    return 2;

  if (px_end != (&x + 1))
    return 3;

  if (!(py_end > py))
    return 4;

  return 0;
}
