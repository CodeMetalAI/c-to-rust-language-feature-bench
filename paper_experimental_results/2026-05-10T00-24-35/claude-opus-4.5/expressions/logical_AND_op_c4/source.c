/*
 * logical_AND_op_c4
 */

static int g = 0;

static int set_g_return(int gval, int ret) {
  g = gval;
  return ret;
}

int main(void) {
  int y = 0;

  g = 0;
  if (set_g_return(0, 0) && (++y))
    return 1;
  if (y != 0)
    return 2;

  g = 0;
  if (!(set_g_return(1, 1) && (g == 1)))
    return 3;

  return 0;
}
