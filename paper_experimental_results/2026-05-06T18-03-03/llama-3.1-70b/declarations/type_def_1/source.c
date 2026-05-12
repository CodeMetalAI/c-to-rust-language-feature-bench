// type_def_1.c
typedef int MILES;
typedef int KLICKSP();

typedef struct {
  double hi;
  double lo;
} range;

static int g_store = 7;

static int takes_int(int x) { return x + 1; }

static int takes_ptr_to_int(int *p) {
  *p += 3;
  return *p;
}

static range takes_range(range r) {
  range out;
  out.hi = r.hi + 1.0;
  out.lo = r.lo - 1.0;
  return out;
}

static double takes_range_ptr(range *p) { return p->hi + p->lo; }

int f_plain() { return g_store; }

MILES distance;
KLICKSP *metricp;
range x;
range z, *zp;

int main(void) {
  distance = 40;

  if (takes_int(distance) != 41)
    return 1;

  {
    int t = distance;
    if (t != 40)
      return 2;
    if (takes_ptr_to_int(&t) != 43)
      return 3;
  }

  metricp = f_plain;

  if ((*metricp)() != 7)
    return 4;

  if ((*metricp)(1, 2, 3) != 7)
    return 5;

  x.hi = 1.5;
  x.lo = -1.5;

  {
    range y = takes_range(x);
    if ((y.hi + y.lo) != (x.hi + x.lo))
      return 6;
  }

  z.hi = 10.0;
  z.lo = -9.0;
  zp = &z;

  if (takes_range_ptr(zp) != 1.0)
    return 7;

  {
    range copy = *zp;
    if (copy.hi != 10.0 || copy.lo != -9.0)
      return 8;
  }

  return 0;
}
