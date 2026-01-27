**Test name**: fun_defs_2

**Description:**
Example 2 from Section 6.9.1: passing a function to another function and
equivalence between `void g(int (*funcp)(void))` and `void g(int func(void))`.

- A function name `f` is passed to another function as an argument `(g(f))`.
- A parameter declared as `int (*)(void)` and one declared as `int func(void)` behave equivalently
  (the latter adjusts to a pointer-to-function type).
- Calling via `(*funcp)()` and `func()` both work.
