// function_specifiers_c8a.c
_Noreturn void _Exit(int);

_Noreturn int die0(void) { _Exit(0); }

int pick(int x) { return x ? 123 : die0(); }

int main(void) {
  int v = pick(0);
  (void)v;
  return 1;
}
