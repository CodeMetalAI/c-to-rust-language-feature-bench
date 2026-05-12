/* scope_of_macro_7.c
 */
#define PP_CAT(a, b) a##b
#define PP_XCAT(a, b) PP_CAT(a, b)

#define PP_ARG_N(_1, _2, _3, _4, _5, _6, _7, _8, _9, _10, N, ...) N
#define PP_RSEQ_N() 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0
#define PP_NARG_(...) PP_ARG_N(__VA_ARGS__)
#define PP_NARG(...) PP_NARG_(__VA_ARGS__, PP_RSEQ_N())

#define debug(...) PP_NARG(__VA_ARGS__)
#define showlist(...) #__VA_ARGS__
#define report(test, ...) ((test) ? 1 : PP_NARG(__VA_ARGS__))

static int streq(const char *a, const char *b) {
  while (*a && *b) {
    if (*a != *b)
      return 0;
    a++;
    b++;
  }
  return *a == *b;
}

int main(void) {
  int x = 7;
  int y = 3;

  if (debug("Flag") != 1)
    return 1;
  if (debug("X = %d\n", x) != 2)
    return 2;

  if (!streq(showlist(The first, second, and third items.),
             "The first, second, and third items."))
    return 3;

  if (report(x > y, "x is %d but y is %d", x, y) != 1)
    return 4;
  if (report(x < y, "x is %d but y is %d", x, y) != 3)
    return 5;

  return 0;
}
