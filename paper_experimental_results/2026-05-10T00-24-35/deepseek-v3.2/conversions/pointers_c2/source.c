/* pointers_c2.c
 */

typedef unsigned long size_t;

int main(void) {
  int x = 17;
  int *p = &x;

  const int *pc = p;
  volatile int *pv = p;
  const volatile int *pcv = p;

  if ((const void *)pc != (const void *)p)
    return 1;
  if ((const void *)pv != (const void *)p)
    return 2;
  if ((const void *)pcv != (const void *)p)
    return 3;

  if (*pc != 17)
    return 4;
  if (*pv != 17)
    return 5;
  if (*pcv != 17)
    return 6;

  return 0;
}
