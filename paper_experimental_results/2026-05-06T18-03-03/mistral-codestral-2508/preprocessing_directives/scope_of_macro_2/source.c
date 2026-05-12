/* scope_of_macro_2.c
 */

#define max(a, b) ((a) > (b) ? (a) : (b))

int main(void) {
  int i = 3;
  int j = 5;
  if (max(i, j) != 5)
    return 1;

  double x = 2.5;
  double y = 1.5;
  if (max(x, y) != 2.5)
    return 2;

  int k = 1;
  int r = max(k++, 2);
  if (r != 2)
    return 3;
  if (k != 2)
    return 4;

  int m = 3;
  int n = 1;
  r = max(m++, n++);
  if (r != 3)
    return 5;
  if (m != 4)
    return 6;
  if (n != 2)
    return 7;

  return 0;
}
