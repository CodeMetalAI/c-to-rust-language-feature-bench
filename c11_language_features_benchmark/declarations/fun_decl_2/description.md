**Test name**: fun_decl_2

**Description:**
Section 6.7.6.3 (Function declarators), Example 2 coverage.
Declares `apfi` as an array of three pointers to functions returning `int`, where each function has two parameters of type `int *` (with parameter names `x` and `y` that are descriptive only and do not escape the declarator). The test stresses translation by selecting a function pointer from the array at runtime, calling it through both direct and explicitly dereferenced forms, and validating that pointer arguments are passed and mutated correctly via the chosen function.
