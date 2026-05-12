// type_qualif_2.c
const struct s {
  int mem;
} cs = {1};
struct s ncs;

typedef int A[2][3];
const A a = {{4, 5, 6}, {7, 8, 9}};

int use_int(int x) { return x; }

int main(void) {
  int *pi;
  const int *pci;

  ncs = cs;

  pi = &ncs.mem;
  *pi += 10;

  pci = &cs.mem;

  if (use_int(ncs.mem) != 11)
    return 1;

  if (*pci != 1)
    return 2;

  if (a[0][0] != 4)
    return 3;
  if (a[0][1] != 5)
    return 4;
  if (a[0][2] != 6)
    return 5;
  if (a[1][0] != 7)
    return 6;
  if (a[1][1] != 8)
    return 7;
  if (a[1][2] != 9)
    return 8;

  return 0;
}
