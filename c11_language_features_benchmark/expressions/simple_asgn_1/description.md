**Test name**: simple_asgn_1

**Description:**
Validates C11 §6.5.16.1 (Simple assignment) Example 1. The test executes the pattern
`(c = f()) == -1` with `f()` returning `-1` and checks that the comparison is performed using
the value after conversion to `char` (and subsequent integer promotion). It verifies the
required behavior for both signed and unsigned plain `char`: the comparison is true when
`char` is signed and false when `char` is unsigned.
