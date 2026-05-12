/*
 * rel_op_c6b
 */

struct S {
  int x;
  int y;
};

int main(void) {
  int *np1 = (int *)0;
  int *np2 = (int *)0;
  if (np1 != np2)
    return 1;

  struct S s = {0, 0};
  void *ps = (void *)&s;
  void *px = (void *)&s.x;
  if (ps != px)
    return 2;

  int a[3] = {1, 2, 3};
  int *end1 = &a[3];
  int *end2 = a + 3;
  if (end1 != end2)
    return 3;

  return 0;
}
