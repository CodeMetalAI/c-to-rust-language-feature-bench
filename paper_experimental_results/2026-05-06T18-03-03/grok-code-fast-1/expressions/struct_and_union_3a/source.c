/*
 * struct_and_union_3a
 */

union U {
  struct {
    int alltypes;
  } n;
  struct {
    int type;
    int intnode;
  } ni;
  struct {
    int type;
    double doublenode;
  } nf;
};

int main(void) {
  union U u;

  u.nf.type = 1;

  u.nf.doublenode = 3.14;

  if (u.n.alltypes != 1)
    return 1;
  if (u.ni.type != 1)
    return 1;
  if (u.nf.type != 1)
    return 2;
  if (u.nf.doublenode != 3.14)
    return 3;

  return 0;
}
