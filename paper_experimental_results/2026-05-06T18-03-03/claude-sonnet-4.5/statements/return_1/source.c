/* return_1.c
 */

struct s {
  double i;
};

union u {
  struct {
    int f1;
    struct s f2;
  } u1;
  struct {
    struct s f3;
    int f4;
  } u2;
};

static union u g;

struct s f(void) { return g.u1.f2; }

int foo(void) { return 1; }

int main(void) {
  g.u1.f2.i = 1.0;
  g.u2.f3.i = 1.0;
  g.u1.f2 = f();
  g.u2.f3 = f();
  double val = foo();
  return g.u1.f2.i + g.u2.f3.i + val == 3.0 ? 0 : 1;
}
