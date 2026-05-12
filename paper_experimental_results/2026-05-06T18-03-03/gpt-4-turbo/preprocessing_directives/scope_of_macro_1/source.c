/* scope_of_macro_1.c
 */
#define TABSIZE 100

int a[TABSIZE];

#undef TABSIZE

int TABSIZE;

int main(void) {
  if (sizeof(a) / sizeof(a[0]) != 100)
    return 1;
  TABSIZE = 7;
  if (TABSIZE != 7)
    return 2;
  return 0;
}
