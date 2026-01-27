**Test name**: fun_decl_4

**Description:**
Section 6.7.6.3 (Function declarators), Example 4 coverage.
Exercises a prototype with a variably modified array parameter `double a[n][n*m+300]` by passing a concrete `double b[4][308]` where `308 == 4*2+300`. The callee computes the dependent bound (`k = n*m+300`) and updates every element via `a[i][j] += x`, validating that the parameter is treated as a pointer to a VLA whose second dimension depends on runtime values `n` and `m`.
