/*
 * prefix_dec_c2
 */

int main(void) {
  int x = 5;

  int y = --x;
  if (y != 4)
    return 1;
  if (x != 4)
    return 2;

  x = 10;
  y = --x;
  int z = 10;
  z -= 1;
  if (y != 9)
    return 3;
  if (x != 9)
    return 4;
  if (z != 9)
    return 5;

  return 0;
}
