// type_def_3.c
typedef signed int t;
typedef int plain;

struct tag {
  unsigned t : 4;
  const t : 5;
  plain r : 5;
};

static t id(t x) { return x; }

static t call_cb(t (*cb)(t), t x) { return cb(x); }

static t use_bits(struct tag s) {
  t a = (t)s.t;
  t b = (t)s.r;
  return (t)(a * 100 + b);
}

t f(t(t));

int main(void) {
  struct tag s;

  s.t = 15u;
  s.r = 31;

  if (use_bits(s) != (t)(15 * 100 + 31))
    return 1;

  {
    t f(t(t));
    long t;

    t = 1234L;

    if (t != 1234L)
      return 2;

    if (call_cb(id, 7) != 7)
      return 3;

    if (f(id) != 9)
      return 4;
  }

  return 0;
}

t f(t (*pf)(t)) { return call_cb(pf, 9); }
