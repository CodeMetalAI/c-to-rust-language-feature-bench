/* bool_char_int_c1.c */
typedef unsigned long size_t;

#define TYPE_ID(x)                                                             \
  _Generic((x),                                                                \
      _Bool: 1,                                                                \
      char: 2,                                                                 \
      signed char: 3,                                                          \
      unsigned char: 4,                                                        \
      short: 5,                                                                \
      unsigned short: 6,                                                       \
      int: 7,                                                                  \
      unsigned int: 8,                                                         \
      long: 9,                                                                 \
      unsigned long: 10,                                                       \
      long long: 11,                                                           \
      unsigned long long: 12,                                                  \
      default: 99)

enum E { E_NEG = -1, E_POS = 1 };

static int expect_type(int got, int want) { return got == want; }

int main(void) {
  if (!expect_type(TYPE_ID((_Bool)1 + 0), 7))
    return 1;
  if (!expect_type(TYPE_ID((char)1 + 0), 7))
    return 2;
  if (!expect_type(TYPE_ID((signed char)1 + 0), 7))
    return 3;
  if (!expect_type(TYPE_ID((unsigned char)1 + 0), 7))
    return 4;
  if (!expect_type(TYPE_ID((short)1 + 0), 7))
    return 5;
  if (!expect_type(TYPE_ID((unsigned short)1 + 0), 7))
    return 6;

  if (!expect_type(TYPE_ID((int)0 + (unsigned int)0), 8))
    return 7;
  if (!expect_type(TYPE_ID((long)0 + (unsigned long)0), 10))
    return 8;
  if (!expect_type(TYPE_ID((long long)0 + (unsigned long long)0), 12))
    return 9;

  if (!expect_type(TYPE_ID((int)0 + (long)0), 9))
    return 10;
  if (!expect_type(TYPE_ID((long)0 + (long long)0), 11))
    return 11;
  if (!expect_type(TYPE_ID((int)0 + (long long)0), 11))
    return 12;

  if (!expect_type(TYPE_ID((enum E)0 + 0u), TYPE_ID((int)0 + 0u)))
    return 13;
  if (!expect_type(TYPE_ID((enum E)0 + 0), TYPE_ID((int)0 + 0)))
    return 14;

  return 0;
}
