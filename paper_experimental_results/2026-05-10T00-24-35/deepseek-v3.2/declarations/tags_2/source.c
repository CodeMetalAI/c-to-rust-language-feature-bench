/*
 * C11 6.7.2.3 — Tags (Example 2)
 */

#include <stddef.h>

struct s2;

struct s1 {
  int v1;
  struct s2 *s2p;
};

struct s2 {
  int v2;
  struct s1 *s1p;
};

int main(void) {
  struct s1 a;
  struct s2 b;

  a.v1 = 10;
  b.v2 = 20;

  a.s2p = &b;
  b.s1p = &a;

  if (a.s2p->v2 != 20)
    return 1;

  if (b.s1p->v1 != 10)
    return 2;

  if (a.s2p->s1p != &a)
    return 3;

  return 0;
}
