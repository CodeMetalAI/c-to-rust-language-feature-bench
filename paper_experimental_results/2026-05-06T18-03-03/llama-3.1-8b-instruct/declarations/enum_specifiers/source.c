// enum_specifiers.c

enum hue { chartreuse, burgundy, claret = 20, winedark };

static int check_vals(enum hue v) {
  if (v == chartreuse)
    return 10;
  if (v == burgundy)
    return 11;
  if (v == claret)
    return 12;
  if (v == winedark)
    return 13;
  return 0;
}

int main(void) {
  enum hue col, *cp;

  if ((int)chartreuse != 0)
    return 1;
  if ((int)burgundy != 1)
    return 2;
  if ((int)claret != 20)
    return 3;
  if ((int)winedark != 21)
    return 4;

  col = claret;
  cp = &col;

  if (*cp != claret)
    return 5;
  if (*cp == burgundy)
    return 6;

  if (check_vals(*cp) != 12)
    return 7;

  col = winedark;
  if (check_vals(*cp) != 13)
    return 8;

  return 0;
}
