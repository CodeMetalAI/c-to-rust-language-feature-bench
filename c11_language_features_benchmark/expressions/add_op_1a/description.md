**Test name**: add_op_1a

**Description:**
Validates C11 §6.5.6 pointer arithmetic for pointers to variable length array (VLA) types. The test
declares a VLA `int a[n][m]`, initializes a pointer `p` of type `int (*)[m]` to the first row, advances
it by one row using `p += 1`, writes through `(*p)[2]`, and checks that this updates `a[1][2]`. It also
checks that subtracting `p - a` yields the correct row offset.
