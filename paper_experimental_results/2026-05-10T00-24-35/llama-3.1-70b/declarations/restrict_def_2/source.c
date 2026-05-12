// restrict_def_2.c
volatile int g_off = 9;

void f(int n, int *restrict p, int *restrict q) {
  while (n-- > 0)
    *p++ = *q++;
}

static int check_range_eq(const int *a, const int *b, int n) {
  int i = 0;
  while (i < n) {
    if (a[i] != b[i])
      return 0;
    i += 1;
  }
  return 1;
}

int main(void) {
  int buf[20];
  int snapshot[20];

  int i = 0;
  while (i < 20) {
    buf[i] = i * 11 + 3;
    snapshot[i] = buf[i];
    i += 1;
  }

  {
    int off = g_off;
    int n = 8;

    int *p = &buf[0];
    int *q = &buf[off];

    f(n, p, q);

    if (!check_range_eq(&buf[0], &snapshot[off], n))
      return 1;

    if (!check_range_eq(&buf[off], &snapshot[off], n))
      return 2;

    if (buf[off - 1] != snapshot[off - 1])
      return 3;

    if (buf[off + n] != snapshot[off + n])
      return 4;
  }

  {
    int off = g_off;
    int n = 8;

    int *p = &buf[off];
    int *q = &buf[0];

    f(n, p, q);

    if (!check_range_eq(&buf[off], &buf[0], n))
      return 5;
  }

  return 0;
}
