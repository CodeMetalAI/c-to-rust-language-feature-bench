/* pointers_c8.c */
int f_int(int x) { return x + 1; }
double f_double(double x) { return x + 0.5; }

int main(void) {
  int (*p_int)(int) = f_int;
  double (*p_double)(double) = f_double;

  void *v1 = (void *)p_int;
  void *v2 = (void *)p_double;

  double (*p_double2)(double) = (double (*)(double))v1;
  int (*p_int2)(int) = (int (*)(int))v2;

  void *v1b = (void *)p_double2;
  void *v2b = (void *)p_int2;

  if (v1b != v1)
    return 1;
  if (v2b != v2)
    return 2;

  int (*p_int3)(int) = (int (*)(int))v1b;
  double (*p_double3)(double) = (double (*)(double))v2b;

  if ((void *)p_int3 != (void *)p_int)
    return 3;
  if ((void *)p_double3 != (void *)p_double)
    return 4;

  return 0;
}
