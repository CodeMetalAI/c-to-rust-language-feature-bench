/* scope_of_macro_3.c
 */

#define x 3
#define F(a) ((a) + x)
#undef x
#define x 2

#define t(a) a
#define g F
#define q(x) x
#define p() int
#define r(x, y) x##y
#define str(x) #x

int v = F(10);
int u = F((3, 4));
int w = t(g)(0);

p() i[q()] = {q(1), r(2, 3), r(4, ), r(, 5), r(, )};
char c[2][6] = {str(hello), str()};

int main(void) {
  if (v != 12)
    return 1;
  if (u != 6)
    return 2;
  if (w != 2)
    return 3;

  if (i[0] != 1)
    return 4;
  if (i[1] != 23)
    return 5;
  if (i[2] != 4)
    return 6;
  if (i[3] != 5)
    return 7;

  if (c[0][0] != 'h')
    return 8;
  if (c[0][1] != 'e')
    return 9;
  if (c[0][2] != 'l')
    return 10;
  if (c[0][3] != 'l')
    return 11;
  if (c[0][4] != 'o')
    return 12;
  if (c[0][5] != '\0')
    return 13;

  if (c[1][0] != '\0')
    return 14;

  return 0;
}
