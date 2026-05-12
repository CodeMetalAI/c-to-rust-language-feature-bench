/* simple_asgn_3.c
 */

int main(void) {
  const char **cpp;
  char *p;
  const char c = 'A';

  cpp = (const char **)&p;
  *cpp = &c;
  *p = 0;

  if (c != 'A')
    return 1;

  return 0;
}
