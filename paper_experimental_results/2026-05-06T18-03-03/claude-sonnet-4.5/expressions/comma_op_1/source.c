/*
 * comma_op_1
 */

int f(int a, int t, int c) { return a + t + c; }

int main(void) {

  int a = 0, c = 0;
  int t;

  int v = f(a, (t = 3, t + 2), c);

  if (v != 5) {
    return 1;
  }

  int t1;
  if ((t1 = 3, t + 2) != 5) {
    return 2;
  }

  int t2;
  int t3 = (1 < 2) ? (t2 = 0, t2 + 2) : (t2 = 1, t2 + 2);
  if (t3 != 2) {
    return 3;
  }

  return 0;
}
