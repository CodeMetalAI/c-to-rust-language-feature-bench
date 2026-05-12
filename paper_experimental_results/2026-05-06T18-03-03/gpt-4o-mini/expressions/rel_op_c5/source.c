/*
 * rel_op_c5
 */

struct st {
  int a;
  int b;
};

union un {
  int i;
  double d;
};

int main(void) {
  int x = 0;
  int *px1 = &x;
  int *px2 = &x;
  if (px1 != px2)
    return 1;

  int arr[3] = {1, 2, 3};
  int *p_end1 = &arr[3];
  int *p_end2 = arr + 3;
  if (p_end1 != p_end2)
    return 2;

  int *p0 = &arr[0];
  int *p2 = &arr[2];
  if (!(p2 > p0))
    return 3;
  if (!(p0 < p2))
    return 4;

  int *q_last = &arr[2];
  int *q1 = q_last + 1;
  if (!(q1 > p0))
    return 5;

  struct st s = {0, 0};
  char *sa = (char *)&s.a;
  char *sb = (char *)&s.b;
  if (!(sb > sa))
    return 6;

  union un u;
  char *ui = (char *)&u.i;
  char *ud = (char *)&u.d;
  if (ui != ud)
    return 7;

  return 0;
}
