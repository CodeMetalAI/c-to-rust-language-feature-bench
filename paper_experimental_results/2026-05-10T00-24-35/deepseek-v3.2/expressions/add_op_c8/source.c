/* add_op_c8.c */

int main(void) {
  int a[5] = {10, 20, 30, 40, 50};

  int *p0 = &a[0];
  int *p2 = p0 + 2;
  int *p4 = p2 + 2;
  int *p_last = &a[4];
  int *p_end = p_last + 1;

  if (*p2 != 30)
    return 1;
  if (*p4 != 50)
    return 2;

  int *p_back = p4 - 3;
  if (*p_back != 20)
    return 3;

  int *p_from_end = p_end - 1;
  if (p_from_end != p_last)
    return 4;
  if (*p_from_end != 50)
    return 5;

  if ((p0 + 5) != p_end)
    return 6;

  return 0;
}
