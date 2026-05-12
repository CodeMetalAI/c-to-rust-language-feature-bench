/*
 * function_calls
 */

#include <stddef.h>

static int seen_f1 = 0, seen_f2 = 0, seen_f3 = 0, seen_f4 = 0;
static int logv[5];
static int logn = 0;

static void log_call(int id) {
  if (logn < 5)
    logv[logn++] = id;
}

static int f1(void) {
  seen_f1 = 1;
  log_call(1);
  return 0;
}
static int f2(void) {
  seen_f2 = 1;
  log_call(2);
  return 20;
}
static int f3(void) {
  seen_f3 = 1;
  log_call(3);
  return 30;
}
static int f4(void) {
  seen_f4 = 1;
  log_call(4);
  return 40;
}

typedef int (*fn2_t)(int, int);

static int target(int a, int b) {
  (void)a;
  (void)b;
  log_call(9);

  if (!seen_f1 || !seen_f2 || !seen_f3 || !seen_f4)
    return 100;

  return 0;
}

int main(void) {
  fn2_t pf[1] = {target};

  int rc = (*pf[f1()])(f2(), f3() + f4());
  if (rc != 0)
    return rc;

  if (logn != 5)
    return 1;
  if (logv[4] != 9)
    return 2;

  int counts[10] = {0};
  for (int i = 0; i < 4; ++i)
    counts[logv[i]]++;

  if (counts[1] != 1 || counts[2] != 1 || counts[3] != 1 || counts[4] != 1)
    return 3;

  return 0;
}
