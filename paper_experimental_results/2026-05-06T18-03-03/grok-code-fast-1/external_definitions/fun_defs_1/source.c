/* fun_defs_1.c
 */

static int arr[256];

extern int f(unsigned char a, unsigned char b) {
  arr[b] = 1;
  return b;
}
extern int g(a, b)
int a;
int b;
{
  arr[(unsigned char)b] += b;
  return a + b;
}

int main() {
  signed char a = 0;
  signed char b = -1;

  int r1 = f(a, b);
  int r2 = g(a, b);

  if (r1 != 255)
    return 1;
  if (r2 != -1)
    return 2;
  unsigned char ub = (unsigned char)b;
  if (arr[ub] != 0)
    return 3;
  return 0;
}
