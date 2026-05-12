// type_names.c
int g_arr[3] = {1, 2, 3};

int *fip() { return &g_arr[1]; }

static int f_plain(void) { return 7; }

static int f_var(unsigned int x, ...) { return (int)(x + 3u); }

static int use_vla(int n, int (*p)[n]) { return (*p)[0] * 10 + (*p)[n - 1]; }

int main(void) {
  {
    if (sizeof((int){0}) != sizeof(int))
      return 1;
  }

  {
    int *p = (int *)&g_arr[0];
    if (*(int *)(p) != 1)
      return 2;
  }

  {
    int *q = (int *[3]){&g_arr[0], &g_arr[1], &g_arr[2]}[2];
    if (*q != 3)
      return 3;
  }

  {
    int (*pa)[3] = (int (*)[3]) & g_arr;
    if ((*pa)[1] != 2)
      return 4;
  }

  {
    int n = 3;
    int r = use_vla(n, (int (*)[n]) & g_arr);
    if (r != 13)
      return 5;
  }

  {
    int *(*pf)() = (int *(*)())fip;
    if (*pf() != 2)
      return 6;
  }

  {
    int (*pf)(void) = (int (*)(void))f_plain;
    if (pf() != 7)
      return 7;
  }

  {
    int (*const af[])(unsigned int, ...) = {f_var, f_var};

    if (af[0](4u, 0) != 7)
      return 8;

    if (af[1](10u, 1, 2, 3) != 13)
      return 9;
  }

  return 0;
}
