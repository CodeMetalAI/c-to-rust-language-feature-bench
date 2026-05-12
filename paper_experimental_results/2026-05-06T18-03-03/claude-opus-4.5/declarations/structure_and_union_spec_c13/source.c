/* structure_and_union_spec_c13.c */

typedef unsigned long uptr;

struct Outer {
  int base;

  struct {
    int a;

    union {
      int u_as_int;

      struct {
        int b;

        union {
          int deep;
          struct {
            int x;
            int y;
          };
        };

        int c;
      };

      unsigned u_as_unsigned;
    };

    int d;
  };

  union {
    int tail_i;

    struct {
      int tail_x;

      union {
        int tail_y;
        struct {
          int tail_p;
          int tail_q;
        };
      };
    };
  };
};

static int check_designated_init(void) {
  struct Outer o = {.base = 10,

                    .a = 1,
                    .b = 2,
                    .deep = 99,
                    .c = 3,
                    .d = 4,

                    .tail_x = 7,
                    .tail_y = 8};

  if (o.base != 10)
    return 1;
  if (o.a != 1)
    return 2;
  if (o.b != 2)
    return 3;
  if (o.deep != 99)
    return 4;
  if (o.c != 3)
    return 5;
  if (o.d != 4)
    return 6;
  if (o.tail_x != 7)
    return 7;
  if (o.tail_y != 8)
    return 8;

  return 0;
}

static int check_union_aliasing_via_flattened_names(void) {
  struct Outer o;
  o.base = 0;

  o.deep = 0x11223344;

  if (o.x != 0x11223344)
    return 20;

  o.x = 5;
  o.y = 6;
  if (o.x != 5)
    return 21;
  if (o.y != 6)
    return 22;

  o.tail_p = 40;
  o.tail_q = 41;
  if (o.tail_p != 40)
    return 23;
  if (o.tail_q != 41)
    return 24;

  o.tail_y = -9;
  if (o.tail_y != -9)
    return 25;

  return 0;
}

static int check_addressability(void) {
  struct Outer o;

  uptr pa = (uptr)(void *)&o.a;
  uptr pb = (uptr)(void *)&o.b;
  uptr pdeep = (uptr)(void *)&o.deep;
  uptr ptx = (uptr)(void *)&o.tail_x;
  uptr pty = (uptr)(void *)&o.tail_y;

  if (pa == 0 || pb == 0 || pdeep == 0 || ptx == 0 || pty == 0)
    return 30;
  if (pa == pb)
    return 31;
  if (ptx == pty)
    return 32;

  return 0;
}

int main(void) {
  int r;

  r = check_designated_init();
  if (r)
    return r;

  r = check_union_aliasing_via_flattened_names();
  if (r)
    return r;

  r = check_addressability();
  if (r)
    return r;

  return 0;
}
