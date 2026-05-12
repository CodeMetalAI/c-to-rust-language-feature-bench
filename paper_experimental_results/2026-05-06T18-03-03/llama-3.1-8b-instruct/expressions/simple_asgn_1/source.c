/*
 * simple_asgn_1
 */

static int f(void) { return -1; }

int main(void) {
  char c;

  int eq = ((c = f()) == -1);

  int char_is_signed = ((char)-1 < 0);

  if (char_is_signed) {

    if (eq != 1)
      return 1;
  } else {
    if (eq != 0)
      return 2;
  }

  return 0;
}
