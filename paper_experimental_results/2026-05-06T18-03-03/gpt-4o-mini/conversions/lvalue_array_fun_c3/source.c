/* lvalue_array_fun_c3.c */
typedef unsigned long size_t;

#define TYPE_ID(x)                                                             \
  _Generic((x),                                                                \
      int *: 1,                                                                \
      int (*)[3]: 2,                                                           \
      const char *: 3,                                                         \
      const char[4]: 4,                                                        \
      default: 99)

int main(void) {
  int a[3] = {10, 20, 30};

  if (TYPE_ID(a) != 1)
    return 1;
  if (TYPE_ID(&a) != 2)
    return 2;

  if (*a != 10)
    return 3;
  if (*(a + 1) != 20)
    return 4;
  if (*(a + 2) != 30)
    return 5;

  if (sizeof a != 3 * sizeof(int))
    return 6;
  if (_Alignof(a) != _Alignof(int))
    return 7;

  char s[4] = "abc";
  if (TYPE_ID("abc") != 3)
    return 8;
  if (s[0] != 'a')
    return 9;
  if (s[1] != 'b')
    return 10;
  if (s[2] != 'c')
    return 11;
  if (s[3] != '\0')
    return 12;

  return 0;
}
