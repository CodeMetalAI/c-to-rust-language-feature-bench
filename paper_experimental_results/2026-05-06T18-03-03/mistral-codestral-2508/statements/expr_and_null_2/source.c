/* expr_and_null_2.c
 * Test for EXAMPLE 2 from 6.8.3.
 */

int main(void) {
  char *s = "12345";
  int acc = 0;
  while (*s++ != '\0')
    acc++;
  return acc == 5 ? 0 : 1;
}
