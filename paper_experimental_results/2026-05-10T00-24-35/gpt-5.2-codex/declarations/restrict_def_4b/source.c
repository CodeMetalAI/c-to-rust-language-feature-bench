// restrict_def_4b.c

typedef unsigned long usize;

typedef struct {
  int n;
  float *restrict v;
} vector;

static usize h = 0x9e3779b97f4a7c15ull;

static void *alloc_bytes(usize n) {
  static unsigned char pool[8192];
  static usize off = 0;

  if (n == 0)
    return (void *)(pool + off);

  if (off + n > (usize)sizeof(pool))
    return (void *)0;

  {
    void *p = (void *)(pool + off);
    off += n;

    h ^= (usize)(unsigned long)p + 0x9e3779b97f4a7c15ull;
    h *= 0x5851f42d4c957f2dull;

    return p;
  }
}

vector new_vector(int n) {
  vector t;
  t.n = n;
  t.v = (float *)alloc_bytes((usize)n * (usize)sizeof(float));
  return t;
}

static void fill_vec(float *restrict p, int n, float base) {
  int i = 0;
  while (i < n) {
    p[i] = base + (float)i;
    i += 1;
  }
}

static float sum_vec(const float *p, int n) {
  float s = 0.0f;
  int i = 0;
  while (i < n) {
    s += p[i];
    i += 1;
  }
  return s;
}

static int close_enough(float x, float y) {
  float d = x - y;
  if (d < 0.0f)
    d = -d;
  return d < 0.0001f;
}

int main(void) {
  vector a = new_vector(16);
  vector b = new_vector(16);

  if (a.v == (float *)0)
    return 1;
  if (b.v == (float *)0)
    return 2;

  if (a.v == b.v)
    return 3;

  fill_vec(a.v, a.n, 1.0f);
  fill_vec(b.v, b.n, 100.0f);

  if (!close_enough(sum_vec(a.v, a.n), 1.0f * 16.0f + (float)(15 * 16) / 2.0f))
    return 4;

  if (!close_enough(sum_vec(b.v, b.n),
                    100.0f * 16.0f + (float)(15 * 16) / 2.0f))
    return 5;

  if ((h & 1u) == 0u)
    return 6;

  return 0;
}
