/* bool_char_int_c2.c */
#define TYPE_ID(x)                                                             \
  _Generic((x),                                                                \
      int: 1,                                                                  \
      unsigned int: 2,                                                         \
      long: 3,                                                                 \
      unsigned long: 4,                                                        \
      long long: 5,                                                            \
      unsigned long long: 6,                                                   \
      float: 7,                                                                \
      double: 8,                                                               \
      long double: 9,                                                          \
      default: 99)

struct BF {
  unsigned int u1 : 1;
  int i1 : 1;
  _Bool b1 : 1;
};

int main(void) {
  if (TYPE_ID((signed char)1 + 0) != 1)
    return 1;
  if (TYPE_ID((unsigned char)1 + 0) != 1)
    return 2;
  if (TYPE_ID((short)1 + 0) != 1)
    return 3;
  if (TYPE_ID((unsigned short)1 + 0) != 1)
    return 4;

  struct BF bf;
  bf.u1 = 1u;
  bf.i1 = -1;
  bf.b1 = 1;

  if (TYPE_ID(bf.u1 + 0) != 1)
    return 5;
  if (TYPE_ID(bf.i1 + 0) != 1)
    return 6;
  if (TYPE_ID(bf.b1 + 0) != 1)
    return 7;

  if (TYPE_ID((float)1.0f) != 7)
    return 8;
  if (TYPE_ID((double)1.0) != 8)
    return 9;
  if (TYPE_ID((long double)1.0L) != 9)
    return 10;

  return 0;
}
