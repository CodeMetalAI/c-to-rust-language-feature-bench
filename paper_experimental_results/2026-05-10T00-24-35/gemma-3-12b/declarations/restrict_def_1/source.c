// restrict_def_1.c
int *restrict a;
int *restrict b;
extern int c[];

union Bank {
  int left[8];
  int right[8];
};

static union Bank bank_a;
static union Bank bank_b;

int c[8];

static int *choose_view(union Bank *u, int which) {
  if (which)
    return u->right;
  return u->left;
}

static void fill(int *p, int n, int base) {
  int i = 0;
  while (i < n) {
    p[i] = base + i;
    i += 1;
  }
}

static void bump(int *p, int n, int k) {
  int i = 0;
  while (i < n) {
    p[i] += k;
    i += 1;
  }
}

static int sum(int *p, int n) {
  int s = 0;
  int i = 0;
  while (i < n) {
    s += p[i];
    i += 1;
  }
  return s;
}

int main(void) {
  int which_a = 0;
  int which_b = 1;

  a = choose_view(&bank_a, which_a);
  b = choose_view(&bank_b, which_b);

  fill(a, 8, 100);
  fill(b, 8, 200);
  fill(c, 8, 300);

  bump(a, 8, 1);
  bump(b, 8, 2);
  bump(c, 8, 3);

  if (sum(a, 8) != (101 + 102 + 103 + 104 + 105 + 106 + 107 + 108))
    return 1;

  if (sum(b, 8) != (202 + 203 + 204 + 205 + 206 + 207 + 208 + 209))
    return 2;

  if (sum(c, 8) != (303 + 304 + 305 + 306 + 307 + 308 + 309 + 310))
    return 3;

  if (bank_a.left[0] != 101)
    return 4;

  if (bank_b.right[7] != 209)
    return 5;

  if (c[0] != 303 || c[7] != 310)
    return 6;

  return 0;
}
