/*
 * add_op_1a
 */

int main(void) {
  int n = 4, m = 3;
  int a[n][m];

  int (*p)[m] = a;

  p += 1;
  (*p)[2] = 99;

  if (a[1][2] != 99)
    return 1;

  n = (int)(p - a);
  if (n != 1)
    return 2;

  return 0;
}
