**Test name**: rel_op_c6a

**Description:**
Validates the interpretation of chained relational expressions described in C11 §6.5.8,
Footnote 107. The test demonstrates that `a<b<c` is parsed as `(a<b)<c`, not as a
mathematical chained comparison. It checks both cases where `a<b` is true and false and
verifies that the resulting integer value (0 or 1) is compared against `c`.
