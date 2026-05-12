/* lvalue_array_fun_c1.c */
typedef unsigned long size_t;

#define OK_MOD(x) (sizeof((x) = (x)) > 0)

struct S1 {
  int a;
  const int b;
};
struct S2 {
  struct S1 s;
};
struct S3 {
  int a;
};

int main(void) {
  int i = 1;
  int *p = &i;

  if (!OK_MOD(i))
    return 1;
  if (!OK_MOD(*p))
    return 2;

  int a[2] = {1, 2};
  (void)a;
  (void)sizeof(a);

  const int ci = 3;
  (void)ci;

  struct S3 s3 = {4};
  if (!OK_MOD(s3.a))
    return 3;

  struct S1 s1 = {5, 6};
  struct S2 s2 = {{7, 8}};
  (void)s1;
  (void)s2;

  {
    int t = 9;
    unsigned char *bp = (unsigned char *)&t;
    bp[0] = bp[0];
    t = t;
    if (t != t)
      return 4;
  }

  return 0;
}
