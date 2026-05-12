/* scope_of_macro_4.c
 */

#define str(s) #s
#define xstr(s) str(s)
#define INCFILE(n) vers##n
#define glue(a, b) a##b
#define xglue(a, b) glue(a, b)
#define HIGHLOW "hello"
#define LOW LOW ", world"
#define dbgfmt(s, t) "x" #s "= %d, x" #t "= %s"

int x1 = 7;
char x2[] = "hi";

char fmt[] = dbgfmt(1, 2);
char inc[] = xstr(INCFILE(2).h);
char a[] = glue(HIGH, LOW);
char b[] = xglue(HIGH, LOW);

int main(void) {
  if (x1 != 7)
    return 1;
  if (x2[0] != 'h')
    return 2;
  if (x2[1] != 'i')
    return 3;
  if (x2[2] != '\0')
    return 4;

  if (sizeof(fmt) != 15)
    return 5;
  if (fmt[0] != 'x')
    return 6;
  if (fmt[1] != '1')
    return 7;
  if (fmt[2] != '=')
    return 8;
  if (fmt[3] != ' ')
    return 9;
  if (fmt[4] != '%')
    return 10;
  if (fmt[5] != 'd')
    return 11;
  if (fmt[6] != ',')
    return 12;
  if (fmt[7] != ' ')
    return 13;
  if (fmt[8] != 'x')
    return 14;
  if (fmt[9] != '2')
    return 15;
  if (fmt[10] != '=')
    return 16;
  if (fmt[11] != ' ')
    return 17;
  if (fmt[12] != '%')
    return 18;
  if (fmt[13] != 's')
    return 19;
  if (fmt[14] != '\0')
    return 20;

  if (sizeof(inc) != 8)
    return 21;
  if (inc[0] != 'v')
    return 22;
  if (inc[1] != 'e')
    return 23;
  if (inc[2] != 'r')
    return 24;
  if (inc[3] != 's')
    return 25;
  if (inc[4] != '2')
    return 26;
  if (inc[5] != '.')
    return 27;
  if (inc[6] != 'h')
    return 28;
  if (inc[7] != '\0')
    return 29;

  if (sizeof(a) != 6)
    return 30;
  if (a[0] != 'h')
    return 31;
  if (a[1] != 'e')
    return 32;
  if (a[2] != 'l')
    return 33;
  if (a[3] != 'l')
    return 34;
  if (a[4] != 'o')
    return 35;
  if (a[5] != '\0')
    return 36;

  if (sizeof(b) != 13)
    return 37;
  if (b[0] != 'h')
    return 38;
  if (b[1] != 'e')
    return 39;
  if (b[2] != 'l')
    return 40;
  if (b[3] != 'l')
    return 41;
  if (b[4] != 'o')
    return 42;
  if (b[5] != ',')
    return 43;
  if (b[6] != ' ')
    return 44;
  if (b[7] != 'w')
    return 45;
  if (b[8] != 'o')
    return 46;
  if (b[9] != 'r')
    return 47;
  if (b[10] != 'l')
    return 48;
  if (b[11] != 'd')
    return 49;
  if (b[12] != '\0')
    return 50;

  return 0;
}
