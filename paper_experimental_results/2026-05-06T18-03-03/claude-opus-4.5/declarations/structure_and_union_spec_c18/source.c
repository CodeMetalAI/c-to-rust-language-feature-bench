// structure_and_union_spec_c18.c
typedef unsigned char u8;
typedef unsigned int u32;

#define OFFSETOF(type, member) ((unsigned long)&(((type *)0)->member))

struct Packet {
  u32 tag;
  u32 n;
  u32 sum;
  u32 data[];
};

static u32 compute_sum(u32 n) {
  u32 s = 0;
  u32 i = 0;
  while (i < n) {
    s += (i + 1u) * 3u + 1u;
    i += 1;
  }
  return s;
}

static int test_nonempty_object(void) {
  enum { N = 7 };

  union {
    u8 raw[sizeof(struct Packet) + N * sizeof(u32) + 32u];
    u32 align;
  } storage;

  struct Packet *p = (struct Packet *)(void *)storage.raw;

  p->tag = 0xA1B2C3D4u;
  p->n = N;

  {
    unsigned long off = OFFSETOF(struct Packet, data);
    u8 *expected = storage.raw + off;
    u8 *got1 = (u8 *)(void *)&p->data;
    u8 *got2 = (u8 *)(void *)&(*p).data;

    if (got1 != expected)
      return 1;
    if (got2 != expected)
      return 2;
  }

  {
    u32 i = 0;
    while (i < p->n) {
      u32 v = (i + 1u) * 3u + 1u;
      p->data[i] = v;
      (*p).data[i] += 0u;
      i += 1;
    }
  }

  p->sum = 0;
  {
    u32 i = 0;
    while (i < p->n) {
      p->sum += p->data[i];
      i += 1;
    }
  }

  if (p->sum != compute_sum(N))
    return 3;

  if ((unsigned long)sizeof(struct Packet) < OFFSETOF(struct Packet, data))
    return 4;

  return 0;
}

static int test_zero_element_object(void) {
  union {
    u8 raw[sizeof(struct Packet)];
    u32 align;
  } storage;

  struct Packet *p = (struct Packet *)(void *)storage.raw;

  p->tag = 0u;
  p->n = 0u;
  p->sum = 0u;

  {
    unsigned long off = OFFSETOF(struct Packet, data);
    u8 *expected = storage.raw + off;

    u8 *got1 = (u8 *)(void *)&p->data;
    u8 *got2 = (u8 *)(void *)&(*p).data;

    if (got1 != expected)
      return 10;
    if (got2 != expected)
      return 11;
  }

  return 0;
}

int main(void) {
  int r;

  r = test_nonempty_object();
  if (r)
    return r;

  r = test_zero_element_object();
  if (r)
    return r;

  return 0;
}
