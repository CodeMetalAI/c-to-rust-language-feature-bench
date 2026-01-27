**Test name**: fun_decl_1

**Description:**
Section 6.7.6.3 (Function declarators), Example 1 coverage.
Exercises declarator binding differences among `int f(void)`, `int *fip()`, and `int (*pfi)()`: calling `f` returns an `int`, calling `fip` returns `int *` that is then dereferenced via the `*fip()` binding, and `pfi` is a pointer to a function with no parameter specification that must be dereferenced to a function designator and then called. The test stresses translation by selecting and invoking function pointers through multiple indirections and by validating the distinct semantics of `*fip()` versus `(*pfi)()`.
