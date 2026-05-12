/* combined_features.c
 * Features threaded through a 3x5 int array whose values seed the struct operations:
 *   expressions/array_subscripting  [expr.postfix.subscript]
 *   expressions/struct_and_union_1  [expr.postfix.member]
 *   declarations/type_def_2         [decl.type_defs]
 *
 * Array fill (subscript) and pointer-arithmetic equivalence check match the
 * individual array_subscripting test exactly.  Local struct variables match
 * the individual type_def_2 test exactly -- no array-repeat initialization to
 * cue Copy derivation.  Struct field values are seeded from the int array, so
 * every section depends on every other section being correct.
 */
#include <stddef.h>
#include <stdio.h>

typedef struct s1 { int x; int y; } T1, *Tp1;
typedef struct s2 { int n;        } T2, *Tp2;

static T1  make_t1(int x, int y)  { T1 v = {x, y}; return v; }
static int takes_t1(T1 v)         { return v.x + v.y + 1; }
static int takes_s1(struct s1 v)  { return v.x + v.y + 2; }
static int takes_tp1(Tp1 p)       { p->x += 100; return p->x; }
static int takes_int(int v)       { return v + 4; }
static int takes_t2(T2 v)         { return v.n + 5; }

int main(void) {
  /* [expr.postfix.subscript] 3x5 int array; fill via subscript, verify via pointer arithmetic */
  int x[3][5];
  for (int i = 0; i < 3; ++i)
    for (int j = 0; j < 5; ++j)
      x[i][j] = i + j;
  /* x[0]={0,1,2,3,4}, x[1]={1,2,3,4,5}, x[2]={2,3,4,5,6} */

  for (int i = 0; i < 3; ++i)
    for (int j = 0; j < 5; ++j) {
      int a = x[i][j];
      int b = *(*(x + i) + j);
      if (a != b) return 1;
    }

  int *p0 = &x[0][0];
  int *p1 = &x[1][0];
  if ((ptrdiff_t)(p1 - p0) != 5) return 2;

  /* [expr.postfix.member] rvalue member access on returned struct, args from array */
  if (make_t1(x[1][0], x[1][1]).x != x[1][0]) return 3;  /* .x == 1 */
  if (make_t1(x[1][0], x[1][1]).y != x[1][1]) return 4;  /* .y == 2 */
  if (make_t1(x[0][3], x[1][2]).x +
      make_t1(x[2][0], x[0][4]).y   != 7)      return 5;
  /* x[0][3]=3, x[1][2]=3, x[2][0]=2, x[0][4]=4: make_t1(3,3).x + make_t1(2,4).y = 3+4=7 */

  /* [decl.type_defs] local struct variables seeded from array;
     same variable passed to typedef-named and struct-tag-named params --
     requires Copy on T1/struct s1 in correct Rust translation */
  T1 a;
  struct s1 b;
  Tp1 tp;
  a.x = x[1][0]; a.y = x[1][1];   /* {1, 2} */
  b.x = x[2][0]; b.y = x[2][1];   /* {2, 3} */

  if (takes_t1(a) != 4)  return 6;   /* 1+2+1=4,  T1 param */
  if (takes_s1(a) != 5)  return 7;   /* 1+2+2=5,  struct s1 param, same var a */
  if (takes_t1(b) != 6)  return 8;   /* 2+3+1=6,  T1 param */
  if (takes_s1(b) != 7)  return 9;   /* 2+3+2=7,  struct s1 param, same var b */

  tp = &a;
  if (takes_tp1(tp) != 101) return 10;   /* a.x: 1+100=101 */
  if (a.x != 101)           return 11;   /* aliasing: original reflects mutation */
  if (takes_int(a.x) != 105) return 12;  /* 101+4=105 */

  {
    int q = 0;
    q += (int)sizeof(T1);
    q += (int)sizeof(struct s1);
    if (q == 0) return 13;
  }

  {
    T2 c;
    Tp2 r;
    c.n = x[2][0];               /* = 2, seeded from array */
    r = &c;
    if (r->n != x[2][0]) return 14;   /* = 2 */
    if (takes_t2(c) != 7) return 15;  /* 2+5=7 */
  }

  printf("PASS\n");
  return 0;
}
