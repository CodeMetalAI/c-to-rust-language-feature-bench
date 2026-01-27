**Test name**: add_op_1b

**Description:**
Validates C11 §6.5.6 (Additive operators) Example 1 for arrays of known constant size. The test declares
a fixed-size two-dimensional array `int a[4][3]` and a pointer `p` of type `int (*)[3]` initialized to
`a`. It checks that `p += 1` advances the pointer by one row, that writing through `(*p)[2]` updates
`a[1][2]`, and that subtracting `p - a` yields the correct row offset, matching the behavior of the
variable-length array case.
