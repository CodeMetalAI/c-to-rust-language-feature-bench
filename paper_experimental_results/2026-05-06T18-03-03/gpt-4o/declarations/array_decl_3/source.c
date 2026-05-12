// array_decl_3.c
extern int n;
extern int m;

int n;
int m;

static volatile int gate;

static int check_a(int nn, int mm, int a[nn][6][mm], int (*p)[6][mm]) {
  int i = 0;
  while (i < nn) {
    int j = 0;
    while (j < 6) {
      int k = 0;
      while (k < mm) {
        if (a[i][j][k] != (*p)[j][k] + i)
          return 1;
        k += 1;
      }
      j += 1;
    }
    i += 1;
    p += 1;
  }
  return 0;
}

static int check_c(int nn, int mm, int c[nn][nn][6][mm], int (*r)[nn][6][mm]) {
  int i = 0;
  while (i < nn) {
    int j = 0;
    while (j < nn) {
      int u = 0;
      while (u < 6) {
        int v = 0;
        while (v < mm) {
          if (c[i][j][u][v] != r[i][j][u][v])
            return 1;
          v += 1;
        }
        u += 1;
      }
      j += 1;
    }
    i += 1;
  }
  return 0;
}

void fcompat(void) {
  int nn = n;
  int mm = m;

  int a[nn][6][mm];
  int (*p)[6][mm];

  int c[nn][nn][6][mm];
  int (*r)[nn][6][mm];

  int i = 0;
  while (i < nn) {
    int j = 0;
    while (j < 6) {
      int k = 0;
      while (k < mm) {
        a[i][j][k] = (i + 1) * 10000 + (j + 1) * 100 + (k + 1);
        k += 1;
      }
      j += 1;
    }
    i += 1;
  }

  i = 0;
  while (i < nn) {
    int j = 0;
    while (j < nn) {
      int u = 0;
      while (u < 6) {
        int v = 0;
        while (v < mm) {
          c[i][j][u][v] =
              (i + 1) * 1000000 + (j + 1) * 10000 + (u + 1) * 100 + (v + 1);
          v += 1;
        }
        u += 1;
      }
      j += 1;
    }
    i += 1;
  }

  p = a;
  if (check_a(nn, mm, a, p) != 0)
    gate = 1;

  r = c;
  if (check_c(nn, mm, c, r) != 0)
    gate = 2;
}

int main(void) {
  n = 6;
  m = n + 1;

  gate = 0;
  fcompat();

  if (gate != 0)
    return 1;

  return 0;
}
