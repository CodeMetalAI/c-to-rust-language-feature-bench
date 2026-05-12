// fun_decl_2.c
static int hmix(int a, int b) {
  unsigned ua = (unsigned)a;
  unsigned ub = (unsigned)b;
  ua ^= ua >> 16;
  ub ^= ub >> 15;
  return (int)(ua * 1103515245u + ub * 12345u);
}

static int f0(int *x, int *y) {
  int t = *x;
  *x = *y;
  *y = t;
  return hmix(*x, *y);
}

static int f1(int *x, int *y) {
  int a = *x;
  int b = *y;
  *x = a + 7;
  *y = b - 3;
  return hmix(*x, *y) ^ 1;
}

static int f2(int *x, int *y) {
  int a = *x;
  int b = *y;
  int d = a - b;
  if (d < 0)
    d = -d;
  *x = d;
  *y = a + b;
  return hmix(*x, *y) ^ 2;
}

static int run(int (*pf)(int *x, int *y), int *a, int *b) {
  int r1 = pf(a, b);
  int r2 = (*pf)(a, b);
  return r1 ^ (r2 + 3);
}

int main(void) {
  int (*apfi[3])(int *x, int *y);

  apfi[0] = f0;
  apfi[1] = f1;
  apfi[2] = f2;

  {
    int v[6];
    int i = 0;
    while (i < 6) {
      v[i] = 40 + i * 3;
      i += 1;
    }

    int sel = (hmix(v[0], v[5]) & 3);
    if (sel == 3)
      sel = 2;

    {
      int *x = &v[sel];
      int *y = &v[sel + 1];

      int before_x = *x;
      int before_y = *y;

      int out = run(apfi[sel], x, y);

      if (sel == 0) {
        if (*x != before_y)
          return 1;
        if (*y != before_x)
          return 2;
        if ((out ^ 3) != (hmix(*x, *y) ^ (hmix(*x, *y) + 3)))
          return 3;
      } else if (sel == 1) {
        if (*x != before_x + 7)
          return 4;
        if (*y != before_y - 3)
          return 5;
      } else {
        int d = before_x - before_y;
        if (d < 0)
          d = -d;
        if (*x != d)
          return 6;
        if (*y != before_x + before_y)
          return 7;
      }

      if (apfi[sel] == (int (*)(int *, int *))0)
        return 8;
    }
  }

  return 0;
}
