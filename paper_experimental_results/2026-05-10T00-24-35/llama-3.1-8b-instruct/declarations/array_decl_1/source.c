// array_decl_1.c
typedef unsigned long usize;

static volatile int calls;

static int pos(int x) {
  calls += 1;
  if (x <= 0)
    return 1;
  return x;
}

float fa[11];
float *afp[17];

static float backing[17];

static void init_globals(void) {
  int i = 0;
  while (i < 11) {
    fa[i] = (float)(i + 1);
    i += 1;
  }
  i = 0;
  while (i < 17) {
    backing[i] = (float)(100 + i);
    afp[i] = &backing[i];
    i += 1;
  }
}

static int sum_ints_from_float(const float *p, int n) {
  int s = 0;
  int i = 0;
  while (i < n) {
    s += (int)p[i];
    i += 1;
  }
  return s;
}

static int sum_pointed_ints(float *const *pp, int n) {
  int s = 0;
  int i = 0;
  while (i < n) {
    s += (int)(*pp[i]);
    i += 1;
  }
  return s;
}

static int takes_params(float a[static const 11],
                        float *afp2[static const 17]) {
  int s1 = (int)a[0] + (int)a[10];
  int s2 = (int)(*afp2[0]) + (int)(*afp2[16]);
  return s1 + s2;
}

int main(void) {
  init_globals();

  if (fa[0] != 1.0f || fa[10] != 11.0f)
    return 1;

  if ((int)(*afp[0]) != 100 || (int)(*afp[16]) != 116)
    return 2;

  {
    int n1 = pos(11);
    int n2 = pos(17);

    float vla1[n1];
    float *vla2[n2];

    int i = 0;
    while (i < n1) {
      vla1[i] = fa[i] * 2.0f;
      i += 1;
    }

    i = 0;
    while (i < n2) {
      vla2[i] = afp[i];
      i += 1;
    }

    if (calls != 2)
      return 3;

    {
      usize span =
          (usize)((char *)(void *)&vla1[n1 - 1] - (char *)(void *)&vla1[0]);
      if (span != (usize)(n1 - 1) * (usize)sizeof(float))
        return 4;
    }

    if (sum_ints_from_float(vla1, 11) !=
        (2 + 4 + 6 + 8 + 10 + 12 + 14 + 16 + 18 + 20 + 22))
      return 5;

    if (sum_pointed_ints(vla2, 17) !=
        (100 + 101 + 102 + 103 + 104 + 105 + 106 + 107 + 108 + 109 + 110 + 111 +
         112 + 113 + 114 + 115 + 116))
      return 6;

    if (takes_params(vla1, vla2) !=
        ((int)vla1[0] + (int)vla1[10] + (int)(*vla2[0]) + (int)(*vla2[16])))
      return 7;
  }

  return 0;
}
