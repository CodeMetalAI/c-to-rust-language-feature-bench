/*
 * compound_literal_6
 */

int main(void) {
  const char *a = (const char[]){"abc"};
  const char *b = "abc";

  if (a[0] != 'a' || a[1] != 'b' || a[2] != 'c' || a[3] != '\0')
    return 1;
  if (b[0] != 'a' || b[1] != 'b' || b[2] != 'c' || b[3] != '\0')
    return 2;

  if ((a == b) != 0 && (a == b) != 1)
    return 3;

  return 0;
}
