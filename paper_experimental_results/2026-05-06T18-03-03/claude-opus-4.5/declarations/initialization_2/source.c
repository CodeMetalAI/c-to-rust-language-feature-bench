// initialization_2.c
static int sum3(const int a[3]) { return a[0] + a[1] + a[2]; }

int main(void) {
  int x[] = {1, 3, 5};

  if (sizeof(x) / sizeof(x[0]) != 3)
    return 1;

  if (x[0] != 1 || x[1] != 3 || x[2] != 5)
    return 2;

  if (sum3(x) != 9)
    return 3;

  {
    int *p = x;
    if (p[2] != 5)
      return 4;
  }

  return 0;
}
