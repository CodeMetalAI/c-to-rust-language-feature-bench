#define CAT(a, b) a##b
#define XCAT(a, b) CAT(a, b)
#define CAT3(a, b, c) XCAT(XCAT(a, b), c)

#define D0 0
#define D1 1
#define D2 2
#define D3 3
#define D4 4
#define D5 5
#define D6 6
#define D7 7
#define D8 8
#define D9 9

#define PICKD(n) XCAT(D, n)

#define VXYZ(a, b, c) CAT3(PICKD(a), PICKD(b), PICKD(c))
#define VYZ(b, c) XCAT(PICKD(b), PICKD(c))
#define VXZ(a, c) XCAT(PICKD(a), PICKD(c))
#define VXY(a, b) XCAT(PICKD(a), PICKD(b))

static unsigned rng_state = 2463534242u;
static unsigned rng_next(void) {
  unsigned x = rng_state;
  x ^= x << 13;
  x ^= x >> 17;
  x ^= x << 5;
  rng_state = x;
  return x;
}

static int pick_1_9(void) { return (int)(rng_next() % 9u) + 1; }

static int pick_0_9(void) { return (int)(rng_next() % 10u); }

static int val_xyz[9][10][10] = {
#define ROWC(a, b)                                                             \
  {VXYZ(a, b, 0), VXYZ(a, b, 1), VXYZ(a, b, 2), VXYZ(a, b, 3), VXYZ(a, b, 4),  \
   VXYZ(a, b, 5), VXYZ(a, b, 6), VXYZ(a, b, 7), VXYZ(a, b, 8), VXYZ(a, b, 9)}
#define ROWB(a)                                                                \
  {ROWC(a, 0), ROWC(a, 1), ROWC(a, 2), ROWC(a, 3), ROWC(a, 4),                 \
   ROWC(a, 5), ROWC(a, 6), ROWC(a, 7), ROWC(a, 8), ROWC(a, 9)}
    ROWB(1), ROWB(2), ROWB(3), ROWB(4), ROWB(5),
    ROWB(6), ROWB(7), ROWB(8), ROWB(9)
#undef ROWB
#undef ROWC
};

static int val_yz[9][10] = {
#define ROWY(b)                                                                \
  {VYZ(b, 0), VYZ(b, 1), VYZ(b, 2), VYZ(b, 3), VYZ(b, 4),                      \
   VYZ(b, 5), VYZ(b, 6), VYZ(b, 7), VYZ(b, 8), VYZ(b, 9)}
    ROWY(1), ROWY(2), ROWY(3), ROWY(4), ROWY(5),
    ROWY(6), ROWY(7), ROWY(8), ROWY(9)
#undef ROWY
};

static int val_xz[9][10] = {
#define ROWX(a)                                                                \
  {VXZ(a, 0), VXZ(a, 1), VXZ(a, 2), VXZ(a, 3), VXZ(a, 4),                      \
   VXZ(a, 5), VXZ(a, 6), VXZ(a, 7), VXZ(a, 8), VXZ(a, 9)}
    ROWX(1), ROWX(2), ROWX(3), ROWX(4), ROWX(5),
    ROWX(6), ROWX(7), ROWX(8), ROWX(9)
#undef ROWX
};

static int val_xy[9][10] = {
#define ROWX2(a)                                                               \
  {VXY(a, 0), VXY(a, 1), VXY(a, 2), VXY(a, 3), VXY(a, 4),                      \
   VXY(a, 5), VXY(a, 6), VXY(a, 7), VXY(a, 8), VXY(a, 9)}
    ROWX2(1), ROWX2(2), ROWX2(3), ROWX2(4), ROWX2(5),
    ROWX2(6), ROWX2(7), ROWX2(8), ROWX2(9)
#undef ROWX2
};

int main(void) {
  for (int it = 0; it < 50; it++) {
    int a = pick_1_9();
    int b = pick_0_9();
    int c = pick_0_9();

    int v1 = val_xyz[a - 1][b][c];
    if (v1 != 100 * a + 10 * b + c)
      return 1;

    int b2 = pick_1_9();
    int c2 = pick_0_9();
    int v2 = val_yz[b2 - 1][c2];
    if (v2 != 10 * b2 + c2)
      return 2;

    int a3 = pick_1_9();
    int c3 = pick_0_9();
    int v3 = val_xz[a3 - 1][c3];
    if (v3 != 10 * a3 + c3)
      return 3;

    int a4 = pick_1_9();
    int b4 = pick_0_9();
    int v4 = val_xy[a4 - 1][b4];
    if (v4 != 10 * a4 + b4)
      return 4;
  }

  return 0;
}
