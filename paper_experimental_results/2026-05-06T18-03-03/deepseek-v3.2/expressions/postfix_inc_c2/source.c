/*
 * postfix_inc_c2
 */

int main(void) {
  _Atomic int x = 41;

  int old = x++;
  if (old != 41)
    return 1;
  if (x != 42)
    return 2;

  old = x++;
  if (old != 42)
    return 3;
  if (x != 43)
    return 4;

  x = 100;
  int y = (x++) + 0;
  if (y != 100)
    return 5;
  if (x != 101)
    return 6;

  return 0;
}
