/*
 * compound_literal_3
 */

struct point {
  int x;
  int y;
};

static int foo_val(struct point a, struct point b) {
  if (a.x != 1)
    return 1;
  if (a.y != 1)
    return 2;
  if (b.x != 3)
    return 3;
  if (b.y != 4)
    return 4;
  return 0;
}

static int bar_ptr(const struct point *a, const struct point *b) {
  if (a->x != 1)
    return 5;
  if (a->y != 1)
    return 6;
  if (b->x != 3)
    return 7;
  if (b->y != 4)
    return 8;
  return 0;
}

int main(void) {
  if (foo_val((struct point){.x = 1, .y = 1}, (struct point){.x = 3, .y = 4}) !=
      0)
    return 10;

  if (bar_ptr(&(struct point){.x = 1, .y = 1},
              &(struct point){.x = 3, .y = 4}) != 0)
    return 11;

  return 0;
}
