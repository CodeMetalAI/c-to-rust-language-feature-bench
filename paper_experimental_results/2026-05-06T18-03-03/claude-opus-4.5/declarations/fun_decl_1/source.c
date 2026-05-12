// fun_decl_1.c
static int gstore[4];

int f(void) {
  gstore[0] = 111;
  return gstore[0] + 1;
}

int *fip() {
  gstore[1] = 222;
  return &gstore[1];
}

static int alt0(void) {
  gstore[2] = 333;
  return gstore[2] - 1;
}

static int alt1(void) {
  gstore[3] = 444;
  return gstore[3] + 2;
}

static int use_call_through(int (*pf)()) {
  int r = (*pf)();
  return r;
}

static int choose(int x) {
  if (x & 1)
    return 1;
  return 0;
}

int main(void) {
  int (*pfi)();

  int r_f = f();
  if (r_f != 112)
    return 1;

  int v_fip = *fip();
  if (v_fip != 222)
    return 2;

  pfi = alt0;
  if (choose(gstore[0]) == 0)
    pfi = alt1;

  {
    int r_pfi = (*pfi)();
    int r_use = use_call_through(pfi);

    if (r_pfi != r_use)
      return 3;

    if (pfi == alt0) {
      if (r_pfi != 332)
        return 4;
    } else {
      if (r_pfi != 446)
        return 5;
    }
  }

  {
    int q = (*pfi)();
    if (q == 0)
      return 6;
  }

  return 0;
}
