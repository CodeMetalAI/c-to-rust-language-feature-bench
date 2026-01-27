**Test name**: eq_op_c3

**Description:**
Validates C11 §6.5.9, Footnote 108. The test demonstrates that operator precedence parses
`a<b == c<d` as `(a<b) == (c<d)`, so the expression evaluates to 1 exactly when the two
relational comparisons have the same truth-value, and evaluates to 0 when they differ.
