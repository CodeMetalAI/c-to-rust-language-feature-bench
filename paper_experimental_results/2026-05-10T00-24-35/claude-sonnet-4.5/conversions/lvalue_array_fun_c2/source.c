/* lvalue_array_fun_c2.c */

typedef unsigned long size_t;

#define TYPE_ID(x)                                                             \
  _Generic((x), int: 1, unsigned int: 2, _Atomic(int): 3, default: 99)

int main(void) {
  int x = 3;
  const int cx = 4;
  _Atomic(int) ax = 5;

  if (sizeof x != sizeof(int))
    return 1;
  if (_Alignof(x) != _Alignof(int))
    return 2;

  if (*(&x) != 3)
    return 3;

  if (TYPE_ID(x) != 1)
    return 4;
  if (TYPE_ID(ax) != 3)
    return 5;

  if (TYPE_ID(+cx) != 1)
    return 6;
  if (TYPE_ID(+ax) != 1)
    return 7;

  int y = x;
  if (y != 3)
    return 8;

  x++;
  if (x != 4)
    return 9;

  struct S {
    int m;
  };
  struct S s = {7};
  if (s.m != 7)
    return 10;
  s.m = 8;
  if (s.m != 8)
    return 11;

  return 0;
}
