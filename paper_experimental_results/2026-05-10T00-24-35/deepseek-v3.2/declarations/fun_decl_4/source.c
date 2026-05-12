// fun_decl_4.c
static volatile unsigned long long sink;

static unsigned long long mix_u64(unsigned long long x) {
  x ^= x >> 33;
  x *= 0xff51afd7ed558ccdULL;
  x ^= x >> 33;
  x *= 0xc4ceb9fe1a85ec53ULL;
  x ^= x >> 33;
  return x;
}

static unsigned long long hash_matrix(int n, int k, double a[n][k]) {
  unsigned long long h = 0x9e3779b97f4a7c15ULL;
  int i = 0;
  while (i < n) {
    int j = 0;
    while (j < k) {
      union {
        double d;
        unsigned long long u;
      } u;
      u.d = a[i][j];
      h = mix_u64(h ^ (u.u + (unsigned long long)(i * 131 + j * 17)));
      j += 1;
    }
    i += 1;
  }
  return h;
}

void addscalar(int n, int m, double a[n][n * m + 300], double x) {
  int k = n * m + 300;
  int i = 0;
  while (i < n) {
    int j = 0;
    while (j < k) {
      a[i][j] += x;
      j += 1;
    }
    i += 1;
  }
}

int main(void) {
  int n = 4;
  int m = 2;

  int k = n * m + 300;

  double b[4][308];

  int i = 0;
  while (i < n) {
    int j = 0;
    while (j < k) {
      b[i][j] = (double)(i * 1000 + j) * 0.25;
      j += 1;
    }
    i += 1;
  }

  {
    unsigned long long before = hash_matrix(n, k, b);

    addscalar(n, m, b, 2.17);

    unsigned long long after = hash_matrix(n, k, b);

    if (before == after)
      return 1;
  }

  {
    double x = 2.17;

    int i2 = 0;
    while (i2 < n) {
      int j2 = 0;
      while (j2 < k) {
        double expect = (double)(i2 * 1000 + j2) * 0.25 + x;
        double got = b[i2][j2];
        double diff = got - expect;
        if (diff < 0.0)
          diff = -diff;
        if (diff > 0.0000001)
          return 2;
        j2 += 1;
      }
      i2 += 1;
    }
  }

  sink ^= (unsigned long long)b[0][0];
  if (sink == 0ULL)
    return 3;

  return 0;
}
