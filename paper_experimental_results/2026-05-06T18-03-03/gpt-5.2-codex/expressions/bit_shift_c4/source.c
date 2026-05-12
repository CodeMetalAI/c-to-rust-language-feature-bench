/*
 * bit_shift_c4
 */

int main(void) {
  unsigned int x = 1u;

  unsigned int y = x << 3;
  if (y != 8u)
    return 1;

  if ((0u << 5) != 0u)
    return 2;

  if ((2u << 2) != 8u)
    return 3;

  return 0;
}
