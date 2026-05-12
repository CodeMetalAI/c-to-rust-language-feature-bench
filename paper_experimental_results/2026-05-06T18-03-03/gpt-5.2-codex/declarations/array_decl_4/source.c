// array_decl_4.c
extern int n;

int n = 9;

static volatile int sink;

static int backing[9][9];

static void fill_backing(void) {
  int i = 0;
  while (i < 9) {
    int j = 0;
    while (j < 9) {
      backing[i][j] = (i + 1) * 100 + (j + 1);
      j += 1;
    }
    i += 1;
  }
}

static int checksum_2d(int m, int a[m][m]) {
  int s = 0;
  int i = 0;
  while (i < m) {
    int j = 0;
    while (j < m) {
      s ^= a[i][j] + i * 131 + j * 17;
      j += 1;
    }
    i += 1;
  }
  return s;
}

void fvla(int m, int C[m][m]) {
  typedef int VLA[m][m];

  int D[m];
  int (*s)[m] = C;
  static int (*q)[m];

  int i = 0;
  while (i < m) {
    D[i] = (i + 1) * 7 + 3;
    i += 1;
  }

  q = backing;

  i = 0;
  while (i < m) {
    int j = 0;
    while (j < m) {
      q[i][j] = s[i][j] + D[(i + j) % m];
      j += 1;
    }
    i += 1;
  }

  sink ^= checksum_2d(m, (VLA *)q);
}

int main(void) {
  int m = n;

  int X[m][m];
  int Y[m][m];

  fill_backing();

  {
    int i = 0;
    while (i < m) {
      int j = 0;
      while (j < m) {
        X[i][j] = (i + 1) * 10 + (j + 1);
        Y[i][j] = (i + 1) * 20 + (j + 1);
        j += 1;
      }
      i += 1;
    }
  }

  fvla(m, X);

  {
    int expect[9][9];
    int i = 0;
    while (i < m) {
      int j = 0;
      while (j < m) {
        int d = ((i + j) % m + 1) * 7 + 3;
        expect[i][j] = X[i][j] + d;
        j += 1;
      }
      i += 1;
    }
    if (checksum_2d(m, backing) != checksum_2d(m, expect))
      return 1;
  }

  fvla(m, Y);

  {
    int expect2[9][9];
    int i = 0;
    while (i < m) {
      int j = 0;
      while (j < m) {
        int d = ((i + j) % m + 1) * 7 + 3;
        expect2[i][j] = Y[i][j] + d;
        j += 1;
      }
      i += 1;
    }
    if (checksum_2d(m, backing) != checksum_2d(m, expect2))
      return 2;
  }

  if (sink == 0)
    return 3;

  return 0;
}
