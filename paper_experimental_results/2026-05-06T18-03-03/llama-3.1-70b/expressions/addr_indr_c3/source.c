/*
 * addr_indr_c3
 */

int main(void) {
  int x = 7;
  int *p = &x;

  if (&x != p)
    return 1;

  if (&(*p) != p)
    return 2;

  int a[3] = {10, 20, 30};

  if (&a[0] != a + 0)
    return 3;
  if (&a[1] != a + 1)
    return 4;
  if (&a[2] != a + 2)
    return 5;

  if (*(a + 1) != 20)
    return 6;

  return 0;
}
