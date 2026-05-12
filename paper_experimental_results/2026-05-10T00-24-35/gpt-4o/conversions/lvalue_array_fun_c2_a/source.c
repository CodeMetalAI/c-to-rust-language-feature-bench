/* lvalue_array_fun_c2_a.c */
typedef unsigned long size_t;

#define TYPE_ID(x)                                                             \
  _Generic((x),                                                                \
      int: 1,                                                                  \
      int *: 2,                                                                \
      const int *: 3,                                                          \
      _Atomic(int): 4,                                                         \
      _Atomic(int) *: 5,                                                       \
      int (*)(int): 6,                                                         \
      const int: 7,                                                            \
      default: 99)

int id(int v) { return v + 1; }

int main(void) {
  const int cx = 9;
  _Atomic(int) ax = 11;

  int a[3] = {1, 2, 3};

  int *pa = a;
  if (pa[0] != 1)
    return 1;
  if (pa[1] != 2)
    return 2;
  if (pa[2] != 3)
    return 3;

  if (TYPE_ID(a) != 99)
    return 4;
  if (TYPE_ID(&a[0]) != 2)
    return 5;

  if (TYPE_ID(&cx) != 3)
    return 6;
  if (TYPE_ID(+cx) != 1)
    return 7;
  if ((+cx) != 9)
    return 8;

  if (TYPE_ID(ax) != 4)
    return 9;
  if (TYPE_ID(&ax) != 5)
    return 10;
  if (TYPE_ID(+ax) != 1)
    return 11;
  if ((+ax) != 11)
    return 12;

  int (*fp)(int) = id;
  if (TYPE_ID(id) != 6)
    return 13;
  if (fp(4) != 5)
    return 14;
  if (id(4) != 5)
    return 15;

  if (sizeof a != 3 * sizeof(int))
    return 16;
  if (_Alignof(a[0]) != _Alignof(int))
    return 17;

  return 0;
}
