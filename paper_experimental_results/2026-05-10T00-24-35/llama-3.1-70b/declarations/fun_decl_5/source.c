// fun_decl_5.c
double maximum(int n, int m, double a[n][m]);
double maximum(int n, int m, double a[][m]);

double maximum(int n, int m, double a[][m]) {
  double mx = a[0][0];
  int i = 0;
  while (i < n) {
    int j = 0;
    while (j < m) {
      double v = a[i][j];
      if (v > mx)
        mx = v;
      j += 1;
    }
    i += 1;
  }
  return mx;
}

static double call_as_6(double (*pf)(int, int, double (*)[6]), int n, int m,
                        double a[n][m]) {
  double (*p)[6] = (double (*)[6])a;
  return pf(n, m, p);
}

void f(double (*restrict a)[5]);
void f(double a[restrict][5]);

void f(double a[restrict][5]) {
  int i = 0;
  while (i < 3) {
    a[i][0] += 1.0;
    a[i][4] += 1.0;
    i += 1;
  }
}

int main(void) {
  int n = 4;
  int m = 6;

  double mat[4][6];

  {
    int i = 0;
    while (i < n) {
      int j = 0;
      while (j < m) {
        mat[i][j] = 0.0;
        j += 1;
      }
      i += 1;
    }
    mat[2][3] = 9.5;
    mat[1][5] = 7.0;
  }

  {
    double r0 = maximum(n, m, mat);
    double r1 =
        call_as_6((double (*)(int, int, double (*)[6]))maximum, n, m, mat);

    if (r0 != 9.5)
      return 1;
    if (r1 != 9.5)
      return 2;
  }

  {
    double a3[3][5];
    int i = 0;
    while (i < 3) {
      int j = 0;
      while (j < 5) {
        a3[i][j] = (double)(i * 10 + j);
        j += 1;
      }
      i += 1;
    }

    f(a3);

    if (a3[0][0] != 1.0)
      return 3;
    if (a3[0][4] != 5.0)
      return 4;
    if (a3[1][0] != 11.0)
      return 5;
    if (a3[1][4] != 15.0)
      return 6;
    if (a3[2][0] != 21.0)
      return 7;
    if (a3[2][4] != 25.0)
      return 8;
  }

  return 0;
}
