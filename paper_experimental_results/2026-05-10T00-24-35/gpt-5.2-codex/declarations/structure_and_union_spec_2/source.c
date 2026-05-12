/* structure_and_union_spec_2.c
 */

#include <stddef.h>
#include <stdlib.h>

int main(void) {
  struct s {
    int n;
    double d[];
  };

  struct ss {
    int n;
  };

  if (sizeof(struct s) < sizeof(struct ss))
    return 1;

  if (offsetof(struct s, d) != sizeof(struct s))
    return 1;

  struct s *s1 = malloc(sizeof(struct s) + sizeof(double) * 8);
  struct s *s2 = malloc(sizeof(struct s) + sizeof(double) * 5);

  if (!s1 || !s2)
    return 1;

  s1->d[0] = 42.0;
  s2->d[0] = 24.0;

  if (s1->d[0] != 42.0 || s2->d[0] != 24.0)
    return 1;

  free(s1);
  free(s2);

  return 0;
}
