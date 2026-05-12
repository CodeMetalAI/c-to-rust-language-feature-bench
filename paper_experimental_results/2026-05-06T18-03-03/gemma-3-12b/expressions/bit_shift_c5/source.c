/*
 * bit_shfit_c5
 */

int main(void) {
  unsigned int x = 16u;

  unsigned int y = x >> 2;
  if (y != 4u)
    return 1;

  if ((0u >> 5) != 0u)
    return 2;

  if ((8u >> 1) != 4u)
    return 3;

  return 0;
}
