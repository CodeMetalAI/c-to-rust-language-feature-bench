// ptr_decl.c
typedef int *int_ptr;

typedef struct {
  const int *p_to_const;
  int *const const_p;
  const int_ptr const_p_via_typedef;
} bundle;

static int pick(const int *p, int bias) {
  int v = *p;
  if (v & 1)
    return v + bias;
  return v - bias;
}

static void retarget_ptr_to_const(bundle *b, const int *a, const int *c,
                                  int sel) {
  if (sel)
    b->p_to_const = a;
  else
    b->p_to_const = c;
}

static int same_addr(const void *x, const void *y) { return x == y; }

static int check_bundle(bundle b, int *expected_const_p_target) {
  if (!same_addr((const void *)b.const_p,
                 (const void *)expected_const_p_target))
    return 100;
  if (!same_addr((const void *)b.const_p_via_typedef,
                 (const void *)expected_const_p_target))
    return 101;
  return 0;
}

int main(void) {
  int x = 10;
  int y = 21;

  const int *ptr_to_constant = &x;

  int *const constant_ptr = &x;
  const int_ptr clarified_constant_ptr = &x;

  bundle b = {ptr_to_constant, constant_ptr, clarified_constant_ptr};

  if (check_bundle(b, &x) != 0)
    return 1;

  if (*b.p_to_const != 10)
    return 2;

  retarget_ptr_to_const(&b, &y, &x, pick(&y, 0) > 0);
  if (*b.p_to_const != 21)
    return 3;

  retarget_ptr_to_const(&b, &x, &y, pick(&x, 1) < 0);
  if (*b.p_to_const != 21)
    return 4;

  *b.const_p += 5;
  if (x != 15)
    return 5;

  *b.const_p_via_typedef += 7;
  if (x != 22)
    return 6;

  if (check_bundle(b, &x) != 0)
    return 7;

  if (*b.p_to_const != 21)
    return 8;

  return 0;
}
