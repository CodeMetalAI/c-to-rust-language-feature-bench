// initialization_3.c
static int check_row(const int a[3], int x, int y, int z) {
  return a[0] == x && a[1] == y && a[2] == z;
}

int main(void) {
  int y1[4][3] = {
      {1, 3, 5},
      {2, 4, 6},
      {3, 5, 7},
  };

  int y2[4][3] = {1, 3, 5, 2, 4, 6, 3, 5, 7};

  if (!check_row(y1[0], 1, 3, 5))
    return 1;
  if (!check_row(y1[1], 2, 4, 6))
    return 2;
  if (!check_row(y1[2], 3, 5, 7))
    return 3;
  if (!check_row(y1[3], 0, 0, 0))
    return 4;

  if (!check_row(y2[0], 1, 3, 5))
    return 5;
  if (!check_row(y2[1], 2, 4, 6))
    return 6;
  if (!check_row(y2[2], 3, 5, 7))
    return 7;
  if (!check_row(y2[3], 0, 0, 0))
    return 8;

  {
    int *p1 = &y1[0][0];
    int *p2 = &y2[0][0];
    if (p1[11] != 0)
      return 9;
    if (p2[11] != 0)
      return 10;
  }

  return 0;
}
