/*
structure_and_union_spec_1.c
*/

int main() {
  struct v {
    union {
      struct {
        int i, j;
      };
      struct {
        long k, l;
      } w;
    };
    int m;
  } v1;
  v1.i = 2;
  v1.w.k = 5;

  if (v1.i != 2)
    return 1;

  if (v1.w.k != 5)
    return 1;

  return 0;
}
