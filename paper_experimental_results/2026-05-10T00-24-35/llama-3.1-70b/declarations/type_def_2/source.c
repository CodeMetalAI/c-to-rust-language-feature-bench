// type_def_2.c
typedef struct s1 {
  int x;
} t1, *tp1;
typedef struct s2 {
  int x;
} t2, *tp2;

static int takes_t1(t1 v) { return v.x + 1; }

static int takes_s1(struct s1 v) { return v.x + 2; }

static int takes_tp1(tp1 p) {
  p->x += 3;
  return p->x;
}

static int takes_int(int v) { return v + 4; }

static int takes_t2(t2 v) { return v.x + 5; }

int main(void) {
  t1 a;
  struct s1 b;
  tp1 p;

  a.x = 10;
  b.x = 20;

  if (takes_t1(a) != 11)
    return 1;

  if (takes_s1(a) != 12)
    return 2;

  if (takes_t1(b) != 21)
    return 3;

  if (takes_s1(b) != 22)
    return 4;

  p = &a;

  if (takes_tp1(p) != 13)
    return 5;

  if (a.x != 13)
    return 6;

  if (takes_int(a.x) != 17)
    return 7;

  {
    int q = 0;
    q += (int)sizeof(t1);
    q += (int)sizeof(struct s1);
    if (q == 0)
      return 8;
  }

  {
    t2 c;
    tp2 r;
    c.x = 30;
    r = &c;
    if (r->x != 30)
      return 9;
    if (takes_t2(c) != 35)
      return 10;
  }

  return 0;
}
