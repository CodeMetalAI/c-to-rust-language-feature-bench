/*
 * prefix_inc_c1
 */

int main(void) {
  int x = 5;

  int y = ++x;
  if (y != 6)
    return 1;
  if (x != 6)
    return 2;

  x = 10;
  y = ++x;
  int z = 10;
  z += 1;
  if (y != 11)
    return 3;
  if (z != 11)
    return 4;
  if (z != 11)
    return 5;

  return 0;
}
