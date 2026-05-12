/* pointers_c1.c */
/* pointer_void_roundtrip.c */

typedef unsigned long size_t;

struct S {
  int a;
  double b;
};

int main(void) {
  int xi = 42;
  double xd = 3.25;
  struct S xs = {7, 9.5};

  int *pi = &xi;
  double *pd = &xd;
  struct S *ps = &xs;

  void *v1 = pi;
  void *v2 = pd;
  void *v3 = ps;

  int *pi2 = (int *)v1;
  double *pd2 = (double *)v2;
  struct S *ps2 = (struct S *)v3;

  if (pi2 != pi)
    return 1;
  if (pd2 != pd)
    return 2;
  if (ps2 != ps)
    return 3;

  if (*pi2 != 42)
    return 4;
  if (*pd2 != 3.25)
    return 5;
  if (ps2->a != 7)
    return 6;
  if (ps2->b != 9.5)
    return 7;

  void *v1b = (void *)pi2;
  void *v2b = (void *)pd2;
  void *v3b = (void *)ps2;

  if (v1b != v1)
    return 8;
  if (v2b != v2)
    return 9;
  if (v3b != v3)
    return 10;

  return 0;
}
