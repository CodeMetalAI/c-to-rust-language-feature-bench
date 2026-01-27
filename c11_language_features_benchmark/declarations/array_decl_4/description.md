**Test name**: array_decl_4

**Description:**
Section 6.7.6.2 (Array declarators), Example 4 (variably modified types) coverage.
Exercises that VLA/VM declarations occur only at block scope or prototype scope by using a prototype-scope VLA parameter `C[m][m]`, a block-scope VLA object `D[m]`, a block-scope typedef `VLA[m][m]`, and block-scope pointers to VLA (`s` and a `static` VM pointer `q`). The test stresses translation by mixing VLA parameters, VLA locals, and a static-duration pointer whose VM type depends on `m`, then validating correct indexed access through these VLA-derived types across multiple calls.
