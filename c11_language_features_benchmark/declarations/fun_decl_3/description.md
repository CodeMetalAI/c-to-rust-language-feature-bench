**Test name**: fun_decl_3

**Description:**
Section 6.7.6.3 (Function declarators), Example 3 coverage.
Implements and exercises `int (*fpfi(int (*)(long), int))(int, ...);`: `fpfi` accepts a function-pointer parameter of type `int (*)(long)` plus an `int`, and returns a pointer to a variadic function of type `int (*)(int, ...)`. The test stresses translation by selecting among returned variadic function pointers at runtime, storing/calling through the returned pointer, and invoking it with extra arguments of different types while verifying the returned results depend only on the required fixed `int` parameter.
