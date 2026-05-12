/*
 * compound_literal_5
 */

int main(void) {
  const char *s1 = "/tmp/fileXXXXXX";

  char *s2 = (char[]){"/tmp/fileXXXXXX"};

  const char *s3 = (const char[]){"/tmp/fileXXXXXX"};

  if (s1[0] != '/' || s2[0] != '/' || s3[0] != '/')
    return 1;
  if (s1[1] != 't' || s2[1] != 't' || s3[1] != 't')
    return 2;

  s2[0] = 'X';
  if (s2[0] != 'X')
    return 3;

  return 0;
}
