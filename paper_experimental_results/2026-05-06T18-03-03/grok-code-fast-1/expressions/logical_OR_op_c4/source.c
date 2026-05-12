/*
 * logical_OR_op_c4
 */

static int g = 0;

static int set_g_return(int gval, int ret) {
  g = gval;
  return ret;
}

int main(void) {
  int y = 0;

  g = 0;
  if (!(set_g_return(1, 0) || (g == 1)))
    return 1;

  g = 0;
  if (set_g_return(0, 1) || (++y))
    return 2;
  if (y != 0)
    return 3;

  return 0;
}
