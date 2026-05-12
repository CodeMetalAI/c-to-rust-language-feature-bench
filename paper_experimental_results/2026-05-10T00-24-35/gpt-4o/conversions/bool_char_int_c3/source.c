/* bool_char_int_c3.c */
#define TYPE_ID(x) _Generic((x), int: 1, unsigned int: 2, default: 99)

int main(void) {
  volatile char pc = (char)-1;
  volatile signed char sc = (signed char)-1;
  volatile unsigned char uc = (unsigned char)-1;

  int ipc = pc;
  int isc = sc;
  int iuc = uc;

  if (TYPE_ID(pc + 0) != 1)
    return 1;
  if (TYPE_ID(sc + 0) != 1)
    return 2;
  if (TYPE_ID(uc + 0) != 1)
    return 3;

  if (isc != -1)
    return 4;

  if ((char)-1 < 0) {
    if (ipc != -1)
      return 5;
    if ((unsigned int)iuc != 255u)
      return 6;
  } else {
    if ((unsigned int)ipc != 255u)
      return 7;
    if ((unsigned int)iuc != 255u)
      return 8;
  }

  return 0;
}
