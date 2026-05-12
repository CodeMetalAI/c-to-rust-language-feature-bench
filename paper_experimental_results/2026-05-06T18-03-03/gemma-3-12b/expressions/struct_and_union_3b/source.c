/*
 * struct_and_union_3b
 */

struct t1 {
  int m;
};
struct t2 {
  int m;
};

union u12 {
  struct t1 s1;
  struct t2 s2;
};

static int f(union u12 *u) {
  if (u->s1.m < 0)
    u->s2.m = -u->s2.m;
  return u->s1.m;
}

int main(void) {
  union u12 u;

  u.s1.m = -7;
  u.s2.m = 7;

  if (f(&u) != -7)
    return 1;
  if (u.s2.m != -7)
    return 2;

  return 0;
}
