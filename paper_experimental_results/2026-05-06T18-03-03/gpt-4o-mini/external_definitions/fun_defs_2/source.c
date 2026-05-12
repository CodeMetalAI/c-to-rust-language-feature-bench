/* fun_defs_2.c
 */

static int counter = 0;

static int f(void) {
  counter += 7;
  return counter;
}

static int g_ptr(int (*funcp)(void)) {
  int a = (*funcp)();
  int b = funcp();
  return a + b;
}

static int g_fun(int func(void)) {
  int a = func();
  int b = (*func)();
  return a + b;
}

int main(void) {
  counter = 0;

  int r1 = g_ptr(f);
  if (counter != 14)
    return 1;
  if (r1 != 21)
    return 2;

  int r2 = g_fun(f);
  if (counter != 28)
    return 3;
  if (r2 != 49)
    return 4;

  return 0;
}
