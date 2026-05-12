/*
 * struct_and_union_2
 */

struct s {
  int i;
  const int ci;
};

static struct s s = {1, 2};
static const struct s cs = {3, 4};
static volatile struct s vs = {5, 6};

static void f(int *p) { (void)p; }
static void f4(const int *p) { (void)p; }
static void f2(volatile int *p) { (void)p; }
static void g(volatile const int *p) { (void)p; }

int main(void) {
  f(&s.i);
  f4(&s.ci);

  f4(&cs.i);
  f4(&cs.ci);

  f2(&vs.i);
  g(&vs.ci);

  if (s.i != 1)
    return 1;
  if (s.ci != 2)
    return 2;
  if (cs.i != 3)
    return 3;
  if (cs.ci != 4)
    return 4;
  if (vs.i != 5)
    return 5;
  if (vs.ci != 6)
    return 6;

  s.i = 10;
  if (s.i != 10)
    return 7;

  vs.i = 20;
  if (vs.i != 20)
    return 8;

  return 0;
}
