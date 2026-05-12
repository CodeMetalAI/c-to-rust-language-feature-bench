// initialization_4.c
static int check_row(const int a[3], int x, int y, int z) {
  return a[0] == x && a[1] == y && a[2] == z;
}

int main(void) {
  int z[4][3] = {{1}, {2}, {3}, {4}};

  if (!check_row(z[0], 1, 0, 0))
    return 1;
  if (!check_row(z[1], 2, 0, 0))
    return 2;
  if (!check_row(z[2], 3, 0, 0))
    return 3;
  if (!check_row(z[3], 4, 0, 0))
    return 4;

  {
    int *p = &z[0][0];
    if (p[1] != 0)
      return 5;
    if (p[4] != 2)
      return 6;
    if (p[7] != 0)
      return 7;
  }

  return 0;
}
