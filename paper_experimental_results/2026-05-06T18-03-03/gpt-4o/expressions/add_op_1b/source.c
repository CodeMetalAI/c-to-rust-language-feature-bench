/*
 * add_op_1b
 */

int main(void) {
  int a[4][3];

  int (*p)[3] = a;

  p += 1;
  (*p)[2] = 99;

  if (a[1][2] != 99)
    return 1;

  if ((p - a) != 1)
    return 2;

  return 0;
}
