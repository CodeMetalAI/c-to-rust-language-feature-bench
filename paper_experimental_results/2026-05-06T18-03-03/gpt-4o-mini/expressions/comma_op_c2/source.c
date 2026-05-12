/* comma_op_c2.c
 */

int f1() { return 1; }
int f2(int t) { return t + 1; }

int main(void) {
  int t = 10;
  int val = (t = f1(), f2(t));
  return val == 2 ? 0 : 1;
}
