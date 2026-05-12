// fun_decl_3.c
static int g_seed;

static int cb0(long x) { return (int)(x ^ 0x13579L) + 7; }

static int cb1(long x) { return (int)(x * 3L) - 11; }

static int v0(int x, ...) { return g_seed + x + 1000; }

static int v1(int x, ...) { return g_seed + x - 2000; }

int (*fpfi(int (*pf)(long), int k))(int, ...) {
  long t = (long)k * 97L + 1234L;
  g_seed = pf(t) + k;
  if ((g_seed & 1) == 0)
    return v0;
  return v1;
}

static int call_through(int (*pf)(int, ...), int x) {
  int r1 = pf(x);
  int r2 = pf(x, 1, 2, 3);
  int r3 = pf(x, 1.25, (void *)pf, (long long)0x1122334455667788LL);
  return r1 ^ (r2 + 5) ^ (r3 + 9);
}

int main(void) {
  int (*vf)(int, ...);

  vf = fpfi(cb0, 17);

  {
    int expected_seed = cb0((long)17 * 97L + 1234L) + 17;
    int base = expected_seed + 40;

    if (vf == v0) {
      if (vf(40) != base + 1000)
        return 1;
      if (vf(40, 1, 2) != base + 1000)
        return 2;
      if (call_through(vf, 40) !=
          ((base + 1000) ^ (base + 1000 + 5) ^ (base + 1000 + 9)))
        return 3;
    } else {
      if (vf(40) != base - 2000)
        return 4;
      if (vf(40, 1, 2) != base - 2000)
        return 5;
      if (call_through(vf, 40) !=
          ((base - 2000) ^ (base - 2000 + 5) ^ (base - 2000 + 9)))
        return 6;
    }
  }

  vf = fpfi(cb1, 8);

  {
    int expected_seed = cb1((long)8 * 97L + 1234L) + 8;
    int base = expected_seed + 9;

    if (vf == v0) {
      if (vf(9) != base + 1000)
        return 7;
      if (vf(9, 0, 0, 0, 0) != base + 1000)
        return 8;
      if (call_through(vf, 9) !=
          ((base + 1000) ^ (base + 1000 + 5) ^ (base + 1000 + 9)))
        return 9;
    } else {
      if (vf(9) != base - 2000)
        return 10;
      if (vf(9, 0, 0, 0, 0) != base - 2000)
        return 11;
      if (call_through(vf, 9) !=
          ((base - 2000) ^ (base - 2000 + 5) ^ (base - 2000 + 9)))
        return 12;
    }
  }

  return 0;
}
