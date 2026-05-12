// initialization_5.c
struct {
  int a[3], b;
} w[] = {{1}, 2};

int main(void) {
  if (sizeof(w) / sizeof(w[0]) != 2)
    return 1;

  if (w[0].a[0] != 1)
    return 2;
  if (w[0].a[1] != 0)
    return 3;
  if (w[0].a[2] != 0)
    return 4;
  if (w[0].b != 0)
    return 5;

  if (w[1].a[0] != 2)
    return 6;
  if (w[1].a[1] != 0)
    return 7;
  if (w[1].a[2] != 0)
    return 8;
  if (w[1].b != 0)
    return 9;

  return 0;
}
