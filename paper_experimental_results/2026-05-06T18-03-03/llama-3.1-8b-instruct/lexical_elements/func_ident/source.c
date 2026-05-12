/*
 * func_ident
 */

static int streq(const char *a, const char *b) {
  int i = 0;
  while (a[i] == b[i]) {
    if (a[i] == 0)
      return 1;
    i += 1;
  }
  return 0;
}

static int check_name(const char *got, const char *expect) {
  return streq(got, expect);
}

static int f() {
  if (!check_name(__func__, "f"))
    return -1;
  return 0;
}

static int g() {
  if (!check_name(__func__, "g"))
    return -2;
  return 0;
}

int main(void) {
  if (!check_name(__func__, "main"))
    return 1;
  return f() + g();
}
