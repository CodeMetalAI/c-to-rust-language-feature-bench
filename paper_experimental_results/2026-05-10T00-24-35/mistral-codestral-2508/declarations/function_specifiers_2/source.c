// function_specifiers_2.c
_Noreturn void stop(void) {
  volatile int x = 0;
  while (1) {
    x += 1;
  }
}

_Noreturn void f(void) { stop(); }

_Noreturn void g(int i) {
  if (i > 0)
    stop();
}

int main(void) {
  if (1) {
    f();
  }

  g(1);

  return 0;
}
