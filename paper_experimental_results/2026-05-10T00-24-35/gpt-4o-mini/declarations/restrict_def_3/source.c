// restrict_def_3.c
volatile int opaque = 3;

void h(int n, int *restrict p, int *restrict q, int *restrict r) {
  int i;
  for (i = 0; i < n; i++)
    p[i] = q[i] + r[i];
}

static void init_seq(int *x, int n, int base) {
  int i = 0;
  while (i < n) {
    x[i] = base + i * 7 + 1;
    i += 1;
  }
}

static int check_unchanged(const int *x, const int *y, int n) {
  int i = 0;
  while (i < n) {
    if (x[i] != y[i])
      return 0;
    i += 1;
  }
  return 1;
}

static int check_p_is_double_q(const int *p, const int *q, int n) {
  int i = 0;
  while (i < n) {
    if (p[i] != q[i] + q[i])
      return 0;
    i += 1;
  }
  return 1;
}

static void touch_readonly(const int *x, int n) {
  volatile int sink = 0;
  int i = 0;
  while (i < n) {
    sink ^= x[i] + opaque;
    i += 1;
  }
}

int main(void) {
  int a[64];
  int b[64];
  int b_snapshot[64];

  init_seq(a, 64, 10);
  init_seq(b, 64, 100);

  {
    int i = 0;
    while (i < 64) {
      b_snapshot[i] = b[i];
      i += 1;
    }
  }

  touch_readonly(b, 64);

  h(64, a, b, b);

  if (!check_p_is_double_q(a, b_snapshot, 64))
    return 1;

  if (!check_unchanged(b, b_snapshot, 64))
    return 2;

  touch_readonly(b, 64);

  return 0;
}
