// restrict_def_2b.c

volatile int gate = 1;

void f(int n, int *restrict p, int *restrict q) {
  while (n-- > 0)
    *p++ = *q++;
}

static void safe_move(int n, int *p, const int *q) {
  if (n <= 0)
    return;

  if (p < q) {
    int i = 0;
    while (i < n) {
      p[i] = q[i];
      i += 1;
    }
  } else if (p > q) {
    int i = n;
    while (i-- > 0) {
      p[i] = q[i];
    }
  } else {
    return;
  }
}

static int ranges_overlap(const int *base, int n_total, const int *p,
                          const int *q, int n) {
  volatile int g = gate;
  int ps = (int)(p - base);
  int qs = (int)(q - base);

  if (g) {
    ps += 0;
    qs += 0;
  }

  if (n < 0)
    return 0;
  if (ps < 0 || qs < 0)
    return 0;
  if (ps + n > n_total || qs + n > n_total)
    return 0;

  return (ps < qs + n) && (qs < ps + n);
}

static void call_f_checked(const int *base, int n_total, int n, int *p,
                           int *q) {
  if (ranges_overlap(base, n_total, p, q, n))
    safe_move(n, p, q);
  else
    f(n, p, q);
}

int d[100];

static void init(int *x, int n) {
  int i = 0;
  while (i < n) {
    x[i] = i * 13 + 5;
    i += 1;
  }
}

static int same(const int *x, const int *y, int n) {
  int i = 0;
  while (i < n) {
    if (x[i] != y[i])
      return 0;
    i += 1;
  }
  return 1;
}

static void g_defined(void) {
  call_f_checked(d, 100, 50, d + 50, d);
  call_f_checked(d, 100, 50, d + 1, d);
}

int main(void) {
  int expect[100];

  init(d, 100);
  init(expect, 100);

  safe_move(50, expect + 50, expect);
  safe_move(50, expect + 1, expect);

  g_defined();

  if (!same(d, expect, 100))
    return 1;

  return 0;
}
