/* pathfinder.c
 * Features threaded through a 3x5 int array whose values seed the struct
 * operations:
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

typedef struct coord    { int row; int col; } Coord,    *CoordPtr;
typedef struct waypoint { int cost;         } Waypoint, *WaypointPtr;

static Coord make_coord(int row, int col)  { Coord v = {row, col}; return v; }
static int   coord_sum(Coord v)            { return v.row + v.col + 1; }
static int   coord_sum_s(struct coord v)   { return v.row + v.col + 2; }
static int   shift_row(CoordPtr p)         { p->row += 100; return p->row; }
static int   padded(int v)                 { return v + 4; }
static int   entry_cost(Waypoint v)        { return v.cost + 5; }

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
  if (make_coord(x[1][0], x[1][1]).row != x[1][0]) return 3;  /* .row == 1 */
  if (make_coord(x[1][0], x[1][1]).col != x[1][1]) return 4;  /* .col == 2 */
  if (make_coord(x[0][3], x[1][2]).row +
      make_coord(x[2][0], x[0][4]).col   != 7)      return 5;
  /* x[0][3]=3, x[1][2]=3, x[2][0]=2, x[0][4]=4:
     make_coord(3,3).row + make_coord(2,4).col = 3+4=7 */

  /* [decl.type_defs] local struct variables seeded from array;
     same variable passed to typedef-named and struct-tag-named params --
     requires Copy on Coord/struct coord in correct Rust translation */
  Coord        start;
  struct coord goal;
  CoordPtr     cursor;
  start.row = x[1][0]; start.col = x[1][1];  /* {1, 2} */
  goal.row  = x[2][0]; goal.col  = x[2][1];  /* {2, 3} */

  if (coord_sum(start)   != 4)  return 6;   /* 1+2+1=4,  Coord param */
  if (coord_sum_s(start) != 5)  return 7;   /* 1+2+2=5,  struct coord param, same var start */
  if (coord_sum(goal)    != 6)  return 8;   /* 2+3+1=6,  Coord param */
  if (coord_sum_s(goal)  != 7)  return 9;   /* 2+3+2=7,  struct coord param, same var goal */

  cursor = &start;
  if (shift_row(cursor) != 101) return 10;  /* start.row: 1+100=101 */
  if (start.row         != 101) return 11;  /* aliasing: original reflects mutation */
  if (padded(start.row) != 105) return 12;  /* 101+4=105 */

  {
    int q = 0;
    q += (int)sizeof(Coord);
    q += (int)sizeof(struct coord);
    if (q == 0) return 13;
  }

  {
    Waypoint    wp;
    WaypointPtr wp_ptr;
    wp.cost = x[2][0];               /* = 2, seeded from array */
    wp_ptr = &wp;
    if (wp_ptr->cost != x[2][0]) return 14;  /* = 2 */
    if (entry_cost(wp) != 7)     return 15;  /* 2+5=7 */
  }

  printf("PASS\n");
  return 0;
}
