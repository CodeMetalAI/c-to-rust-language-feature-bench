/*
 * compound_literals_2
 */

static int f(int init) {
  int x = init;
  int *p = &x;

  p = (int[2]){*p};

  if (p[0] != init)
    return 1;
  if (p[1] != 0)
    return 2;

  return 0;
}

int main(void) {
  if (f(7) != 0)
    return 10;
  if (f(-3) != 0)
    return 11;
  return 0;
}
