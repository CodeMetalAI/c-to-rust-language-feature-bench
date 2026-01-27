// initialization_13.c
union u {
  int any_member;
  unsigned u_member;
  unsigned char bytes[4];
};

int main(void) {
  union u x = {.any_member = 42};

  if (x.any_member != 42)
    return 1;

  {
    union u y = {.u_member = 7u};
    if (y.u_member != 7u)
      return 2;
  }

  return 0;
}
