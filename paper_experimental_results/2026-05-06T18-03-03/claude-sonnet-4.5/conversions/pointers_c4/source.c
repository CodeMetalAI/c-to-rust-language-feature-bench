/* pointers_c4.c */

int f(int x) { return x; }

int main(void) {
  int *p0 = 0;
  void *v0 = (void *)0;
  int *p1 = (int *)v0;

  int (*fp0)(int) = 0;
  int (*fp1)(int) = (int (*)(int))0;
  int (*fp2)(int) = (int (*)(int))v0;

  if (p0 != p1)
    return 1;
  if (p0 != (int *)0)
    return 2;

  if (v0 != (void *)0)
    return 3;
  if (v0 != (void *)p0)
    return 4;

  if (fp0 != fp1)
    return 5;
  if (fp1 != fp2)
    return 6;
  if (fp0 != (int (*)(int))0)
    return 7;

  if (p0 != (int *)fp0)
    return 8;

  return 0;
}
