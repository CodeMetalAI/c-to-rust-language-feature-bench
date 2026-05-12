// initialization_7.c
typedef int A[];

static int sum(const int *p, int n) {
  int s = 0;
  int i = 0;
  while (i < n) {
    s += p[i];
    i += 1;
  }
  return s;
}

int main(void) {
  A a = {1, 2};
  A b = {3, 4, 5};

  if (sizeof(a) / sizeof(a[0]) != 2)
    return 1;
  if (sizeof(b) / sizeof(b[0]) != 3)
    return 2;

  if (sum(a, 2) != 3)
    return 3;
  if (sum(b, 3) != 12)
    return 4;

  {
    int *pa = a;
    int *pb = b;
    if (pa[1] != 2)
      return 5;
    if (pb[2] != 5)
      return 6;
  }

  return 0;
}
