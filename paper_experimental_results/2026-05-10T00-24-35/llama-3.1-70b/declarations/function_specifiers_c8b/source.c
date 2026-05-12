// function_specifiers_c8b.c

_Noreturn void _Exit(int);

int g;

_Noreturn void die_if(int x) { _Exit((x == 7 && g == 9) ? 0 : 2); }

int main(void) {
  void (*p)(int) = die_if;
  g = 9;
  p(7);
  return 3;
}
