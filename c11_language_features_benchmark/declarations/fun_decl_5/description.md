**Test name**: fun_decl_5

**Description:**
Section 6.7.6.3 (Function declarators), Example 5 coverage.
Exercises compatibility of variably modified array parameters by defining `maximum(int n, int m, double a[n][m])` and calling it both directly and through a function-pointer type that treats the third parameter as an unspecified VLA pointer (`double (*)[*]`), relying on the prototype-compatibility rules for `a[n][m]`, `a[*][*]`, `a[][ *]`, and `a[][m]`. Also exercises compatibility of `void f(double (* restrict a)[5])` with array-parameter forms by calling `f` on a 2D array and validating correct indexed updates, including the requirement that the argument designate at least three rows of 5 doubles for the exercised call.
