// initialization_9.c
enum { member_one, member_two };

static int check(const char *a, const char *b) { return a[0] == b[0]; }

int main(void) {
  const char *nm[] = {
      [member_two] = "member two",
      [member_one] = "member one",
  };

  if (!check(nm[member_one], "member one"))
    return 1;
  if (!check(nm[member_two], "member two"))
    return 2;

  {
    const char **p = nm;
    if (p[member_one][7] != 'o')
      return 3;
    if (p[member_two][7] != 't')
      return 4;
  }

  return 0;
}
