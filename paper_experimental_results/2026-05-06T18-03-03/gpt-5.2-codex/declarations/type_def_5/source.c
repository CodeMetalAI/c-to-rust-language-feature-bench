// type_def_5.c
static volatile int sink;

void copyt(int n) {
  typedef int B[n];

  n += 1;

  B a;
  int b[n];

  int i = 0;
  while (i < n) {
    b[i] = 1000 + i;
    i += 1;
  }

  i = 1;
  while (i < n) {
    a[i - 1] = b[i];
    i += 1;
  }

  if ((int)(sizeof(a) / sizeof(a[0])) != (n - 1))
    sink = 1;
  if ((int)(sizeof(b) / sizeof(b[0])) != n)
    sink = 2;

  if (a[0] != 1001)
    sink = 3;
  if (a[n - 2] != 1000 + (n - 1))
    sink = 4;
}

int main(void) {
  sink = 0;
  copyt(7);
  if (sink != 0)
    return 1;
  return 0;
}
