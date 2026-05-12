// type_def_4.c
typedef void fv(int), (*pfv)(int);

static int hits;

static void handler_a(int x) { hits += x + 1; }

static void handler_b(int x) { hits += x + 2; }

void (*signal(int, void (*)(int)))(int);

fv *signal(int s, fv *h) {
  if (s & 1)
    return handler_a;
  return h;
}

pfv signal_alias(int s, pfv h) { return signal(s, h); }

static int call_signal_as_plain(int s, void (*h)(int), int arg) {
  void (*r)(int) = signal(s, h);
  r(arg);
  return hits;
}

static int call_signal_as_fv(int s, fv *h, int arg) {
  fv *r = signal(s, h);
  r(arg);
  return hits;
}

static int call_signal_as_pfv(int s, pfv h, int arg) {
  pfv r = signal_alias(s, h);
  r(arg);
  return hits;
}

int main(void) {
  hits = 0;

  if (call_signal_as_plain(1, handler_b, 3) != (3 + 1))
    return 1;

  hits = 0;
  if (call_signal_as_fv(2, handler_b, 4) != (4 + 2))
    return 2;

  hits = 0;
  if (call_signal_as_pfv(3, handler_b, 5) != (5 + 1))
    return 3;

  return 0;
}
