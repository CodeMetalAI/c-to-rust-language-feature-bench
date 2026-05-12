// initialization_8.c
int main(void) {
  char s[] = "abc";
  char t[3] = "abc";

  if (s[0] != 'a' || s[1] != 'b' || s[2] != 'c' || s[3] != '\0')
    return 1;

  if (t[0] != 'a' || t[1] != 'b' || t[2] != 'c')
    return 2;

  s[0] = 'x';
  t[2] = 'y';

  if (s[0] != 'x')
    return 3;
  if (t[2] != 'y')
    return 4;

  {
    char *p = "abc";
    if (p[0] != 'a' || p[1] != 'b' || p[2] != 'c' || p[3] != '\0')
      return 5;
  }

  return 0;
}
