/* storage_duration_c6.c */

typedef unsigned long uptr;

int distinct_instances(int depth, uptr prev_addr) {
  int local; /* no initializer */

  uptr addr = (uptr)(void *)&local;

  if (prev_addr != 0 && addr == prev_addr)
    return 1;

  if (depth == 0)
    return 0;

  return distinct_instances(depth - 1, addr);
}

int main(void) {
  if (distinct_instances(8, 0) != 0)
    return 1;
  return 0;
}
