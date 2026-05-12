// restrict_def_4.c
volatile int gate4 = 1;

static void copy_restrict(int n, int *restrict p, int *restrict q) {
  while (n-- > 0)
    *p++ = *q++;
}

static void fill(int *p, int n, int base) {
  int i = 0;
  while (i < n) {
    p[i] = base + i * 5 + 1;
    i += 1;
  }
}

static int same(const int *a, const int *b, int n) {
  int i = 0;
  while (i < n) {
    if (a[i] != b[i])
      return 0;
    i += 1;
  }
  return 1;
}

static int sum(const int *a, int n) {
  int s = 0;
  int i = 0;
  while (i < n) {
    s += a[i];
    i += 1;
  }
  return s;
}

int main(void) {
  int x[32];
  int y[32];
  int y_snapshot[32];

  fill(x, 32, 10);
  fill(y, 32, 100);

  {
    int i = 0;
    while (i < 32) {
      y_snapshot[i] = y[i];
      i += 1;
    }
  }

  {
    int *restrict p1;
    int *restrict q1;

    p1 = x;
    q1 = y;

    if (gate4) {
      int *restrict p2 = p1;
      int *restrict q2 = q1;

      copy_restrict(16, p2 + 8, q2 + 4);

      {
        int expect[16];
        int i = 0;
        while (i < 16) {
          expect[i] = y_snapshot[4 + i];
          i += 1;
        }
        if (!same(&x[8], expect, 16))
          return 1;
      }

      if (!same(y, y_snapshot, 32))
        return 2;
    }

    copy_restrict(8, p1 + 0, q1 + 24);

    {
      int expect2[8];
      int i = 0;
      while (i < 8) {
        expect2[i] = y_snapshot[24 + i];
        i += 1;
      }
      if (!same(&x[0], expect2, 8))
        return 3;
    }
  }

  if (!same(y, y_snapshot, 32))
    return 4;

  if (sum(x, 32) == 0)
    return 5;

  return 0;
}
