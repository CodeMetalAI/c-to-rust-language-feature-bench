/* simple_asgn_2.c
 */

int main(void) {

  char c;
  int i;
  long l;

  i = 100;

  l = (c = i);

  if (c != (char)i)
    return 1;
  if (l != (long)c)
    return 2;
  if (l != (long)(char)i)
    return 3;

  return 0;
}
