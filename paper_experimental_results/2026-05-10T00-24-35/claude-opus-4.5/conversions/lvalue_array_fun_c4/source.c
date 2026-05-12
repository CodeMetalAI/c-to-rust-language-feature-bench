/* lvalue_array_fun_c4.c */

typedef unsigned long size_t;

#define TYPE_ID(x)                                                             \
  _Generic((x), int (*)(int): 1, int (*)(int, int): 2, default: 99)

int f1(int x) { return x + 1; }
int f2(int x, int y) { return x + y; }

int main(void) {
  int (*p1)(int) = f1;
  int (*q1)(int) = &f1;

  if (TYPE_ID(p1) != 1)
    return 1;
  if (TYPE_ID(q1) != 1)
    return 2;

  if (p1 != q1)
    return 3;

  if (p1(3) != 4)
    return 4;
  if (f1(3) != 4)
    return 5;

  int (*r1)(int) = (1 ? f1 : f1);
  if (TYPE_ID(r1) != 1)
    return 6;
  if (r1(4) != 5)
    return 7;

  int (*p2)(int, int) = f2;
  int (*q2)(int, int) = &f2;

  if (TYPE_ID(p2) != 2)
    return 8;
  if (TYPE_ID(q2) != 2)
    return 9;

  if (p2 != q2)
    return 10;
  if (p2(2, 3) != 5)
    return 11;
  if (f2(2, 3) != 5)
    return 12;

  int (*r2)(int, int) = (0 ? f2 : f2);
  if (TYPE_ID(r2) != 2)
    return 13;
  if (r2(10, 20) != 30)
    return 14;

  return 0;
}
