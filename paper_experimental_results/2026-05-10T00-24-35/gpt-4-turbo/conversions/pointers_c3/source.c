/* pointers_c3.c
 */

typedef unsigned long size_t;

int f(int x) { return x + 1; }

int main(void) {
  int x = 1;
  int *px = &x;
  int (*pf)(int) = f;

  int *n1 = 0;
  void *v0 = (void *)0;
  int *n2 = (int *)v0;

  int (*nf1)(int) = 0;
  void *vf0 = (void *)0;
  int (*nf2)(int) = (int (*)(int))vf0;

  if (n1 != 0)
    return 1;
  if (n2 != 0)
    return 2;
  if (v0 != 0)
    return 3;

  if (n1 == px)
    return 4;
  if (n2 == px)
    return 5;

  if (nf1 != 0)
    return 6;
  if (nf2 != 0)
    return 7;
  if (nf1 == pf)
    return 8;
  if (nf2 == pf)
    return 9;

  void *vx = px;
  void *vf = (void *)pf;

  if (vx == 0)
    return 10;
  if (vf == 0)
    return 11;
  if (vx == v0)
    return 12;
  if (vf == vf0)
    return 13;

  return 0;
}
