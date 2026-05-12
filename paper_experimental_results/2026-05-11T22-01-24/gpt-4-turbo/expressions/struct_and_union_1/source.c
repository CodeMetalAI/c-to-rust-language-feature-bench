/*
 * struct_and_union_1
 */

typedef struct {
  int x;
  int y;
} S;

static S f(void) {
  S s = {7, 11};
  return s;
}

int main(void) {
  int vx = f().x;
  int vy = f().y;

  if (vx != 7)
    return 1;
  if (vy != 11)
    return 2;

  if ((f().x + f().y) != 18)
    return 3;

  return 0;
}
