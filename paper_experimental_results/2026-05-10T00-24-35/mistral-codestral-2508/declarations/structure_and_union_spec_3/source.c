/* structure_and_union_spec_3
 */

#include <stddef.h>
#include <stdlib.h>

struct s {
  struct {
    int i;
  };
  int a[];
};

int main(void) {
  struct s *p;

  if (offsetof(struct s, i) != 0)
    return 1;

  if (offsetof(struct s, a) != sizeof(struct s))
    return 2;

  p = malloc(sizeof(struct s) + 4 * sizeof(int));
  if (!p)
    return 3;

  p->i = 7;
  p->a[0] = 11;
  p->a[3] = 22;

  if (p->i != 7 || p->a[0] != 11 || p->a[3] != 22)
    return 4;

  free(p);
  return 0;
}
