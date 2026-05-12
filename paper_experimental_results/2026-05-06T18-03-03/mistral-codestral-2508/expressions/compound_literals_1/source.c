/*
 * compound_literals_1
 */

struct pair {
  int a;
  int b;
};

int main(void) {
  int *p = (int[]){10, 20, 30};
  if (p[0] != 10)
    return 1;
  if (p[1] != 20)
    return 2;
  if (p[2] != 30)
    return 3;

  struct pair q = (struct pair){7, 11};
  if (q.a != 7)
    return 4;
  if (q.b != 11)
    return 5;

  if (((int[]){1, 2, 3})[1] != 2)
    return 6;

  return 0;
}
