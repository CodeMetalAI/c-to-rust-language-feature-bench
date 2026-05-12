/* pointers_c5.c*/

typedef unsigned long uintptr_t;

int main(void) {
  int x = 1;
  int *px = &x;

  uintptr_t ip = (uintptr_t)px;
  int *p2 = (int *)ip;

  if ((uintptr_t)p2 != ip)
    return 1;

  uintptr_t z = 1;
  void *vp = (void *)z;
  uintptr_t z2 = (uintptr_t)vp;

  if (z2 != z)
    return 2;

  return 0;
}
