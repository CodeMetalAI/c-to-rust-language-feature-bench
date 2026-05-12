/* expr_and_null_3.c
 * Test for EXAMPLE 3 from 6.8.3.
 */

int main(void) {
  int x = 0;

  {
    goto end;
    x = 1;
  end:;
  }

  x++;
  return (x == 1) ? 0 : 1;
}
