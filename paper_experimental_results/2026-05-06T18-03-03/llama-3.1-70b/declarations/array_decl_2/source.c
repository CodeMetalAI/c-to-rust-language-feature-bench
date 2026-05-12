// array_decl_2.c
typedef unsigned long usize;

extern int *x;
extern int y[];

int y[7];

static int backing[3] = {10, 20, 30};
int *x = backing;

#define OFFSETOF(type, member) ((usize) & (((type *)0)->member))

struct HoldP {
  int *p;
};

struct HoldA {
  int tag;
  int a[];
};

static int sum_ptr(const int *p, int n) {
  int s = 0;
  int i = 0;
  while (i < n) {
    s += p[i];
    i += 1;
  }
  return s;
}

static int sum_arr7(const int a[7]) {
  int s = 0;
  int i = 0;
  while (i < 7) {
    s += a[i];
    i += 1;
  }
  return s;
}

static int mutate_through_pointer(int *p) {
  p[1] += 5;
  return p[0] + p[1] + p[2];
}

int main(void) {
  y[0] = 1;
  y[1] = 2;
  y[2] = 3;
  y[3] = 4;
  y[4] = 5;
  y[5] = 6;
  y[6] = 7;

  if (sum_arr7(y) != (1 + 2 + 3 + 4 + 5 + 6 + 7))
    return 1;

  if (sum_ptr(x, 3) != (10 + 20 + 30))
    return 2;

  if (mutate_through_pointer(x) != (10 + 25 + 30))
    return 3;

  if (backing[1] != 25)
    return 4;

  {
    int *t = y;
    if (t[6] != 7)
      return 5;
  }

  {
    struct HoldP hp;
    struct HoldA *ha;

    hp.p = y;
    if (hp.p[0] != 1)
      return 6;

    ha = (struct HoldA *)(void *)y;

    if ((unsigned char *)(void *)&ha->a[0] !=
        (unsigned char *)(void *)ha + OFFSETOF(struct HoldA, a))
      return 7;

    if (ha->a[2] != 3)
      return 8;
  }

  if (y[0] != 1)
    return 9;

  return 0;
}
